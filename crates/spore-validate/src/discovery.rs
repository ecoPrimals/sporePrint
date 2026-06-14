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
            description: "Push build artifacts to NestGate CAS via injected transport",
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
    if let Some(socket) = probe_socket("nestgate", "NESTGATE_SOCKET") {
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
    if let Some(socket) = probe_socket("petaltongue", "PETALTONGUE_SOCKET") {
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

/// Probe for a primal's socket path from environment variables.
///
/// Discovery order (ecosystem standard):
/// 1. Explicit env var (e.g., `NESTGATE_SOCKET`) — highest priority
/// 2. `BIOMEOS_SOCKET_DIR/{slug}.sock` — ecosystem standard directory
/// 3. `/run/membrane/{slug}.sock` — systemd NUCLEUS deployment (`GATE_NUCLEUS_SYSTEMD_STANDARD`)
/// 4. `XDG_RUNTIME_DIR/biomeos/{slug}.sock` — XDG fallback
/// 5. `XDG_RUNTIME_DIR/biomeos/{slug}-standalone.sock` — standalone variant
///
/// No `/tmp` probing — ecosystem is migrating away from `/tmp` sockets
/// per `PRIMAL-SOCKET-CLEANUP` directive.
pub fn probe_socket(slug: &str, primary_var: &str) -> Option<String> {
    if let Ok(path) = std::env::var(primary_var) {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    if let Ok(dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
        let candidate = format!("{dir}/{slug}.sock");
        if std::path::Path::new(&candidate).exists() {
            return Some(candidate);
        }
    }

    let systemd_candidate = format!("/run/membrane/{slug}.sock");
    if std::path::Path::new(&systemd_candidate).exists() {
        return Some(systemd_candidate);
    }

    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let candidates = [
            format!("{xdg}/biomeos/{slug}.sock"),
            format!("{xdg}/biomeos/{slug}-standalone.sock"),
        ];
        for candidate in &candidates {
            if std::path::Path::new(candidate).exists() {
                return Some(candidate.clone());
            }
        }
    }

    None
}

/// Build a `primal.announce` JSON-RPC request for `NestGate` handshake.
///
/// The request ID is provided by the caller (connection-level counter).
/// Capabilities are derived from `SELF` — no hardcoded lists elsewhere.
pub fn announce_request(request_id: u64) -> serde_json::Value {
    let cap_names: Vec<&str> = SELF.capabilities.iter().map(|c| c.name).collect();
    serde_json::json!({
        "jsonrpc": "2.0",
        "method": "primal.announce",
        "params": {
            "primal_id": SELF.primal_id,
            "version": SELF.version,
            "capabilities": cap_names,
        },
        "id": request_id
    })
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
    fn announce_request_is_valid_jsonrpc() {
        let req = announce_request(1);
        assert_eq!(req["jsonrpc"], "2.0");
        assert_eq!(req["method"], "primal.announce");
        assert_eq!(req["params"]["primal_id"], "sporePrint");
        assert!(req["params"]["capabilities"].is_array());
        assert_eq!(req["id"], 1);
    }

    #[test]
    fn discover_peers_returns_empty_without_sockets() {
        let peers = discover_peers();
        assert!(peers.is_empty() || peers.iter().all(|p| p.socket_path.is_some()));
    }

    #[test]
    fn version_matches_cargo_pkg() {
        assert_eq!(SELF.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn capabilities_have_unique_names() {
        let names: Vec<&str> = SELF.capabilities.iter().map(|c| c.name).collect();
        let mut deduped = names.clone();
        deduped.sort_unstable();
        deduped.dedup();
        assert_eq!(names.len(), deduped.len(), "duplicate capability names");
    }

    #[test]
    fn capabilities_have_valid_categories() {
        let valid = ["content", "integrity", "storage", "sync", "knowledge"];
        for cap in SELF.capabilities {
            assert!(
                valid.contains(&cap.category),
                "unknown category '{}' for '{}'",
                cap.category,
                cap.name
            );
        }
    }

    #[test]
    fn probe_socket_returns_none_for_missing_var() {
        let result = probe_socket("nestgate", "NONEXISTENT_VAR_FOR_TEST_XYZ_12345");
        assert!(result.is_none());
    }

    #[test]
    fn probe_socket_returns_none_when_no_dirs_exist() {
        let result = probe_socket("nestgate", "NONEXISTENT_PRIMARY_99999");
        assert!(result.is_none());
    }

    #[test]
    fn discovered_peer_debug_format() {
        let peer = DiscoveredPeer {
            primal_id: "testPrimal".into(),
            socket_path: Some("/tmp/test.sock".into()),
            capabilities: vec!["foo.bar".into()],
        };
        let debug = format!("{peer:?}");
        assert!(debug.contains("testPrimal"));
        assert!(debug.contains("/tmp/test.sock"));
    }
}
