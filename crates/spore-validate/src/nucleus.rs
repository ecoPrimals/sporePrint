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
use crate::nucleus_probe::{probe_ribocipher_acceptance, probe_socket_health};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

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

/// Common deployment roles — used for informational warnings only.
/// Custom roles are valid; this list helps catch typos.
const COMMON_ROLES: &[&str] = &[
    "canary",
    "production",
    "development",
    "relay",
    "compute",
    "storage",
    "gateway",
];

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

    /// Validate the profile's internal consistency without probing live sockets.
    ///
    /// Checks referential integrity across sections: launch order refers to
    /// declared primals, mesh config is coherent, roles are known values.
    #[must_use]
    pub fn structural_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        let primal_names: Vec<&str> = self.primals.keys().map(String::as_str).collect();

        if let Some(ref role) = self.profile.role {
            if !COMMON_ROLES.contains(&role.as_str()) {
                warnings.push(format!(
                    "profile.role '{role}' is not a common role — verify spelling (common: {})",
                    COMMON_ROLES.join(", ")
                ));
            }
        }

        if let Some(ref launch) = self.launch {
            for slug in &launch.order {
                if !self.primals.contains_key(slug) {
                    warnings.push(format!(
                        "launch.order references '{slug}' which is not declared in [primals]"
                    ));
                }
            }
            if let Some(ref pa) = launch.parallel_after {
                if !launch.order.contains(pa) {
                    warnings.push(format!(
                        "launch.parallel_after '{pa}' is not in launch.order"
                    ));
                }
            }
            for name in &primal_names {
                if !launch.order.iter().any(|s| s == *name) {
                    warnings.push(format!(
                        "primal '{name}' declared but missing from launch.order"
                    ));
                }
            }
        }

        if let Some(ref mesh) = self.mesh {
            if mesh.federation_enabled == Some(true) && mesh.node_id.is_none() {
                warnings.push("mesh.federation_enabled is true but mesh.node_id is not set".into());
            }
            if mesh.federation_enabled == Some(true) && mesh.peers.is_empty() {
                warnings.push("mesh.federation_enabled is true but mesh.peers is empty".into());
            }
        }

        if let Some(ref health) = self.health {
            for crit in &health.critical {
                if !self.primals.contains_key(crit) {
                    warnings.push(format!(
                        "health.critical references '{crit}' which is not declared in [primals]"
                    ));
                }
            }
            if let Some(min) = health.min_healthy {
                if min > self.primals.len() {
                    warnings.push(format!(
                        "health.min_healthy ({min}) exceeds total primals ({})",
                        self.primals.len()
                    ));
                }
            }
        }

        warnings
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
    /// Deployment role — checked against common roles for typo warnings.
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
    /// JSON-RPC methods to probe for readiness (profile-driven, replaces hardcoded tables).
    #[serde(default)]
    pub probe_methods: Vec<String>,
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
    /// Validated by `structural_warnings()` against `launch.order`.
    #[serde(default)]
    pub parallel_after: Option<String>,
}

/// Mesh/federation configuration.
#[derive(Debug, Deserialize)]
pub struct MeshConfig {
    /// Node identity — `structural_warnings()` flags missing `node_id` when federation is enabled.
    #[serde(default)]
    pub node_id: Option<String>,
    /// Whether federation is enabled for this profile.
    #[serde(default)]
    pub federation_enabled: Option<bool>,
    /// Bootstrap peer addresses — validated by `structural_warnings()`.
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
    /// Structural warnings from profile validation (independent of live probing).
    pub structural_warnings: Vec<String>,
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

    let structural_warnings = profile.structural_warnings();

    ValidationResult {
        profile_name: profile.profile.name.clone(),
        total_declared: profile.primals.len(),
        healthy,
        missing,
        critical_met,
        min_healthy_met,
        structural_warnings,
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
                probe_methods: vec![],
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
    fn structural_warnings_clean_profile() {
        let toml_str = r#"
[profile]
name = "clean"
role = "production"

[primals]
beardog = { required = true }
songbird = { required = true }

[launch]
order = ["beardog", "songbird"]
parallel_after = "beardog"

[mesh]
node_id = "gate-1"
federation_enabled = true
peers = ["10.13.37.1:7700"]

[health]
min_healthy = 1
critical = ["beardog"]
"#;
        let profile: NucleusProfile = toml::from_str(toml_str).unwrap();
        assert!(profile.structural_warnings().is_empty());
    }

    #[test]
    fn structural_warnings_unknown_role() {
        let toml_str = r#"
[profile]
name = "test"
role = "undefined_role"
"#;
        let profile: NucleusProfile = toml::from_str(toml_str).unwrap();
        let warnings = profile.structural_warnings();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("undefined_role"));
    }

    #[test]
    fn structural_warnings_launch_order_references_undeclared() {
        let mut primals = BTreeMap::new();
        primals.insert(
            "beardog".into(),
            PrimalEntry {
                required: true,
                role: None,
                probe_methods: vec![],
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
            health: None,
            launch: Some(LaunchConfig {
                order: vec!["beardog".into(), "ghost".into()],
                parallel_after: None,
            }),
            mesh: None,
        };
        let warnings = profile.structural_warnings();
        assert!(warnings.iter().any(|w| w.contains("ghost")));
    }

    #[test]
    fn structural_warnings_parallel_after_not_in_order() {
        let mut primals = BTreeMap::new();
        primals.insert(
            "beardog".into(),
            PrimalEntry {
                required: true,
                role: None,
                probe_methods: vec![],
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
            health: None,
            launch: Some(LaunchConfig {
                order: vec!["beardog".into()],
                parallel_after: Some("songbird".into()),
            }),
            mesh: None,
        };
        let warnings = profile.structural_warnings();
        assert!(warnings.iter().any(|w| w.contains("parallel_after")));
    }

    #[test]
    fn structural_warnings_federation_without_node_id() {
        let profile = NucleusProfile {
            profile: ProfileMeta {
                name: "test".into(),
                description: None,
                extends: None,
                role: None,
            },
            primals: BTreeMap::new(),
            health: None,
            launch: None,
            mesh: Some(MeshConfig {
                node_id: None,
                federation_enabled: Some(true),
                peers: vec!["10.0.0.1:7700".into()],
            }),
        };
        let warnings = profile.structural_warnings();
        assert!(warnings.iter().any(|w| w.contains("node_id")));
    }

    #[test]
    fn structural_warnings_critical_references_undeclared() {
        let profile = NucleusProfile {
            profile: ProfileMeta {
                name: "test".into(),
                description: None,
                extends: None,
                role: None,
            },
            primals: BTreeMap::new(),
            health: Some(HealthConfig {
                min_healthy: None,
                critical: vec!["phantom".into()],
            }),
            launch: None,
            mesh: None,
        };
        let warnings = profile.structural_warnings();
        assert!(warnings.iter().any(|w| w.contains("phantom")));
    }

    #[test]
    fn structural_warnings_min_healthy_exceeds_total() {
        let mut primals = BTreeMap::new();
        primals.insert(
            "beardog".into(),
            PrimalEntry {
                required: true,
                role: None,
                probe_methods: vec![],
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
                min_healthy: Some(5),
                critical: vec![],
            }),
            launch: None,
            mesh: None,
        };
        let warnings = profile.structural_warnings();
        assert!(warnings.iter().any(|w| w.contains("min_healthy")));
    }

    #[test]
    #[ignore = "requires monorepo layout (../../infra/plasmidBin/profiles/)"]
    fn parse_real_irongate_profile() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../infra/plasmidBin/profiles/irongate-full.toml");
        let profile = parse_profile(&path).unwrap();
        assert_eq!(profile.profile.name, "irongate-full");
        assert_eq!(profile.primals.len(), 13);
    }

    #[test]
    #[ignore = "requires monorepo layout (../../infra/plasmidBin/profiles/)"]
    fn parse_real_flockgate_profile() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../infra/plasmidBin/profiles/flockgate-wan.toml");
        let profile = parse_profile(&path).unwrap();
        assert_eq!(profile.profile.name, "flockgate-wan");
        assert_eq!(profile.primals.len(), 13);
        assert!(profile.mesh.as_ref().unwrap().federation_enabled.unwrap());
    }
}
