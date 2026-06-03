// SPDX-License-Identifier: AGPL-3.0-or-later

//! CAS push — send build artifacts to `NestGate` content-addressed storage.
//!
//! Reads a `build-manifest.json` (produced by `cas-manifest --emit`), then
//! pushes each file's content to `NestGate`'s `content.put` over UNIX socket
//! (JSON-RPC 2.0, newline-delimited). Files already stored are skipped via
//! `content.exists` for efficient dedup.
//!
//! ## Discovery
//!
//! `NestGate` socket path is discovered via (in priority order):
//! 1. `NESTGATE_SOCKET` env var
//! 2. `$XDG_RUNTIME_DIR/biomeos/nestgate.sock`
//! 3. `/tmp/nestgate-standalone-{hostname}.sock`

use crate::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Instant;

/// Deserialized CAS manifest (matches `cas::CasManifest` serialization).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

/// Result of a push operation.
#[derive(Debug)]
pub struct PushResult {
    pub stored: u64,
    pub deduplicated: u64,
    pub errors: u64,
    pub total_bytes_transferred: u64,
    pub elapsed_ms: u64,
}

/// Discover the `NestGate` socket path from environment.
pub fn discover_socket() -> Result<String, Error> {
    if let Ok(path) = std::env::var("NESTGATE_SOCKET") {
        if Path::new(&path).exists() {
            return Ok(path);
        }
        return Err(Error::Config(format!(
            "NESTGATE_SOCKET set to {path} but socket does not exist"
        )));
    }

    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let candidate = format!("{xdg}/biomeos/nestgate.sock");
        if Path::new(&candidate).exists() {
            return Ok(candidate);
        }
        let standalone = format!("{xdg}/biomeos/nestgate-standalone.sock");
        if Path::new(&standalone).exists() {
            return Ok(standalone);
        }
    }

    let hostname = std::fs::read_to_string("/etc/hostname")
        .map_or_else(|_| "unknown".into(), |s| s.trim().to_string());
    let fallback = format!("/tmp/nestgate-standalone-{hostname}.sock");
    if Path::new(&fallback).exists() {
        return Ok(fallback);
    }

    Err(Error::Config(
        "NestGate socket not found. Set NESTGATE_SOCKET or ensure NestGate is running.".into(),
    ))
}

/// Read a stored CAS manifest from disk.
pub fn read_manifest(manifest_path: &Path) -> Result<StoredManifest, Error> {
    let content = std::fs::read_to_string(manifest_path).map_err(|e| Error::io(manifest_path, e))?;
    let manifest: StoredManifest =
        serde_json::from_str(&content).map_err(|e| Error::Config(format!(
            "failed to parse CAS manifest at {}: {e}",
            manifest_path.display()
        )))?;
    Ok(manifest)
}

/// Push all files from a CAS manifest to `NestGate`.
pub fn push_manifest(
    manifest: &StoredManifest,
    public_dir: &Path,
    socket_path: &str,
) -> Result<PushResult, Error> {
    let t0 = Instant::now();
    let mut stored: u64 = 0;
    let mut deduplicated: u64 = 0;
    let mut errors: u64 = 0;
    let mut bytes_transferred: u64 = 0;

    let stream = UnixStream::connect(socket_path).map_err(|e| {
        Error::Config(format!("failed to connect to NestGate at {socket_path}: {e}"))
    })?;
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(30)))
        .ok();

    let mut writer = stream.try_clone().map_err(|e| {
        Error::Config(format!("failed to clone socket: {e}"))
    })?;
    let mut reader = BufReader::new(stream);

    let mut request_id: u64 = 0;

    for (rel_path, entry) in &manifest.files {
        request_id += 1;

        let hash_hex = entry
            .hash
            .strip_prefix("blake3:")
            .unwrap_or(&entry.hash);

        // Check if content already exists (dedup optimization)
        let exists_req = json!({
            "jsonrpc": "2.0",
            "method": "content.exists",
            "params": { "hash": hash_hex },
            "id": request_id
        });

        if let Ok(resp) = send_rpc(&mut writer, &mut reader, &exists_req) {
            if resp["result"]["exists"].as_bool() == Some(true) {
                deduplicated += 1;
                continue;
            }
        }

        let file_path = public_dir.join(rel_path);
        let Ok(contents) = std::fs::read(&file_path) else {
            eprintln!("  WARN: cannot read {rel_path}, skipping");
            errors += 1;
            continue;
        };

        let data_b64 = STANDARD.encode(&contents);
        request_id += 1;

        let put_req = json!({
            "jsonrpc": "2.0",
            "method": "content.put",
            "params": {
                "data": data_b64,
                "content_type": entry.content_type,
                "source": "sporePrint",
                "pipeline": "zola-build",
                "stored_by": "spore-validate cas-push",
                "metadata": {
                    "path": rel_path,
                    "build_hash": &manifest.build_hash,
                    "build_id": &manifest.build_id,
                }
            },
            "id": request_id
        });

        match send_rpc(&mut writer, &mut reader, &put_req) {
            Ok(resp) => {
                if resp.get("error").is_some() && !resp["error"].is_null() {
                    eprintln!(
                        "  ERROR: content.put failed for {rel_path}: {}",
                        resp["error"]
                    );
                    errors += 1;
                } else if resp["result"]["deduplicated"].as_bool() == Some(true) {
                    deduplicated += 1;
                } else {
                    stored += 1;
                    bytes_transferred += entry.size;
                }
            }
            Err(e) => {
                eprintln!("  ERROR: RPC failed for {rel_path}: {e}");
                errors += 1;
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    let elapsed = t0.elapsed().as_millis() as u64;

    Ok(PushResult {
        stored,
        deduplicated,
        errors,
        total_bytes_transferred: bytes_transferred,
        elapsed_ms: elapsed,
    })
}

/// Send a JSON-RPC request and read the newline-delimited response.
fn send_rpc(
    writer: &mut UnixStream,
    reader: &mut BufReader<UnixStream>,
    request: &Value,
) -> Result<Value, Error> {
    let mut payload = serde_json::to_string(request)
        .map_err(|e| Error::Config(format!("JSON encode: {e}")))?;
    payload.push('\n');

    writer
        .write_all(payload.as_bytes())
        .map_err(|e| Error::Config(format!("socket write: {e}")))?;
    writer
        .flush()
        .map_err(|e| Error::Config(format!("socket flush: {e}")))?;

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .map_err(|e| Error::Config(format!("socket read: {e}")))?;

    let response: Value = serde_json::from_str(line.trim())
        .map_err(|e| Error::Config(format!("JSON decode response: {e}")))?;

    Ok(response)
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

        let result = push_manifest(&manifest, dir.path(), "/tmp/nonexistent-nestgate-test.sock");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("failed to connect"));
    }
}
