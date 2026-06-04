// SPDX-License-Identifier: AGPL-3.0-or-later

//! Content-Addressed Storage (CAS) manifest generation for sporePrint build output.
//!
//! Walks the Zola `public/` directory, computes a BLAKE3 hash for every file,
//! and emits a deterministic manifest suitable for `NestGate` CAS ingest.
//!
//! The build hash is a BLAKE3 hash of sorted per-file hashes, making it a
//! deterministic fingerprint of the entire build output.

use crate::error::Error;
use std::collections::BTreeMap;
use std::path::Path;
use walkdir::WalkDir;

/// A single file in the CAS manifest.
#[derive(Debug, serde::Serialize)]
pub struct CasEntry {
    pub hash: String,
    pub size: u64,
    pub content_type: String,
}

/// Complete CAS manifest for a build.
#[derive(Debug, serde::Serialize)]
pub struct CasManifest {
    pub build_id: String,
    pub build_hash: String,
    pub page_count: usize,
    pub total_bytes: u64,
    pub files: BTreeMap<String, CasEntry>,
}

/// Generate a CAS manifest by hashing every file in `public_dir`.
pub fn generate_manifest(public_dir: &Path) -> CasManifest {
    let mut files = BTreeMap::new();
    let mut total_bytes: u64 = 0;

    for entry in WalkDir::new(public_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let Ok(contents) = std::fs::read(path) else {
            continue;
        };

        let hash = blake3::hash(&contents).to_hex().to_string();
        let size = contents.len() as u64;
        let rel_path = crate::paths::rel_to(path, public_dir)
            .to_string_lossy()
            .replace('\\', "/");

        let content_type = infer_content_type(&rel_path);
        total_bytes += size;

        files.insert(
            rel_path,
            CasEntry {
                hash: format!("blake3:{hash}"),
                size,
                content_type,
            },
        );
    }

    let build_hash = compute_build_hash(&files);
    let page_count = files
        .keys()
        .filter(|k| k.to_ascii_lowercase().ends_with(".html"))
        .count();

    CasManifest {
        build_id: crate::time::now_iso8601(),
        build_hash: format!("blake3:{build_hash}"),
        page_count,
        total_bytes,
        files,
    }
}

/// Compute a deterministic build hash from sorted file hashes.
fn compute_build_hash(files: &BTreeMap<String, CasEntry>) -> String {
    let mut hasher = blake3::Hasher::new();
    for (path, entry) in files {
        hasher.update(path.as_bytes());
        hasher.update(b":");
        hasher.update(entry.hash.as_bytes());
        hasher.update(b"\n");
    }
    hasher.finalize().to_hex().to_string()
}

/// Write the CAS manifest to a JSON file.
pub fn emit_manifest(manifest: &CasManifest, output_path: &Path) -> Result<(), Error> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| Error::io(output_path, e))?;
    }
    let json = serde_json::to_string_pretty(manifest)
        .map_err(|e| Error::Config(format!("JSON serialization failed: {e}")))?;
    std::fs::write(output_path, json).map_err(|e| Error::io(output_path, e))?;
    Ok(())
}

/// Infer MIME content type from file extension.
fn infer_content_type(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        "ico" => "image/x-icon",
        "txt" => "text/plain",
        "toml" => "application/toml",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infer_types_correctly() {
        assert_eq!(infer_content_type("index.html"), "text/html");
        assert_eq!(infer_content_type("css/main.css"), "text/css");
        assert_eq!(infer_content_type("js/app.js"), "application/javascript");
        assert_eq!(infer_content_type("data.json"), "application/json");
        assert_eq!(infer_content_type("wasm/module.wasm"), "application/wasm");
        assert_eq!(infer_content_type("unknown"), "application/octet-stream");
    }

    #[test]
    fn empty_dir_produces_empty_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = generate_manifest(dir.path());
        assert_eq!(manifest.page_count, 0);
        assert_eq!(manifest.total_bytes, 0);
        assert!(manifest.files.is_empty());
    }

    #[test]
    fn manifest_is_deterministic() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.html"), "hello").unwrap();
        std::fs::write(dir.path().join("b.css"), "body{}").unwrap();

        let m1 = generate_manifest(dir.path());
        let m2 = generate_manifest(dir.path());
        assert_eq!(m1.build_hash, m2.build_hash);
        assert_eq!(m1.page_count, 1);
        assert_eq!(m1.files.len(), 2);
    }

    #[test]
    fn content_change_changes_hash() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("page.html"), "v1").unwrap();
        let m1 = generate_manifest(dir.path());

        std::fs::write(dir.path().join("page.html"), "v2").unwrap();
        let m2 = generate_manifest(dir.path());

        assert_ne!(m1.build_hash, m2.build_hash);
        assert_ne!(
            m1.files["page.html"].hash,
            m2.files["page.html"].hash
        );
    }
}
