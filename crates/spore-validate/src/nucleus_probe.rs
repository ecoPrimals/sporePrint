// SPDX-License-Identifier: AGPL-3.0-or-later

//! Live socket probing for NUCLEUS profile validation.
//!
//! Connects to primal sockets via UDS and validates health contract
//! compliance (`{status, primal, version}`) and riboCipher mito-beacon
//! signal acceptance.

use crate::ipc::RIBOCIPHER_MITO_CLEAR;
use crate::nucleus::{HealthContract, ProbeResult};
use crate::paths::PROBE_TIMEOUT;
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

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
pub fn probe_socket_health(socket_path: &str) -> ProbeResult {
    let start = Instant::now();

    let mut reader = match crate::ipc::connect_uds(socket_path, PROBE_TIMEOUT) {
        Ok(r) => r,
        Err(e) => return probe_failed(start, format!("{e}")),
    };

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

/// Probe whether a primal accepts the `riboCipher` mito-beacon signal prefix.
///
/// Connects to the socket, writes `0xEC 0x01` followed by a `health.liveness`
/// JSON-RPC request, and checks whether the primal responds with valid JSON-RPC
/// rather than closing the connection or returning garbage.
pub fn probe_ribocipher_acceptance(socket_path: &str) -> bool {
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

    serde_json::from_str::<Value>(line.trim()).is_ok() && !line.trim().is_empty()
}
