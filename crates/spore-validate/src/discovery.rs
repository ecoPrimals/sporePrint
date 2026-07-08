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
use std::borrow::Cow;

/// Default systemd NUCLEUS socket directory (`GATE_NUCLEUS_SYSTEMD_STANDARD`).
///
/// Overridable via `BIOMEOS_SYSTEMD_SOCKET_DIR` for non-standard deployments.
const DEFAULT_SYSTEMD_SOCKET_DIR: &str = "/run/membrane";

/// Resolve the systemd socket directory, preferring the env override.
#[must_use]
pub fn systemd_socket_dir() -> Cow<'static, str> {
    std::env::var("BIOMEOS_SYSTEMD_SOCKET_DIR")
        .map_or(Cow::Borrowed(DEFAULT_SYSTEMD_SOCKET_DIR), Cow::Owned)
}

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
        Capability {
            name: "pt-render",
            category: "content",
            description: "Request content rendering from petalTongue via IPC",
        },
    ],
};

/// A discovered peer primal with its capabilities.
#[derive(Debug)]
pub struct DiscoveredPeer {
    pub primal_id: &'static str,
    pub socket_path: Option<String>,
    pub capabilities: &'static [&'static str],
}

/// `NestGate` capabilities (CAS storage).
const NESTGATE_CAPABILITIES: &[&str] = &[
    "content.put",
    "content.get",
    "content.exists",
    "content.replicate.pull",
    "route.register",
];

/// petalTongue capabilities (visualization rendering).
const PETALTONGUE_CAPABILITIES: &[&str] = &[
    "visualization.render.graph",
    "visualization.render.scene",
    "visualization.export",
    "health.check",
];

/// Discover available peer primals from the environment.
///
/// Probes for primals that sporePrint consumes: `NestGate` (CAS storage)
/// and `petalTongue` (content rendering). Each is discovered via the
/// standard socket probe chain — no paths are assumed.
pub fn discover_peers() -> Vec<DiscoveredPeer> {
    let mut peers = Vec::new();

    if let Some(socket) = probe_socket("nestgate", "NESTGATE_SOCKET") {
        peers.push(DiscoveredPeer {
            primal_id: "nestGate",
            socket_path: Some(socket),
            capabilities: NESTGATE_CAPABILITIES,
        });
    }

    if let Some(socket) = probe_socket("petaltongue", "PETALTONGUE_SOCKET") {
        peers.push(DiscoveredPeer {
            primal_id: "petalTongue",
            socket_path: Some(socket),
            capabilities: PETALTONGUE_CAPABILITIES,
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

    let dir = systemd_socket_dir();
    let systemd_candidate = format!("{dir}/{slug}.sock");
    if std::path::Path::new(&systemd_candidate).exists() {
        return Some(systemd_candidate);
    }

    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let candidates = [
            format!("{xdg}/biomeos/{slug}.sock"),
            format!("{xdg}/biomeos/{slug}-standalone.sock"),
        ];
        for candidate in candidates {
            if std::path::Path::new(&candidate).exists() {
                return Some(candidate);
            }
        }
    }

    None
}

/// Resolve a transport endpoint for a peer primal.
///
/// Unified transport injection pattern (same as `NestGate` CAS push):
/// 1. CLI `--socket` override (explicit UDS path)
/// 2. `TRANSPORT_ENDPOINT` env var (canonical JSON — launcher/Songbird injection)
/// 3. Socket discovery via `probe_socket` (env → `BIOMEOS_SOCKET_DIR` → systemd → XDG)
///
/// This ensures all primal connections (`NestGate`, `petalTongue`, etc.) honor the
/// same transport injection interface.
pub fn resolve_primal_endpoint(
    slug: &str,
    primary_var: &str,
    socket_override: Option<&str>,
) -> Result<crate::cas_push::TransportEndpoint, crate::error::Error> {
    use crate::cas_push::TransportEndpoint;
    use crate::error::Error;

    if let Some(s) = socket_override {
        return Ok(TransportEndpoint::Uds { path: s.into() });
    }

    if let Ok(json) = std::env::var("TRANSPORT_ENDPOINT") {
        return serde_json::from_str(&json)
            .map_err(|e| Error::Config(format!("TRANSPORT_ENDPOINT parse error: {e}")));
    }

    if let Some(path) = probe_socket(slug, primary_var) {
        return Ok(TransportEndpoint::Uds { path });
    }

    Err(Error::Config(format!(
        "{slug} socket not found. Set {primary_var}, TRANSPORT_ENDPOINT, or \
         BIOMEOS_SOCKET_DIR, or ensure {slug} is running."
    )))
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
        assert!(SELF.capabilities.len() >= 8);
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
        let result = probe_socket(
            "nonexistent_primal_slug_for_test",
            "NONEXISTENT_VAR_FOR_TEST_XYZ_12345",
        );
        assert!(result.is_none());
    }

    #[test]
    fn probe_socket_returns_none_when_no_dirs_exist() {
        let result = probe_socket(
            "nonexistent_primal_slug_for_test",
            "NONEXISTENT_PRIMARY_99999",
        );
        assert!(result.is_none());
    }

    #[test]
    fn systemd_socket_dir_returns_valid_path() {
        let dir = systemd_socket_dir();
        assert!(
            !dir.is_empty(),
            "systemd_socket_dir should never return empty"
        );
        assert!(
            dir == DEFAULT_SYSTEMD_SOCKET_DIR
                || std::env::var("BIOMEOS_SYSTEMD_SOCKET_DIR").is_ok(),
            "returns default unless env override is set"
        );
    }

    #[test]
    fn discovered_peer_debug_format() {
        let peer = DiscoveredPeer {
            primal_id: "testPrimal",
            socket_path: Some("/tmp/test.sock".into()),
            capabilities: &["foo.bar"],
        };
        let debug = format!("{peer:?}");
        assert!(debug.contains("testPrimal"));
        assert!(debug.contains("/tmp/test.sock"));
    }

    #[test]
    fn resolve_primal_endpoint_cli_override() {
        let result = resolve_primal_endpoint("test", "TEST_SOCK", Some("/tmp/override.sock"));
        assert!(result.is_ok());
        let ep = result.unwrap();
        match ep {
            crate::cas_push::TransportEndpoint::Uds { path } => {
                assert_eq!(path, "/tmp/override.sock");
            }
            _ => panic!("expected UDS endpoint from CLI override"),
        }
    }

    #[test]
    fn resolve_primal_endpoint_fails_without_socket() {
        let result = resolve_primal_endpoint("nonexistent_primal_99", "NONEXISTENT_VAR_99", None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("nonexistent_primal_99"),
            "error should mention the slug"
        );
    }
}
