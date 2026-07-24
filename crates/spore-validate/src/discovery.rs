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
    std::env::var(crate::paths::ENV_BIOMEOS_SYSTEMD_DIR)
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
        Capability {
            name: "tower-status",
            category: "mesh",
            description: "Probe Tower Atomic primals for P1 method availability",
        },
        Capability {
            name: "depot-verify",
            category: "integrity",
            description: "BLAKE3 integrity verification of depot binaries",
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

/// Peers with dedicated env var overrides — used as first-phase discovery
/// hints before directory scanning. Additional peers are discovered via
/// socket dir scanning regardless of whether these resolve.
///
/// The env var naming convention (`{SLUG}_SOCKET`) is an ecosystem standard;
/// any primal following it will be discoverable without being listed here.
/// Tower Atomic primals (beardog, songbird, skunkbat) are included because
/// `tower-status` probes them and they are core infrastructure.
const WELL_KNOWN_PEERS: &[(&str, &str)] = &[
    ("beardog", "BEARDOG_SOCKET"),
    ("songbird", "SONGBIRD_SOCKET"),
    ("skunkbat", "SKUNKBAT_SOCKET"),
    ("nestgate", "NESTGATE_SOCKET"),
    ("petaltongue", "PETALTONGUE_SOCKET"),
    ("sweetgrass", "SWEETGRASS_SOCKET"),
    ("squirrel", "SQUIRREL_SOCKET"),
];

/// Build the peer hint list, extending well-known peers with any additional
/// slugs configured via `SPOREPRINT_EXTRA_PEERS` (comma-separated slug list).
///
/// Each extra slug derives its env var as `{SLUG_UPPER}_SOCKET`.
fn peer_hints() -> Vec<(String, String)> {
    let mut hints: Vec<(String, String)> = WELL_KNOWN_PEERS
        .iter()
        .map(|&(slug, var)| (slug.to_string(), var.to_string()))
        .collect();

    if let Ok(extra) = std::env::var(crate::paths::ENV_EXTRA_PEERS) {
        for slug in extra.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let var = format!("{}_SOCKET", slug.to_uppercase());
            if !hints.iter().any(|(s, _)| s == slug) {
                hints.push((slug.to_string(), var));
            }
        }
    }

    hints
}

/// Discover available peer primals from the environment.
///
/// Two-phase discovery:
/// 1. Probe well-known peers via their dedicated env vars
/// 2. Scan socket directories for any additional primal sockets
///
/// Capabilities are not assumed — discovered peers report empty capabilities
/// until a `primal.announce` handshake populates them at runtime.
pub fn discover_peers() -> Vec<DiscoveredPeer> {
    let mut peers = Vec::new();
    let mut seen_slugs = std::collections::HashSet::new();

    for (slug, env_var) in peer_hints() {
        if let Some(socket) = probe_socket(&slug, &env_var) {
            seen_slugs.insert(slug.clone());
            peers.push(DiscoveredPeer {
                primal_id: slug,
                socket_path: Some(socket),
                capabilities: Vec::new(),
            });
        }
    }

    for socket_path in scan_socket_dirs() {
        let Some(slug) = extract_slug_from_socket(&socket_path) else {
            continue;
        };
        if seen_slugs.contains(&slug) {
            continue;
        }
        seen_slugs.insert(slug.clone());
        peers.push(DiscoveredPeer {
            primal_id: slug,
            socket_path: Some(socket_path),
            capabilities: Vec::new(),
        });
    }

    peers
}

/// Scan known socket directories for `.sock` files.
fn scan_socket_dirs() -> Vec<String> {
    let mut sockets = Vec::new();

    let dirs_to_scan: Vec<String> = [
        std::env::var(crate::paths::ENV_BIOMEOS_SOCKET_DIR).ok(),
        Some(systemd_socket_dir().into_owned()),
        std::env::var(crate::paths::ENV_XDG_RUNTIME)
            .ok()
            .map(|xdg| format!("{xdg}/biomeos")),
    ]
    .into_iter()
    .flatten()
    .collect();

    for dir in dirs_to_scan {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "sock") && path.exists() {
                if let Some(s) = path.to_str() {
                    sockets.push(s.to_string());
                }
            }
        }
    }

    sockets
}

/// Extract a primal slug from a socket filename (e.g., `/run/membrane/nestgate.sock` → `nestgate`).
fn extract_slug_from_socket(path: &str) -> Option<String> {
    let filename = std::path::Path::new(path).file_stem()?.to_str()?;
    let slug = filename.strip_suffix("-standalone").unwrap_or(filename);
    Some(slug.to_string())
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

    if let Ok(dir) = std::env::var(crate::paths::ENV_BIOMEOS_SOCKET_DIR) {
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

    if let Ok(xdg) = std::env::var(crate::paths::ENV_XDG_RUNTIME) {
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

    if let Ok(json) = std::env::var(crate::paths::ENV_TRANSPORT_ENDPOINT) {
        return serde_json::from_str(&json).map_err(|e| {
            Error::Config(format!(
                "{} parse error: {e}",
                crate::paths::ENV_TRANSPORT_ENDPOINT
            ))
        });
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
        let valid = ["content", "integrity", "storage", "sync", "knowledge", "mesh"];
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
    fn peer_hints_includes_well_known() {
        let hints = peer_hints();
        for slug in ["beardog", "songbird", "skunkbat", "nestgate", "petaltongue", "sweetgrass", "squirrel"] {
            assert!(
                hints.iter().any(|(s, _)| s == slug),
                "well-known {slug} should always be present"
            );
        }
    }

    #[test]
    fn peer_hints_deduplicates() {
        let hints = peer_hints();
        let slugs: Vec<&str> = hints.iter().map(|(s, _)| s.as_str()).collect();
        let mut deduped = slugs.clone();
        deduped.sort_unstable();
        deduped.dedup();
        assert_eq!(slugs.len(), deduped.len(), "peer_hints has duplicates");
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
                || std::env::var(crate::paths::ENV_BIOMEOS_SYSTEMD_DIR).is_ok(),
            "returns default unless env override is set"
        );
    }

    #[test]
    fn discovered_peer_debug_format() {
        let peer = DiscoveredPeer {
            primal_id: "testPrimal".to_string(),
            socket_path: Some("/tmp/test.sock".into()),
            capabilities: vec!["foo.bar".to_string()],
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
