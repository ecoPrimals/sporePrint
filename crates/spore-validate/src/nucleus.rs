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
use serde_json::Value;
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
    /// Deserialized for schema completeness; consumed by future role-aware routing.
    #[serde(default)]
    #[allow(dead_code)]
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
pub struct LaunchConfig {
    /// Ordered list of primal slugs for startup sequencing.
    #[serde(default)]
    pub order: Vec<String>,
    /// Primal after which remaining launches can proceed in parallel.
    /// Deserialized for schema completeness; consumed by future parallel-launch validation.
    #[serde(default)]
    #[allow(dead_code)]
    pub parallel_after: Option<String>,
}

/// Mesh/federation configuration.
#[derive(Debug, Deserialize)]
pub struct MeshConfig {
    /// Node identity for this gate in the mesh.
    /// Deserialized for schema completeness; consumed by future mesh-identity validation.
    #[serde(default)]
    #[allow(dead_code)]
    pub node_id: Option<String>,
    /// Whether federation is enabled for this profile.
    #[serde(default)]
    pub federation_enabled: Option<bool>,
    /// Bootstrap peer addresses.
    /// Deserialized for schema completeness; consumed by future peer-reachability probing.
    #[serde(default)]
    #[allow(dead_code)]
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
    pub primal_id: Option<String>,
    pub status: Option<String>,
    /// guideStone health contract compliance: `{status, primal, version}` all present.
    pub health_contract: HealthContract,
    /// Whether the primal accepted a mito-beacon (`0xEC 0x01`) prefixed request.
    /// `None` if riboCipher probing was not performed.
    pub ribocipher_accepted: Option<bool>,
    pub error: Option<String>,
}

/// guideStone health contract compliance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthContract {
    /// All three required fields present: status, primal, version.
    Compliant,
    /// Responds but missing one or more required fields.
    Partial,
    /// Did not respond or returned an error.
    None,
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
/// discovered socket to verify the primal is responsive.
///
/// When `ribocipher` is true (requires `probe`), also tests mito-beacon signal
/// acceptance by connecting a second time with `0xEC 0x01` prefix before the
/// JSON-RPC payload. This diagnoses the genetics-layer wiring issue (Wave 114).
pub fn validate_profile(
    profile: &NucleusProfile,
    probe: bool,
    ribocipher: bool,
) -> ValidationResult {
    let mut healthy = Vec::new();
    let mut missing = Vec::new();

    for (name, entry) in &profile.primals {
        let env_var = format!("{}_SOCKET", name.to_uppercase());
        let socket = discovery::probe_socket(name, &env_var);

        let mut probe_result = if probe {
            socket.as_ref().map(|path| probe_socket_health(path))
        } else {
            None
        };

        if ribocipher && probe {
            if let Some(ref path) = socket {
                let accepted = probe_ribocipher_acceptance(path);
                if let Some(ref mut pr) = probe_result {
                    pr.ribocipher_accepted = Some(accepted);
                }
            }
        }

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

    let critical_names: Vec<&str> = profile.health.as_ref().map_or(Vec::new(), |h| {
        h.critical.iter().map(String::as_str).collect()
    });

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

/// Construct a failed probe result.
fn probe_failed(start: Instant, error: String) -> ProbeResult {
    ProbeResult {
        responsive: false,
        latency: start.elapsed(),
        version: None,
        primal_id: None,
        status: None,
        health_contract: HealthContract::None,
        ribocipher_accepted: None,
        error: Some(error),
    }
}

/// Probe a primal via `health.liveness` (v2.1+) with `health.ping` fallback.
///
/// Uses the shared `ipc::probe_health` for the method negotiation, but
/// wraps the UDS connection setup and timeout logic locally since NUCLEUS
/// probing uses shorter timeouts than CAS transport.
fn probe_socket_health(socket_path: &str) -> ProbeResult {
    let start = Instant::now();

    let stream = match std::os::unix::net::UnixStream::connect(socket_path) {
        Ok(s) => s,
        Err(e) => return probe_failed(start, format!("connect: {e}")),
    };

    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok();
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok();

    let mut reader = BufReader::new(Box::new(stream) as Box<dyn crate::cas_push::ReadWrite>);

    let resp = match crate::ipc::probe_health(&mut reader, 1) {
        Ok(r) => r,
        Err(e) => return probe_failed(start, format!("{e}")),
    };

    let latency = start.elapsed();

    if let Some(err_msg) = crate::ipc::extract_error_message(&resp) {
        return ProbeResult {
            responsive: true,
            latency,
            version: None,
            primal_id: None,
            status: None,
            health_contract: HealthContract::None,
            ribocipher_accepted: None,
            error: Some(err_msg),
        };
    }

    let result_obj = resp.get("result");

    let version = result_obj
        .and_then(|r| r.get("version"))
        .and_then(Value::as_str)
        .map(String::from);

    let primal_id = result_obj
        .and_then(|r| r.get("primal"))
        .and_then(Value::as_str)
        .map(String::from);

    let status = result_obj
        .and_then(|r| r.get("status"))
        .and_then(Value::as_str)
        .map(String::from);

    let health_contract = match (&version, &primal_id, &status) {
        (Some(_), Some(_), Some(_)) => HealthContract::Compliant,
        (None, None, None) => HealthContract::None,
        _ => HealthContract::Partial,
    };

    ProbeResult {
        responsive: true,
        latency,
        version,
        primal_id,
        status,
        health_contract,
        ribocipher_accepted: None,
        error: None,
    }
}

/// `riboCipher` Transport Signal: `MitoBeacon` clear (`0xEC 0x01`).
const RIBOCIPHER_MITO_CLEAR: [u8; 2] = [0xEC, 0x01];

/// Probe whether a primal accepts the `riboCipher` mito-beacon signal prefix.
///
/// Connects to the socket, writes `0xEC 0x01` followed by a `health.liveness`
/// JSON-RPC request, and checks whether the primal responds with valid JSON-RPC
/// rather than closing the connection or returning garbage.
fn probe_ribocipher_acceptance(socket_path: &str) -> bool {
    let Ok(stream) = std::os::unix::net::UnixStream::connect(socket_path) else {
        return false;
    };

    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok();
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok();

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health.liveness",
        "params": {},
        "id": 2
    });

    let Ok(json_payload) = serde_json::to_string(&request) else {
        return false;
    };

    let mut payload = Vec::with_capacity(2 + json_payload.len() + 1);
    payload.extend_from_slice(&RIBOCIPHER_MITO_CLEAR);
    payload.extend_from_slice(json_payload.as_bytes());
    payload.push(b'\n');

    let mut reader = BufReader::new(stream);

    if reader.get_mut().write_all(&payload).is_err() {
        return false;
    }
    if reader.get_mut().flush().is_err() {
        return false;
    }

    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return false;
    }

    // Valid acceptance: primal stripped the prefix and returned JSON-RPC
    serde_json::from_str::<Value>(line.trim()).is_ok() && !line.trim().is_empty()
}

// ── Display ──────────────────────────────────────────────────────────

/// Print NUCLEUS validation results to stdout.
///
/// Displays profile metadata, per-primal health status (with optional probe
/// details), and aggregate compliance summary.
pub fn print_result(profile: &NucleusProfile, result: &ValidationResult, profile_path: &Path) {
    print_header(profile, result, profile_path);
    print_primals(result);
    print_summary(result);
}

fn print_header(profile: &NucleusProfile, result: &ValidationResult, profile_path: &Path) {
    println!("sporePrint: NUCLEUS profile validation");
    println!(
        "  Profile: {} ({})",
        result.profile_name,
        profile_path.display()
    );
    if let Some(desc) = &profile.profile.description {
        println!("  Description: {desc}");
    }
    if let Some(base) = profile.profile.base() {
        println!("  Extends: {base}");
    }
    println!("  Declared primals: {}", result.total_declared);
    if !profile.launch_order().is_empty() {
        println!("  Launch order: {}", profile.launch_order().join(" → "));
    }
    if profile.federation_enabled() {
        println!("  Federation: enabled");
    }
    println!();
}

fn print_primals(result: &ValidationResult) {
    if !result.healthy.is_empty() {
        println!(
            "  HEALTHY ({}/{}):",
            result.healthy.len(),
            result.total_declared
        );
        for p in &result.healthy {
            let probe_info = format_probe_info(p.probe.as_ref());
            println!(
                "    ✅ {} [{}] → {}{}",
                p.name,
                p.role,
                p.socket_path.as_deref().unwrap_or("?"),
                probe_info
            );
        }
    }

    if !result.missing.is_empty() {
        println!();
        println!(
            "  MISSING ({}/{}):",
            result.missing.len(),
            result.total_declared
        );
        for p in &result.missing {
            let marker = if p.required { "❌" } else { "⚠️" };
            let probe_err = format_probe_error(p.probe.as_ref());
            println!(
                "    {marker} {} [{}] (required={}){probe_err}",
                p.name, p.role, p.required
            );
        }
    }

    if result.healthy.iter().any(|p| p.probe.is_some()) {
        let total_probed = result.healthy.len();
        let compliant = count_by_contract(&result.healthy, HealthContract::Compliant);
        let partial = count_by_contract(&result.healthy, HealthContract::Partial);

        println!();
        println!(
            "  Health contract (guideStone): {compliant}/{total_probed} compliant, {partial} partial"
        );

        let all_primals: Vec<_> = result.healthy.iter().chain(result.missing.iter()).collect();
        let ribo_total = all_primals.iter().filter(|p| has_ribo_result(p)).count();

        if ribo_total > 0 {
            let accepted = all_primals.iter().filter(|p| ribo_accepted(p)).count();
            println!("  riboCipher mito-beacon: {accepted}/{ribo_total} accept signal");
        }
    }

    println!();
}

fn print_summary(result: &ValidationResult) {
    println!(
        "  Critical path: {}",
        if result.critical_met {
            "✅ MET"
        } else {
            "❌ FAILED"
        }
    );
    println!(
        "  Min healthy: {}",
        if result.min_healthy_met {
            "✅ MET"
        } else {
            "❌ FAILED"
        }
    );
    println!();
}

fn count_by_contract(primals: &[PrimalStatus], target: HealthContract) -> usize {
    primals
        .iter()
        .filter(|p| {
            p.probe
                .as_ref()
                .is_some_and(|pr| pr.health_contract == target)
        })
        .count()
}

fn has_ribo_result(p: &PrimalStatus) -> bool {
    p.probe
        .as_ref()
        .is_some_and(|pr| pr.ribocipher_accepted.is_some())
}

fn ribo_accepted(p: &PrimalStatus) -> bool {
    p.probe
        .as_ref()
        .is_some_and(|pr| pr.ribocipher_accepted == Some(true))
}

fn format_probe_info(probe: Option<&ProbeResult>) -> String {
    probe.map_or_else(String::new, |pr| {
        let contract_icon = match pr.health_contract {
            HealthContract::Compliant => " [health:✅]",
            HealthContract::Partial => " [health:⚠️]",
            HealthContract::None => "",
        };
        let ribo_icon = match pr.ribocipher_accepted {
            Some(true) => " [mito:✅]",
            Some(false) => " [mito:❌]",
            None => "",
        };
        let version_str = pr
            .version
            .as_deref()
            .map_or(String::new(), |v| format!(", v{v}"));
        let identity_str = pr
            .primal_id
            .as_deref()
            .map_or(String::new(), |id| format!(", id={id}"));
        let status_str = pr
            .status
            .as_deref()
            .map_or(String::new(), |s| format!(", {s}"));
        format!(
            " ({}ms{version_str}{identity_str}{status_str}{contract_icon}{ribo_icon})",
            pr.latency.as_millis()
        )
    })
}

fn format_probe_error(probe: Option<&ProbeResult>) -> String {
    probe.map_or_else(String::new, |pr| {
        let mut info = String::new();
        if let Some(e) = &pr.error {
            info.push_str(" — ");
            info.push_str(e);
        }
        if pr.responsive && pr.health_contract == HealthContract::None {
            info.push_str(" [no health method]");
        }
        info
    })
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
        let result = validate_profile(&profile, false, false);
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

        let result = validate_profile(&profile, false, false);
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
