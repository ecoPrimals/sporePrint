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
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;

/// IPC timeout for method probes.
const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// Tower primal P1 readiness methods to probe.
const TOWER_PROBES: &[(&str, &[&str])] = &[
    (
        "beardog",
        &["auth.public_key", "auth.trusted_issuers", "btsp.capabilities"],
    ),
    (
        "songbird",
        &["mesh.peers", "mesh.capabilities_announce", "mesh.init"],
    ),
    (
        "skunkbat",
        &["method_gate.status", "threat.report", "auth.check"],
    ),
];

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
pub fn probe_tower_status() -> TowerStatus {
    let mut primals = Vec::new();

    for (slug, methods) in TOWER_PROBES {
        let env_var = format!("{}_SOCKET", slug.to_uppercase());
        let socket = discovery::probe_socket(slug, &env_var);

        let method_results: Vec<MethodProbe> = socket.as_ref().map_or_else(
            || {
                methods
                    .iter()
                    .map(|method| MethodProbe {
                        method: (*method).to_string(),
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
            name: (*slug).to_string(),
            socket_path: socket,
            methods: method_results,
        });
    }

    TowerStatus { primals }
}

/// Probe a single JSON-RPC method on a socket, returning availability.
fn probe_single_method(socket_path: &str, method: &str) -> MethodProbe {
    let Ok(stream) = std::os::unix::net::UnixStream::connect(socket_path) else {
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some("connect failed".into()),
        };
    };

    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok();
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok();

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": {},
        "id": 1
    });

    let Ok(mut payload) = serde_json::to_string(&request) else {
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some("encode failed".into()),
        };
    };
    payload.push('\n');

    let mut reader = BufReader::new(stream);

    if reader.get_mut().write_all(payload.as_bytes()).is_err() {
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some("write failed".into()),
        };
    }
    let _ = reader.get_mut().flush();

    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some("read timeout".into()),
        };
    }

    let Ok(resp) = serde_json::from_str::<Value>(line.trim()) else {
        return MethodProbe {
            method: method.to_string(),
            available: false,
            response_summary: Some("invalid JSON".into()),
        };
    };

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
                available: code != -32601,
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
        let socket_display = primal
            .socket_path
            .as_deref()
            .unwrap_or("NOT FOUND");
        println!("  {} → {socket_display}", primal.name);

        for m in &primal.methods {
            let icon = if m.available { "✅" } else { "❌" };
            let summary = m
                .response_summary
                .as_deref()
                .unwrap_or("");
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
    fn tower_probes_cover_all_three_primals() {
        assert_eq!(TOWER_PROBES.len(), 3);
        let slugs: Vec<&str> = TOWER_PROBES.iter().map(|(s, _)| *s).collect();
        assert!(slugs.contains(&"beardog"));
        assert!(slugs.contains(&"songbird"));
        assert!(slugs.contains(&"skunkbat"));
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
