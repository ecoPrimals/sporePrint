// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tower P1 readiness probing — validates method availability on Tower primals.
//!
//! Probes `BearDog` (crypto identity), Songbird (mesh routing), and `SkunkBat`
//! (threat detection) for P1-critical JSON-RPC methods. Used by the
//! `tower-status` CLI subcommand to report deployment readiness.
//!
//! ## Design
//!
//! Each Tower primal is probed independently via its UDS socket. Methods are
//! classified as "available" if the response is NOT `-32601 Method not found`.
//! A `-32603` (internal error like "not initialized") still means the method
//! EXISTS — it just needs activation.

use crate::discovery;
use crate::paths::probe_timeout;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::LazyLock;

/// A default Tower primal probe definition (from bundled TOML).
#[derive(Debug, Deserialize)]
struct DefaultProbe {
    slug: String,
    methods: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DefaultProbeFile {
    probes: Vec<DefaultProbe>,
}

type ProbeList = Vec<(String, Vec<String>)>;

static DEFAULT_TOWER_PROBES: LazyLock<Result<ProbeList, String>> = LazyLock::new(|| {
    const EMBEDDED: &str = include_str!("../default_tower_probes.toml");
    let file: DefaultProbeFile = toml::from_str(EMBEDDED)
        .map_err(|e| format!("embedded default_tower_probes.toml: {e}"))?;
    Ok(file
        .probes
        .into_iter()
        .map(|p| (p.slug, p.methods))
        .collect())
});

/// Return the default Tower primal P1 readiness methods (fallback when profile has no `probe_methods`).
fn default_tower_probes() -> Result<&'static [(String, Vec<String>)], String> {
    DEFAULT_TOWER_PROBES
        .as_ref()
        .map(Vec::as_slice)
        .map_err(Clone::clone)
}

/// Result of probing a single method on a Tower primal.
#[derive(Debug)]
pub struct MethodProbe {
    pub method: String,
    pub available: bool,
    pub response_summary: Option<String>,
}

/// Result of probing all P1 methods on the Tower.
#[derive(Debug)]
pub struct TowerStatus {
    pub primals: Vec<TowerPrimalStatus>,
}

/// Per-primal Tower P1 readiness.
#[derive(Debug)]
pub struct TowerPrimalStatus {
    pub name: String,
    pub socket_path: Option<String>,
    pub methods: Vec<MethodProbe>,
}

/// Probe Tower primals for P1 method availability.
///
/// When a profile is provided, primals with `probe_methods` declared use those
/// methods instead of the built-in defaults. This makes the probe table
/// data-driven (from TOML profiles) rather than hardcoded in Rust.
pub fn probe_tower_status(
    profile: Option<&crate::nucleus::NucleusProfile>,
) -> Result<TowerStatus, String> {
    let probe_targets = build_probe_targets(profile)?;
    let mut primals = Vec::new();

    for (slug, methods) in &probe_targets {
        let env_var = format!("{}_SOCKET", slug.to_uppercase());
        let socket = discovery::probe_socket(slug, &env_var);

        let method_results: Vec<MethodProbe> = socket.as_ref().map_or_else(
            || {
                methods
                    .iter()
                    .map(|method| MethodProbe {
                        method: method.clone(),
                        available: false,
                        response_summary: Some("socket not found".into()),
                    })
                    .collect()
            },
            |path| {
                methods
                    .iter()
                    .map(|method| probe_single_method(path, method))
                    .collect()
            },
        );

        primals.push(TowerPrimalStatus {
            name: slug.clone(),
            socket_path: socket,
            methods: method_results,
        });
    }

    Ok(TowerStatus { primals })
}

/// Build the probe target list: profile-driven methods override defaults.
///
/// Returns `Err` only if no profile is provided and the embedded probe TOML is corrupt.
fn build_probe_targets(
    profile: Option<&crate::nucleus::NucleusProfile>,
) -> Result<Vec<(String, Vec<String>)>, String> {
    if let Some(p) = profile {
        let mut targets: Vec<(String, Vec<String>)> = Vec::new();
        for (name, entry) in &p.primals {
            if !entry.probe_methods.is_empty() {
                targets.push((name.clone(), entry.probe_methods.clone()));
            }
        }
        if !targets.is_empty() {
            return Ok(targets);
        }
    }

    Ok(default_tower_probes()?
        .iter()
        .map(|(slug, methods)| (slug.clone(), methods.clone()))
        .collect())
}

/// Probe a single JSON-RPC method on a socket, returning availability.
fn probe_single_method(socket_path: &str, method: &str) -> MethodProbe {
    let Ok(mut reader) = crate::ipc::connect_uds(socket_path, probe_timeout()) else {
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some("connect failed".into()),
        };
    };

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": {},
        "id": 1
    });

    let resp = match crate::ipc::send_rpc(&mut reader, &request) {
        Ok(r) => r,
        Err(e) => {
            let msg = e.to_string();
            let summary = if msg.contains("read") {
                "read timeout"
            } else if msg.contains("write") {
                "write failed"
            } else {
                "rpc failed"
            };
            return MethodProbe {
                method: method.to_string(),
                available: false,
                response_summary: Some(summary.into()),
            };
        }
    };

    if crate::ipc::is_method_not_found(&resp) {
        let msg = crate::ipc::extract_error_message(&resp).unwrap_or_default();
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some(msg),
        };
    }

    resp.get("error").map_or_else(
        || {
            let summary = summarize_result(resp.get("result"));
            MethodProbe {
                method: method.to_string(),
                available: true,
                response_summary: summary,
            }
        },
        |err| {
            let code = err.get("code").and_then(Value::as_i64).unwrap_or(0);
            let msg = err
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            MethodProbe {
                method: method.to_string(),
                available: code != crate::ipc::JSONRPC_METHOD_NOT_FOUND,
                response_summary: Some(format!("[{code}] {msg}")),
            }
        },
    )
}

/// Produce a short summary of a JSON-RPC result for display.
fn summarize_result(result: Option<&Value>) -> Option<String> {
    let r = result?;
    r.as_object().map_or_else(
        || {
            r.as_array().map_or_else(
                || Some(r.to_string()),
                |arr| Some(format!("[{} items]", arr.len())),
            )
        },
        |obj| {
            let keys: Vec<&str> = obj.keys().map(String::as_str).take(5).collect();
            Some(format!("{{{}}}", keys.join(", ")))
        },
    )
}

/// Print Tower status report to stdout.
pub fn print_tower_status(status: &TowerStatus) {
    println!("sporePrint: Tower P1 readiness probe");
    println!();

    for primal in &status.primals {
        let socket_display = primal.socket_path.as_deref().unwrap_or("NOT FOUND");
        println!("  {} → {socket_display}", primal.name);

        for m in &primal.methods {
            let icon = if m.available { "✅" } else { "❌" };
            let summary = m.response_summary.as_deref().unwrap_or("");
            println!("    {icon} {} {summary}", m.method);
        }
        println!();
    }

    let total_methods: usize = status.primals.iter().map(|p| p.methods.len()).sum();
    let available: usize = status
        .primals
        .iter()
        .flat_map(|p| &p.methods)
        .filter(|m| m.available)
        .count();

    println!("  Tower P1: {available}/{total_methods} methods available");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tower_probes_cover_all_three_primals() {
        let probes = default_tower_probes().unwrap();
        assert_eq!(probes.len(), 3);
        let slugs: Vec<&str> = probes.iter().map(|(s, _)| s.as_str()).collect();
        assert!(slugs.contains(&"beardog"));
        assert!(slugs.contains(&"songbird"));
        assert!(slugs.contains(&"skunkbat"));
    }

    #[test]
    fn build_probe_targets_defaults_without_profile() {
        let targets = build_probe_targets(None).unwrap();
        assert_eq!(targets.len(), 3);
        assert_eq!(targets[0].0, "beardog");
        assert!(!targets[0].1.is_empty());
    }

    #[test]
    fn build_probe_targets_uses_profile_methods() {
        use crate::nucleus::*;
        use std::collections::BTreeMap;

        let mut primals = BTreeMap::new();
        primals.insert(
            "custom_primal".into(),
            PrimalEntry {
                required: true,
                role: Some("test".into()),
                probe_methods: vec!["custom.method".into(), "custom.other".into()],
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
            launch: None,
            mesh: None,
        };

        let targets = build_probe_targets(Some(&profile)).unwrap();
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].0, "custom_primal");
        assert_eq!(targets[0].1, vec!["custom.method", "custom.other"]);
    }

    #[test]
    fn build_probe_targets_falls_back_when_no_probe_methods() {
        use crate::nucleus::*;
        use std::collections::BTreeMap;

        let mut primals = BTreeMap::new();
        primals.insert(
            "beardog".into(),
            PrimalEntry {
                required: true,
                role: Some("crypto".into()),
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
            launch: None,
            mesh: None,
        };

        let targets = build_probe_targets(Some(&profile)).unwrap();
        assert_eq!(
            targets.len(),
            3,
            "falls back to defaults when no probe_methods in profile"
        );
    }

    #[test]
    fn summarize_result_object() {
        let val = serde_json::json!({"foo": 1, "bar": 2});
        let summary = summarize_result(Some(&val)).unwrap();
        assert!(summary.contains("foo"));
        assert!(summary.contains("bar"));
    }

    #[test]
    fn summarize_result_array() {
        let val = serde_json::json!([1, 2, 3]);
        let summary = summarize_result(Some(&val)).unwrap();
        assert_eq!(summary, "[3 items]");
    }

    #[test]
    fn summarize_result_none() {
        assert!(summarize_result(None).is_none());
    }

    #[test]
    fn method_probe_debug_format() {
        let probe = MethodProbe {
            method: "test.method".into(),
            available: true,
            response_summary: Some("{ok}".into()),
        };
        let debug = format!("{probe:?}");
        assert!(debug.contains("test.method"));
        assert!(debug.contains("true"));
    }
}
