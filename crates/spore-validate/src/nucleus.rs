// SPDX-License-Identifier: AGPL-3.0-or-later

//! Proto-nucleate manifest — sub-NUCLEUS topology validation.
//!
//! Parses deployment profiles (e.g., `irongate-full.toml`, `flockgate-wan.toml`)
//! and validates that the running NUCLEUS matches the declared topology.
//!
//! ## Profiles
//!
//! A profile declares which primals are required, their roles, launch ordering,
//! health thresholds, and mesh configuration. sporePrint probes each declared
//! primal's socket and reports compliance.
//!
//! ## Topology Classes
//!
//! - **Full** (13/13): All NUCLEUS primals — production desktop gates
//! - **Tower** (3/3): beardog + songbird + toadstool — minimal compute spine
//! - **Nest** (7/7): Storage specialization — nestgate-centric
//! - **Fieldmouse** (13/13 canary): Full stack, previous-good binaries
//! - **Relay** (2/2): songbird + beardog — WAN relay only

use crate::discovery;
use crate::error::Error;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};

/// A NUCLEUS deployment profile parsed from TOML.
#[derive(Debug, Deserialize)]
pub struct NucleusProfile {
    pub profile: ProfileMeta,
    #[serde(default)]
    pub primals: BTreeMap<String, PrimalEntry>,
    #[serde(default)]
    pub health: Option<HealthConfig>,
    #[serde(default)]
    pub launch: Option<LaunchConfig>,
    #[serde(default)]
    pub mesh: Option<MeshConfig>,
}

impl NucleusProfile {
    /// Whether federation is configured in this profile.
    pub fn federation_enabled(&self) -> bool {
        self.mesh
            .as_ref()
            .and_then(|m| m.federation_enabled)
            .unwrap_or(false)
    }

    /// The declared launch order for primals.
    pub fn launch_order(&self) -> &[String] {
        self.launch.as_ref().map_or(&[], |l| &l.order)
    }
}

/// Profile metadata.
#[derive(Debug, Deserialize)]
pub struct ProfileMeta {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Base profile this extends (e.g., "full", "tower").
    #[serde(default)]
    pub extends: Option<String>,
    /// Deployment role (e.g., "canary", "production").
    #[serde(default)]
    pub role: Option<String>,
}

impl ProfileMeta {
    /// The base profile this extends, if any.
    pub fn base(&self) -> Option<&str> {
        self.extends.as_deref()
    }
}

/// A single primal entry in the profile.
#[derive(Debug, Deserialize)]
pub struct PrimalEntry {
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default)]
    pub role: Option<String>,
}

const fn default_true() -> bool {
    true
}

/// Health threshold configuration.
#[derive(Debug, Deserialize)]
pub struct HealthConfig {
    #[serde(default)]
    pub min_healthy: Option<usize>,
    #[serde(default)]
    pub critical: Vec<String>,
}

/// Launch ordering configuration.
#[derive(Debug, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct LaunchConfig {
    /// Ordered list of primal slugs for startup sequencing.
    #[serde(default)]
    pub order: Vec<String>,
    /// Primal after which remaining launches can proceed in parallel.
    #[serde(default)]
    pub parallel_after: Option<String>,
}

/// Mesh/federation configuration.
#[derive(Debug, Deserialize)]
pub struct MeshConfig {
    /// Node identity for this gate in the mesh.
    #[serde(default)]
    pub node_id: Option<String>,
    /// Whether federation is enabled for this profile.
    #[serde(default)]
    pub federation_enabled: Option<bool>,
    /// Bootstrap peer addresses.
    #[serde(default)]
    pub peers: Vec<String>,
}

/// Result of validating a running NUCLEUS against a profile.
#[derive(Debug)]
pub struct ValidationResult {
    pub profile_name: String,
    pub total_declared: usize,
    pub healthy: Vec<PrimalStatus>,
    pub missing: Vec<PrimalStatus>,
    pub critical_met: bool,
    pub min_healthy_met: bool,
}

/// Status of an individual primal.
#[derive(Debug)]
pub struct PrimalStatus {
    pub name: String,
    pub role: String,
    pub required: bool,
    pub socket_path: Option<String>,
    /// IPC probe result (populated when `--probe` is used).
    pub probe: Option<ProbeResult>,
}

/// Result of sending a `health.ping` JSON-RPC call to a primal socket.
#[derive(Debug)]
pub struct ProbeResult {
    pub responsive: bool,
    pub latency: Duration,
    pub version: Option<String>,
    pub error: Option<String>,
}

impl ValidationResult {
    /// Overall pass: all critical primals healthy AND `min_healthy` threshold met.
    pub const fn passed(&self) -> bool {
        self.critical_met && self.min_healthy_met
    }
}

/// Parse a NUCLEUS profile from a TOML file.
pub fn parse_profile(path: &Path) -> Result<NucleusProfile, Error> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| Error::Config(format!("read profile {}: {e}", path.display())))?;
    let profile: NucleusProfile = toml::from_str(&text)
        .map_err(|e| Error::Config(format!("parse profile {}: {e}", path.display())))?;
    Ok(profile)
}

/// Validate the running NUCLEUS against a parsed profile.
///
/// Probes each declared primal's socket using the standard discovery chain:
/// 1. `{PRIMAL}_SOCKET` env var
/// 2. `BIOMEOS_SOCKET_DIR/{slug}.sock`
/// 3. `/run/membrane/{slug}.sock`
/// 4. `XDG_RUNTIME_DIR/biomeos/{slug}.sock`
///
/// When `probe` is true, additionally sends a `health.ping` JSON-RPC call to each
/// discovered socket to verify the primal is responsive (not just that the socket exists).
pub fn validate_profile(profile: &NucleusProfile, probe: bool) -> ValidationResult {
    let mut healthy = Vec::new();
    let mut missing = Vec::new();

    for (name, entry) in &profile.primals {
        let env_var = format!("{}_SOCKET", name.to_uppercase());
        let socket = discovery::probe_socket(name, &env_var);

        let probe_result = if probe {
            socket.as_ref().map(|path| probe_socket_health(path))
        } else {
            None
        };

        let is_healthy = match (&socket, &probe_result) {
            (Some(_), Some(result)) => result.responsive,
            (Some(_), None) => true,
            _ => false,
        };

        let status = PrimalStatus {
            name: name.clone(),
            role: entry.role.as_deref().unwrap_or("unknown").to_string(),
            required: entry.required,
            socket_path: socket,
            probe: probe_result,
        };

        if is_healthy {
            healthy.push(status);
        } else {
            missing.push(status);
        }
    }

    let critical_names: Vec<&str> = profile
        .health
        .as_ref()
        .map_or(Vec::new(), |h| h.critical.iter().map(String::as_str).collect());

    let critical_met = critical_names
        .iter()
        .all(|name| healthy.iter().any(|s| s.name == *name));

    let min_healthy_threshold = profile
        .health
        .as_ref()
        .and_then(|h| h.min_healthy)
        .unwrap_or(0);

    let min_healthy_met = healthy.len() >= min_healthy_threshold;

    ValidationResult {
        profile_name: profile.profile.name.clone(),
        total_declared: profile.primals.len(),
        healthy,
        missing,
        critical_met,
        min_healthy_met,
    }
}

/// IPC timeout for health probes (fast — just a ping).
const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// Send `health.ping` JSON-RPC over UDS to verify a primal is responsive.
fn probe_socket_health(socket_path: &str) -> ProbeResult {
    let start = Instant::now();

    let stream = match std::os::unix::net::UnixStream::connect(socket_path) {
        Ok(s) => s,
        Err(e) => {
            return ProbeResult {
                responsive: false,
                latency: start.elapsed(),
                version: None,
                error: Some(format!("connect: {e}")),
            };
        }
    };

    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok();
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok();

    let request = json!({
        "jsonrpc": "2.0",
        "method": "health.ping",
        "params": {},
        "id": 1
    });

    let mut payload = match serde_json::to_string(&request) {
        Ok(s) => s,
        Err(e) => {
            return ProbeResult {
                responsive: false,
                latency: start.elapsed(),
                version: None,
                error: Some(format!("encode: {e}")),
            };
        }
    };
    payload.push('\n');

    let mut reader = BufReader::new(stream);

    if let Err(e) = reader.get_mut().write_all(payload.as_bytes()) {
        return ProbeResult {
            responsive: false,
            latency: start.elapsed(),
            version: None,
            error: Some(format!("write: {e}")),
        };
    }
    if let Err(e) = reader.get_mut().flush() {
        return ProbeResult {
            responsive: false,
            latency: start.elapsed(),
            version: None,
            error: Some(format!("flush: {e}")),
        };
    }

    let mut line = String::new();
    if let Err(e) = reader.read_line(&mut line) {
        return ProbeResult {
            responsive: false,
            latency: start.elapsed(),
            version: None,
            error: Some(format!("read: {e}")),
        };
    }

    let latency = start.elapsed();

    let version = serde_json::from_str::<Value>(line.trim())
        .ok()
        .and_then(|v| v.get("result")?.get("version")?.as_str().map(String::from));

    ProbeResult {
        responsive: true,
        latency,
        version,
        error: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_profile() {
        let toml_str = r#"
[profile]
name = "test-full"
description = "Test full NUCLEUS"
extends = "full"

[primals]
beardog = { required = true, role = "crypto-spine" }
songbird = { required = true, role = "discovery-mesh" }
biomeos = { required = true, role = "orchestration" }

[health]
min_healthy = 2
critical = ["beardog", "biomeos"]

[launch]
order = ["beardog", "songbird", "biomeos"]
parallel_after = "beardog"

[mesh]
node_id = "test-gate"
federation_enabled = true
peers = ["157.230.3.183:7700"]
"#;
        let profile: NucleusProfile = toml::from_str(toml_str).unwrap();
        assert_eq!(profile.profile.name, "test-full");
        assert_eq!(profile.primals.len(), 3);
        assert!(profile.primals["beardog"].required);
        assert_eq!(
            profile.primals["beardog"].role.as_deref(),
            Some("crypto-spine")
        );
        assert_eq!(profile.health.as_ref().unwrap().min_healthy, Some(2));
        assert_eq!(profile.health.as_ref().unwrap().critical.len(), 2);
        assert_eq!(profile.launch.as_ref().unwrap().order.len(), 3);
        assert!(profile.mesh.as_ref().unwrap().federation_enabled.unwrap());
    }

    #[test]
    fn parse_minimal_profile() {
        let toml_str = r#"
[profile]
name = "tower-relay"
"#;
        let profile: NucleusProfile = toml::from_str(toml_str).unwrap();
        assert_eq!(profile.profile.name, "tower-relay");
        assert!(profile.primals.is_empty());
        assert!(profile.health.is_none());
    }

    #[test]
    fn validate_empty_profile_passes() {
        let profile = NucleusProfile {
            profile: ProfileMeta {
                name: "empty".into(),
                description: None,
                extends: None,
                role: None,
            },
            primals: BTreeMap::new(),
            health: None,
            launch: None,
            mesh: None,
        };
        let result = validate_profile(&profile, false);
        assert!(result.passed());
        assert_eq!(result.total_declared, 0);
    }

    #[test]
    fn validate_missing_primal_fails_critical() {
        let mut primals = BTreeMap::new();
        primals.insert(
            "nonexistent_primal_xyz".into(),
            PrimalEntry {
                required: true,
                role: Some("test".into()),
            },
        );

        let profile = NucleusProfile {
            profile: ProfileMeta {
                name: "test".into(),
                description: None,
                extends: None,
                role: None,
            },
            primals,
            health: Some(HealthConfig {
                min_healthy: Some(1),
                critical: vec!["nonexistent_primal_xyz".into()],
            }),
            launch: None,
            mesh: None,
        };

        let result = validate_profile(&profile, false);
        assert!(!result.passed());
        assert!(!result.critical_met);
        assert!(!result.min_healthy_met);
        assert_eq!(result.missing.len(), 1);
    }

    #[test]
    fn parse_real_irongate_profile() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../infra/plasmidBin/profiles/irongate-full.toml");
        if path.exists() {
            let profile = parse_profile(&path).unwrap();
            assert_eq!(profile.profile.name, "irongate-full");
            assert_eq!(profile.primals.len(), 13);
        }
    }

    #[test]
    fn parse_real_flockgate_profile() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../infra/plasmidBin/profiles/flockgate-wan.toml");
        if path.exists() {
            let profile = parse_profile(&path).unwrap();
            assert_eq!(profile.profile.name, "flockgate-wan");
            assert_eq!(profile.primals.len(), 13);
            assert!(profile.mesh.as_ref().unwrap().federation_enabled.unwrap());
        }
    }
}
