// SPDX-License-Identifier: AGPL-3.0-or-later

//! CAS push — send build artifacts to `NestGate` content-addressed storage.
//!
//! Reads a `build-manifest.json` (produced by `cas-manifest --emit`), then
//! pushes each file's content to `NestGate`'s `content.put` via JSON-RPC 2.0
//! (newline-delimited). Files already stored are skipped via `content.exists`
//! for efficient dedup.
//!
//! ## Transport
//!
//! Transport is injected, not self-bound. The `connect_transport` function
//! resolves a `TransportEndpoint` to a stream. Supports UDS and TCP;
//! when Songbird ships `ipc.resolve` with transport-qualified endpoints,
//! mesh relay transport can be added without changing push logic.
//!
//! ## riboCipher
//!
//! When `SPOREPRINT_RIBOCIPHER=1`, the transport layer sends the Tier 1
//! clear signal (`0xEC 0x01`) immediately after connection, declaring
//! NDJSON JSON-RPC intent per the ecosystem riboCipher standard.
//! Required for Wave 113+ servers that enforce REJECT on unsignalled connections.
//!
//! ## Discovery
//!
//! `NestGate` socket path is discovered via `discovery::probe_socket`:
//! 1. `NESTGATE_SOCKET` env var (explicit override)
//! 2. `$BIOMEOS_SOCKET_DIR/nestgate.sock` (ecosystem standard)
//! 3. `$XDG_RUNTIME_DIR/biomeos/nestgate.sock` (XDG fallback)

use crate::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

/// A transport-qualified endpoint for primal IPC.
///
/// Primals do not choose their transport — the launcher/Songbird decides.
/// Wire format matches ecosystem canonical: `#[serde(tag = "transport")]`.
///
/// Deserialized from `TRANSPORT_ENDPOINT` env var or Songbird `ipc.resolve` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "transport")]
pub enum TransportEndpoint {
    /// Unix domain socket (single-machine deployment).
    #[serde(rename = "uds")]
    Uds { path: String },
    /// TCP socket (LAN cross-gate or explicit binding).
    #[serde(rename = "tcp")]
    Tcp { host: String, port: u16 },
    /// Mesh relay via Songbird (cross-network, transport-agnostic).
    #[serde(rename = "mesh_relay")]
    MeshRelay { peer_id: String, capability: String },
}

use crate::paths::{TRANSPORT_CONNECT_TIMEOUT, TRANSPORT_IO_TIMEOUT};

use crate::ipc::{ribocipher_enabled, send_ribocipher_signal};

/// Connect to a `NestGate` instance via the specified transport.
///
/// Returns a boxed stream implementing `Read + Write`. The caller never
/// needs to know the underlying transport mechanism. All transports use
/// bounded timeouts to avoid indefinite hangs on WAN links.
///
/// When `SPOREPRINT_RIBOCIPHER=1`, sends the Tier 1 clear signal (`0xEC 0x01`)
/// immediately after connection — required by servers enforcing Wave 113+
/// riboCipher REJECT policy.
pub fn connect_transport(endpoint: &TransportEndpoint) -> Result<Box<dyn ReadWrite>, Error> {
    let mut stream: Box<dyn ReadWrite> = match endpoint {
        TransportEndpoint::Uds { path } => {
            let s = std::os::unix::net::UnixStream::connect(path).map_err(|e| {
                Error::Config(format!(
                    "failed to connect to NestGate via UDS at {path}: {e}"
                ))
            })?;
            s.set_write_timeout(Some(TRANSPORT_IO_TIMEOUT)).ok();
            s.set_read_timeout(Some(TRANSPORT_IO_TIMEOUT)).ok();
            Box::new(s)
        }
        TransportEndpoint::Tcp { host, port } => {
            let addr_str = format!("{host}:{port}");
            let addr: std::net::SocketAddr = addr_str
                .parse()
                .map_err(|e| Error::Config(format!("invalid TCP address {addr_str}: {e}")))?;
            let s = std::net::TcpStream::connect_timeout(&addr, TRANSPORT_CONNECT_TIMEOUT)
                .map_err(|e| {
                    Error::Config(format!(
                        "failed to connect to NestGate via TCP at {addr_str}: {e}"
                    ))
                })?;
            s.set_write_timeout(Some(TRANSPORT_IO_TIMEOUT)).ok();
            s.set_read_timeout(Some(TRANSPORT_IO_TIMEOUT)).ok();
            Box::new(s)
        }
        TransportEndpoint::MeshRelay {
            peer_id,
            capability,
        } => {
            return Err(Error::Config(format!(
                "mesh_relay transport not yet implemented (peer={peer_id}, cap={capability}). \
                 Requires Songbird ipc.resolve Phase 2 M1."
            )));
        }
    };

    if ribocipher_enabled() {
        send_ribocipher_signal(stream.as_mut())?;
    }

    Ok(stream)
}

pub use crate::ipc::ReadWrite;

/// Deserialized CAS manifest (matches `cas::CasManifest` serialization).
#[derive(Debug, Deserialize)]
pub struct StoredManifest {
    pub build_id: String,
    pub build_hash: String,
    pub page_count: usize,
    pub total_bytes: u64,
    pub files: BTreeMap<String, StoredEntry>,
}

#[derive(Debug, Deserialize)]
pub struct StoredEntry {
    pub hash: String,
    pub size: u64,
    pub content_type: String,
}

impl From<crate::cas::CasEntry> for StoredEntry {
    fn from(e: crate::cas::CasEntry) -> Self {
        Self {
            hash: e.hash,
            size: e.size,
            content_type: e.content_type,
        }
    }
}

/// Result of a push operation.
#[derive(Debug)]
pub struct PushResult {
    pub stored: u64,
    pub deduplicated: u64,
    pub errors: u64,
    pub error_messages: Vec<String>,
    pub total_bytes_transferred: u64,
    pub elapsed_ms: u64,
}

/// Read a stored CAS manifest from disk.
pub fn read_manifest(manifest_path: &Path) -> Result<StoredManifest, Error> {
    let content =
        std::fs::read_to_string(manifest_path).map_err(|e| Error::io(manifest_path, e))?;
    let manifest: StoredManifest = serde_json::from_str(&content).map_err(|e| {
        Error::Config(format!(
            "failed to parse CAS manifest at {}: {e}",
            manifest_path.display()
        ))
    })?;
    Ok(manifest)
}

/// Outcome of pushing a single file to `NestGate`.
enum PushFileOutcome {
    Stored { bytes: u64 },
    Deduplicated,
    Error(String),
}

fn encode_file_payload(
    file_path: &Path,
    relative_key: &str,
    content_type: &str,
    metadata: &Value,
) -> Result<Value, Error> {
    let contents = std::fs::read(file_path).map_err(|e| {
        Error::Config(format!(
            "cannot read {} ({relative_key}): {e}",
            file_path.display()
        ))
    })?;
    let data_b64 = STANDARD.encode(&contents);
    Ok(json!({
        "data": data_b64,
        "content_type": content_type,
        "source": "sporePrint",
        "pipeline": "zola-build",
        "stored_by": "spore-validate cas-push",
        "metadata": metadata,
    }))
}

/// Push a single file entry to `NestGate`, returning the outcome.
fn push_single_file(
    reader: &mut BufReader<Box<dyn ReadWrite>>,
    request_id: &mut u64,
    rel_path: &str,
    entry: &StoredEntry,
    public_dir: &Path,
    build_hash: &str,
    build_id: &str,
) -> PushFileOutcome {
    *request_id += 1;

    let hash_hex = entry.hash.strip_prefix("blake3:").unwrap_or(&entry.hash);

    let exists_req = json!({
        "jsonrpc": "2.0",
        "method": "content.exists",
        "params": { "hash": hash_hex },
        "id": *request_id
    });

    if let Ok(resp) = send_rpc(reader, &exists_req) {
        if resp["result"]["exists"].as_bool() == Some(true) {
            return PushFileOutcome::Deduplicated;
        }
    }

    let file_path = public_dir.join(rel_path);
    let metadata = json!({
        "path": rel_path,
        "build_hash": build_hash,
        "build_id": build_id,
    });
    *request_id += 1;

    let Ok(params) = encode_file_payload(&file_path, rel_path, &entry.content_type, &metadata)
    else {
        return PushFileOutcome::Error(format!("WARN: cannot read {rel_path}, skipping"));
    };

    let put_req = json!({
        "jsonrpc": "2.0",
        "method": "content.put",
        "params": params,
        "id": *request_id
    });

    match send_rpc(reader, &put_req) {
        Ok(resp) => {
            if resp.get("error").is_some() && !resp["error"].is_null() {
                PushFileOutcome::Error(format!(
                    "ERROR: content.put failed for {rel_path}: {}",
                    resp["error"]
                ))
            } else if resp["result"]["deduplicated"].as_bool() == Some(true) {
                PushFileOutcome::Deduplicated
            } else {
                PushFileOutcome::Stored { bytes: entry.size }
            }
        }
        Err(e) => PushFileOutcome::Error(format!("ERROR: RPC failed for {rel_path}: {e}")),
    }
}

/// Push all files from a CAS manifest to `NestGate`.
///
/// Accepts a `TransportEndpoint` — the transport is injected by the caller,
/// not chosen by the push logic. Use `discovery::resolve_primal_endpoint()`
/// for discovery-based connection, or accept a Songbird-resolved endpoint directly.
pub fn push_manifest(
    manifest: &StoredManifest,
    public_dir: &Path,
    endpoint: &TransportEndpoint,
) -> Result<PushResult, Error> {
    let t0 = Instant::now();
    let mut stored: u64 = 0;
    let mut deduplicated: u64 = 0;
    let mut errors: u64 = 0;
    let mut error_messages = Vec::new();
    let mut bytes_transferred: u64 = 0;

    let stream = connect_transport(endpoint)?;
    let mut reader = BufReader::new(stream);
    let mut request_id: u64 = 0;

    request_id += 1;
    let announce_req = crate::discovery::announce_request(request_id);
    let _ = send_rpc(&mut reader, &announce_req);

    for (rel_path, entry) in &manifest.files {
        match push_single_file(
            &mut reader,
            &mut request_id,
            rel_path,
            entry,
            public_dir,
            &manifest.build_hash,
            &manifest.build_id,
        ) {
            PushFileOutcome::Stored { bytes } => {
                stored += 1;
                bytes_transferred += bytes;
            }
            PushFileOutcome::Deduplicated => deduplicated += 1,
            PushFileOutcome::Error(msg) => {
                errors += 1;
                error_messages.push(msg);
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    let elapsed = t0.elapsed().as_millis() as u64;

    Ok(PushResult {
        stored,
        deduplicated,
        errors,
        error_messages,
        total_bytes_transferred: bytes_transferred,
        elapsed_ms: elapsed,
    })
}

/// Send a JSON-RPC request and read the newline-delimited response.
///
/// Delegates to the shared `ipc::send_rpc` which also validates response ID
/// correlation per JSON-RPC 2.0 §5.
fn send_rpc(stream: &mut BufReader<Box<dyn ReadWrite>>, request: &Value) -> Result<Value, Error> {
    crate::ipc::send_rpc(stream, request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_manifest_parses_valid_json() {
        let dir = tempfile::tempdir().unwrap();
        let manifest_json = r#"{
            "build_id": "2026-06-03T10:00:00Z",
            "build_hash": "blake3:abc123",
            "page_count": 2,
            "total_bytes": 1000,
            "files": {
                "index.html": {
                    "hash": "blake3:def456",
                    "size": 500,
                    "content_type": "text/html"
                },
                "css/main.css": {
                    "hash": "blake3:789abc",
                    "size": 500,
                    "content_type": "text/css"
                }
            }
        }"#;
        let path = dir.path().join("build-manifest.json");
        std::fs::write(&path, manifest_json).unwrap();

        let manifest = read_manifest(&path).unwrap();
        assert_eq!(manifest.page_count, 2);
        assert_eq!(manifest.total_bytes, 1000);
        assert_eq!(manifest.files.len(), 2);
        assert_eq!(manifest.files["index.html"].content_type, "text/html");
    }

    #[test]
    fn push_manifest_fails_with_bad_socket() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("index.html"), "<html></html>").unwrap();

        let manifest = StoredManifest {
            build_id: "test".into(),
            build_hash: "blake3:abc".into(),
            page_count: 1,
            total_bytes: 13,
            files: BTreeMap::from([(
                "index.html".into(),
                StoredEntry {
                    hash: "blake3:def456".into(),
                    size: 13,
                    content_type: "text/html".into(),
                },
            )]),
        };

        let endpoint = TransportEndpoint::Uds {
            path: "/tmp/nonexistent-nestgate-test.sock".into(),
        };
        let result = push_manifest(&manifest, dir.path(), &endpoint);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("failed to connect"));
    }

    #[test]
    fn transport_endpoint_debug_format() {
        let ep = TransportEndpoint::Uds {
            path: "/run/nestgate.sock".into(),
        };
        let debug = format!("{ep:?}");
        assert!(debug.contains("Uds"));
        assert!(debug.contains("/run/nestgate.sock"));
    }

    #[test]
    fn transport_endpoint_serde_uds_roundtrip() {
        let ep = TransportEndpoint::Uds {
            path: "/run/biomeos/nestgate.sock".into(),
        };
        let json = serde_json::to_string(&ep).unwrap();
        assert!(json.contains(r#""transport":"uds""#));
        let decoded: TransportEndpoint = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded, TransportEndpoint::Uds { path } if path.contains("nestgate")));
    }

    #[test]
    fn transport_endpoint_serde_tcp_roundtrip() {
        let ep = TransportEndpoint::Tcp {
            host: "192.168.1.173".into(),
            port: 9100,
        };
        let json = serde_json::to_string(&ep).unwrap();
        assert!(json.contains(r#""transport":"tcp""#));
        assert!(json.contains(r#""port":9100"#));
        let decoded: TransportEndpoint = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded, TransportEndpoint::Tcp { port: 9100, .. }));
    }

    #[test]
    fn transport_endpoint_serde_mesh_relay() {
        let json = r#"{"transport":"mesh_relay","peer_id":"strandgate","capability":"cas"}"#;
        let ep: TransportEndpoint = serde_json::from_str(json).unwrap();
        assert!(
            matches!(ep, TransportEndpoint::MeshRelay { ref peer_id, .. } if peer_id == "strandgate")
        );
    }

    #[test]
    fn connect_transport_tcp_fails_on_refused() {
        let ep = TransportEndpoint::Tcp {
            host: "127.0.0.1".into(),
            port: 1,
        };
        let Err(e) = connect_transport(&ep) else {
            panic!("expected error");
        };
        assert!(e.to_string().contains("TCP"));
    }

    #[test]
    fn connect_transport_mesh_relay_not_implemented() {
        let ep = TransportEndpoint::MeshRelay {
            peer_id: "test-peer".into(),
            capability: "cas".into(),
        };
        let Err(e) = connect_transport(&ep) else {
            panic!("expected error");
        };
        assert!(e.to_string().contains("not yet implemented"));
    }

    #[test]
    fn ribocipher_signal_writes_correct_bytes() {
        let mut buf = Vec::new();
        send_ribocipher_signal(&mut buf).unwrap();
        assert_eq!(buf, [0xEC, 0x01]);
    }

    #[test]
    fn ribocipher_disabled_by_default() {
        assert!(!ribocipher_enabled());
    }
}
