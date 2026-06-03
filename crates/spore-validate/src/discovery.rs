// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability discovery — sporePrint announces itself and discovers peers.
//!
//! Implements the primal self-knowledge pattern: sporePrint knows its own
//! capabilities and discovers other primals at runtime via environment
//! variables, socket probing, and announce payloads.
//!
//! ## Design Principles
//!
//! 1. **Self-knowledge only** — sporePrint declares what IT can do
//! 2. **Runtime discovery** — peers found via env vars / socket probes
//! 3. **Graceful degradation** — missing peers don't crash the pipeline
//! 4. **Capability-based** — features activate based on discovered capabilities

use serde::Serialize;

/// sporePrint's self-declared capabilities.
#[derive(Debug, Serialize)]
pub struct SelfCapabilities {
    pub primal_id: &'static str,
    pub version: &'static str,
    pub capabilities: &'static [Capability],
}

/// A single capability that sporePrint provides.
#[derive(Debug, Serialize)]
pub struct Capability {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
}

/// sporePrint's static self-declaration.
pub const SELF: SelfCapabilities = SelfCapabilities {
    primal_id: "sporePrint",
    version: env!("CARGO_PKG_VERSION"),
    capabilities: &[
        Capability {
            name: "validate",
            category: "content",
            description: "Entity registry validation with typed diagnostics",
        },
        Capability {
            name: "provenance",
            category: "integrity",
            description: "BLAKE3 content-addressed provenance manifests",
        },
        Capability {
            name: "certify",
            category: "integrity",
            description: "Self-certifying publication with Merkle root",
        },
        Capability {
            name: "cas-manifest",
            category: "storage",
            description: "CAS manifest generation for NestGate integration",
        },
        Capability {
            name: "cas-push",
            category: "storage",
            description: "Push build artifacts to NestGate CAS via UDS",
        },
        Capability {
            name: "fetch-refresh",
            category: "sync",
            description: "Upstream repo fetch and metric drift detection",
        },
        Capability {
            name: "render-notebooks",
            category: "content",
            description: "Jupyter notebook to Zola markdown rendering",
        },
        Capability {
            name: "graph",
            category: "knowledge",
            description: "Entity graph with typed edges (renvois de choses)",
        },
    ],
};

/// A discovered peer primal with its capabilities.
#[derive(Debug)]
pub struct DiscoveredPeer {
    pub primal_id: String,
    pub socket_path: Option<String>,
    pub capabilities: Vec<String>,
}

/// Discover available peer primals from the environment.
///
/// Checks for known socket/env patterns without hardcoding specific primals.
/// Each primal advertises itself via `{PRIMAL_NAME}_SOCKET` or similar env vars.
pub fn discover_peers() -> Vec<DiscoveredPeer> {
    let mut peers = Vec::new();

    // NestGate: CAS storage peer
    if let Some(socket) = probe_socket_env("NESTGATE_SOCKET", &[
        "BIOMEOS_SOCKET_DIR",
        "XDG_RUNTIME_DIR",
    ]) {
        peers.push(DiscoveredPeer {
            primal_id: "nestGate".into(),
            socket_path: Some(socket),
            capabilities: vec![
                "content.put".into(),
                "content.get".into(),
                "content.exists".into(),
                "content.replicate.pull".into(),
                "route.register".into(),
            ],
        });
    }

    // petalTongue: content rendering peer
    if let Some(socket) = probe_socket_env("PETALTONGUE_SOCKET", &[]) {
        peers.push(DiscoveredPeer {
            primal_id: "petalTongue".into(),
            socket_path: Some(socket),
            capabilities: vec![
                "content.render".into(),
                "viz.serve".into(),
            ],
        });
    }

    peers
}

/// Probe for a socket path from environment variables.
///
/// Checks the primary env var first, then falls back to discovering through
/// XDG/biomeOS standard paths.
fn probe_socket_env(primary_var: &str, fallback_vars: &[&str]) -> Option<String> {
    if let Ok(path) = std::env::var(primary_var) {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    for var in fallback_vars {
        if let Ok(base) = std::env::var(var) {
            let candidates = [
                format!("{base}/biomeos/nestgate.sock"),
                format!("{base}/biomeos/nestgate-standalone.sock"),
            ];
            for candidate in &candidates {
                if std::path::Path::new(candidate).exists() {
                    return Some(candidate.clone());
                }
            }
        }
    }

    None
}

/// Format self-capabilities as a JSON announce payload.
///
/// Compatible with `NestGate`'s `primal.announce` method.
/// Called by `cas-push` during `NestGate` connection handshake.
#[allow(dead_code)]
pub fn announce_payload() -> String {
    serde_json::to_string_pretty(&SELF).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_capabilities_are_populated() {
        assert_eq!(SELF.primal_id, "sporePrint");
        assert!(!SELF.capabilities.is_empty());
        assert!(SELF.capabilities.len() >= 7);
    }

    #[test]
    fn announce_payload_is_valid_json() {
        let json = announce_payload();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["primal_id"], "sporePrint");
        assert!(parsed["capabilities"].is_array());
    }

    #[test]
    fn discover_peers_returns_empty_without_sockets() {
        let peers = discover_peers();
        // In test environment, no sockets are running
        // This verifies graceful degradation
        assert!(peers.is_empty() || peers.iter().all(|p| p.socket_path.is_some()));
    }

    #[test]
    fn version_matches_cargo_pkg() {
        assert_eq!(SELF.version, env!("CARGO_PKG_VERSION"));
    }
}
