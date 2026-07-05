// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared NDJSON JSON-RPC 2.0 client — transport-agnostic IPC for all primals.
//!
//! This module consolidates the JSON-RPC client logic previously duplicated
//! across `cas_push`, `nucleus`, `petaltongue`, and `tower`. All primal IPC
//! flows through `send_rpc`, which enforces:
//!
//! - NDJSON framing (newline-delimited JSON)
//! - Response `id` correlation with the request (JSON-RPC 2.0 §5)
//! - Typed error extraction
//!
//! Transport connection and riboCipher signalling remain in `cas_push`
//! since that module owns `TransportEndpoint` and `connect_transport`.

use crate::error::Error;
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};

use crate::cas_push::ReadWrite;

/// Send a JSON-RPC 2.0 request over an NDJSON stream and read the response.
///
/// Validates that the response `id` matches the request `id` per JSON-RPC 2.0
/// §5. Returns the full response `Value` (caller extracts `result` or `error`).
pub fn send_rpc(
    stream: &mut BufReader<Box<dyn ReadWrite>>,
    request: &Value,
) -> Result<Value, Error> {
    let mut payload =
        serde_json::to_string(request).map_err(|e| Error::Config(format!("JSON encode: {e}")))?;
    payload.push('\n');

    let writer = stream.get_mut();
    writer
        .write_all(payload.as_bytes())
        .map_err(|e| Error::Config(format!("transport write: {e}")))?;
    writer
        .flush()
        .map_err(|e| Error::Config(format!("transport flush: {e}")))?;

    let mut line = String::new();
    stream
        .read_line(&mut line)
        .map_err(|e| Error::Config(format!("transport read: {e}")))?;

    let response: Value = serde_json::from_str(line.trim())
        .map_err(|e| Error::Config(format!("JSON decode response: {e}")))?;

    validate_response_id(request, &response)?;

    Ok(response)
}

/// Validate that the response `id` matches the request `id` (JSON-RPC 2.0 §5).
///
/// Tolerates missing `id` in the response (some legacy primals omit it),
/// but rejects mismatched IDs.
fn validate_response_id(request: &Value, response: &Value) -> Result<(), Error> {
    let req_id = request.get("id");
    let resp_id = response.get("id");

    match (req_id, resp_id) {
        (Some(req), Some(resp)) if req != resp => Err(Error::Config(format!(
            "JSON-RPC id mismatch: sent {req}, got {resp}"
        ))),
        _ => Ok(()),
    }
}

/// Extract the error message from a JSON-RPC error response, if present.
pub fn extract_error_message(response: &Value) -> Option<String> {
    let err = response.get("error")?;
    let code = err.get("code").and_then(Value::as_i64).unwrap_or(0);
    let message = err
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    Some(format!("[{code}] {message}"))
}

/// Check if a JSON-RPC error is "method not found" (-32601).
pub fn is_method_not_found(response: &Value) -> bool {
    response
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(Value::as_i64)
        .is_some_and(|code| code == -32601)
}

/// Probe a primal's health using the ecosystem standard method.
///
/// Tries `health.liveness` first (ecosystem v2.1+), falls back to
/// `health.ping` for legacy primals. Returns the raw response `Value`.
pub fn probe_health(
    stream: &mut BufReader<Box<dyn ReadWrite>>,
    request_id: u64,
) -> Result<Value, Error> {
    let liveness_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health.liveness",
        "params": {},
        "id": request_id
    });

    let resp = send_rpc(stream, &liveness_req)?;

    if is_method_not_found(&resp) {
        let ping_req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "health.ping",
            "params": {},
            "id": request_id
        });
        return send_rpc(stream, &ping_req);
    }

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct MockStream {
        read_buf: Cursor<Vec<u8>>,
        write_buf: Vec<u8>,
    }

    impl MockStream {
        fn with_response(json: &Value) -> Self {
            let mut data = serde_json::to_string(json).unwrap();
            data.push('\n');
            Self {
                read_buf: Cursor::new(data.into_bytes()),
                write_buf: Vec::new(),
            }
        }

        fn with_responses(responses: &[Value]) -> Self {
            let mut data = String::new();
            for r in responses {
                data.push_str(&serde_json::to_string(r).unwrap());
                data.push('\n');
            }
            Self {
                read_buf: Cursor::new(data.into_bytes()),
                write_buf: Vec::new(),
            }
        }
    }

    impl std::io::Read for MockStream {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.read_buf.read(buf)
        }
    }

    impl std::io::Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.write_buf.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn send_rpc_roundtrip() {
        let response = serde_json::json!({"jsonrpc": "2.0", "id": 42, "result": {"ok": true}});
        let mock = MockStream::with_response(&response);
        let mut reader = BufReader::new(Box::new(mock) as Box<dyn ReadWrite>);

        let request = serde_json::json!({"jsonrpc": "2.0", "method": "test", "id": 42});
        let resp = send_rpc(&mut reader, &request).unwrap();
        assert_eq!(resp["result"]["ok"], true);
    }

    #[test]
    fn send_rpc_id_mismatch_errors() {
        let response = serde_json::json!({"jsonrpc": "2.0", "id": 99, "result": {}});
        let mock = MockStream::with_response(&response);
        let mut reader = BufReader::new(Box::new(mock) as Box<dyn ReadWrite>);

        let request = serde_json::json!({"jsonrpc": "2.0", "method": "test", "id": 1});
        let result = send_rpc(&mut reader, &request);
        assert!(result.is_err());
    }

    #[test]
    fn probe_health_liveness_success() {
        let response = serde_json::json!({
            "jsonrpc": "2.0", "id": 1,
            "result": {"status": "alive", "primal": "testPrimal", "version": "0.1.0"}
        });
        let mock = MockStream::with_response(&response);
        let mut reader = BufReader::new(Box::new(mock) as Box<dyn ReadWrite>);

        let resp = probe_health(&mut reader, 1).unwrap();
        assert_eq!(resp["result"]["status"], "alive");
    }

    #[test]
    fn probe_health_fallback_to_ping() {
        let not_found = serde_json::json!({
            "jsonrpc": "2.0", "id": 1,
            "error": {"code": -32601, "message": "Method not found"}
        });
        let ping_ok = serde_json::json!({
            "jsonrpc": "2.0", "id": 1,
            "result": {"status": "pong"}
        });
        let mock = MockStream::with_responses(&[not_found, ping_ok]);
        let mut reader = BufReader::new(Box::new(mock) as Box<dyn ReadWrite>);

        let resp = probe_health(&mut reader, 1).unwrap();
        assert_eq!(resp["result"]["status"], "pong");
    }

    #[test]
    fn validate_response_id_matching() {
        let req = serde_json::json!({"id": 1});
        let resp = serde_json::json!({"id": 1, "result": {}});
        assert!(validate_response_id(&req, &resp).is_ok());
    }

    #[test]
    fn validate_response_id_mismatch() {
        let req = serde_json::json!({"id": 1});
        let resp = serde_json::json!({"id": 2, "result": {}});
        assert!(validate_response_id(&req, &resp).is_err());
    }

    #[test]
    fn validate_response_id_missing_tolerant() {
        let req = serde_json::json!({"id": 1});
        let resp = serde_json::json!({"result": {}});
        assert!(validate_response_id(&req, &resp).is_ok());
    }

    #[test]
    fn extract_error_message_present() {
        let resp = serde_json::json!({"error": {"code": -32601, "message": "Method not found"}});
        let msg = extract_error_message(&resp).unwrap();
        assert!(msg.contains("-32601"));
        assert!(msg.contains("Method not found"));
    }

    #[test]
    fn extract_error_message_absent() {
        let resp = serde_json::json!({"result": {}});
        assert!(extract_error_message(&resp).is_none());
    }

    #[test]
    fn is_method_not_found_true() {
        let resp = serde_json::json!({"error": {"code": -32601, "message": "not found"}});
        assert!(is_method_not_found(&resp));
    }

    #[test]
    fn is_method_not_found_false_on_other_error() {
        let resp = serde_json::json!({"error": {"code": -32603, "message": "internal"}});
        assert!(!is_method_not_found(&resp));
    }

    #[test]
    fn is_method_not_found_false_on_success() {
        let resp = serde_json::json!({"result": {"status": "alive"}});
        assert!(!is_method_not_found(&resp));
    }
}
