// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical path constants, timeouts, and helpers for sporePrint.
//!
//! Centralizes all path literals and network constants so the layout and
//! transport parameters can evolve without grep-hunting.

use crate::error::Error;
use std::path::{Path, PathBuf};
use std::time::Duration;
use walkdir::WalkDir;

/// IPC timeout for health / method probes (fast operations).
/// Overridable via `SPOREPRINT_PROBE_TIMEOUT_SECS`.
pub fn probe_timeout() -> Duration {
    duration_from_env("SPOREPRINT_PROBE_TIMEOUT_SECS", 3)
}

/// Transport connect timeout for CAS push and HTTP fetch.
/// Overridable via `SPOREPRINT_CONNECT_TIMEOUT_SECS`.
pub fn transport_connect_timeout() -> Duration {
    duration_from_env("SPOREPRINT_CONNECT_TIMEOUT_SECS", 15)
}

/// Transport I/O (read/write) timeout for CAS push and HTTP fetch.
/// Overridable via `SPOREPRINT_IO_TIMEOUT_SECS`.
pub fn transport_io_timeout() -> Duration {
    duration_from_env("SPOREPRINT_IO_TIMEOUT_SECS", 30)
}

fn duration_from_env(var: &str, default_secs: u64) -> Duration {
    std::env::var(var)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map_or_else(|| Duration::from_secs(default_secs), Duration::from_secs)
}

pub const CONFIG_FILE: &str = "config.toml";
pub const SOURCES_FILE: &str = "sources.toml";
pub const CONTENT_DIR: &str = "content";
pub const CONTENT_MANIFEST: &str = "content-manifest.toml";
pub const ENTITY_GRAPH_JSON: &str = "static/graph/entity-graph.json";
pub const CERTIFICATION_MANIFEST: &str = "static/certification/manifest.json";
pub const CAS_MANIFEST: &str = "static/cas/build-manifest.json";
pub const VIZ_OUTPUT_DIR: &str = "static/viz";
pub const NOTEBOOK_OUTPUT: &str = "content/lab/notebooks";
pub const GATE_MARKER: &str = ".gate";
pub const SPRINGS_DIR: &str = "springs";

// --- Environment variable names (single source of truth) ---

pub const ENV_FORGE_URL: &str = "SPOREPRINT_FORGE_URL";
pub const ENV_RIBOCIPHER: &str = "SPOREPRINT_RIBOCIPHER";
pub const ENV_REFRESH_PAT: &str = "SPOREPRINT_REFRESH_PAT";
pub const ENV_NOTEBOOK_OUTPUT: &str = "SPOREPRINT_NOTEBOOK_OUTPUT";
pub const ENV_TRANSPORT_ENDPOINT: &str = "TRANSPORT_ENDPOINT";
pub const ENV_BIOMEOS_SOCKET_DIR: &str = "BIOMEOS_SOCKET_DIR";
pub const ENV_BIOMEOS_SYSTEMD_DIR: &str = "BIOMEOS_SYSTEMD_SOCKET_DIR";
pub const ENV_XDG_RUNTIME: &str = "XDG_RUNTIME_DIR";
pub const ENV_PLASMIDBIN_CHECKSUMS: &str = "PLASMIDBIN_CHECKSUMS";

/// Default forge URL when `SPOREPRINT_FORGE_URL` is not set.
pub const DEFAULT_FORGE_URL: &str = "https://github.com";

/// Default weight for rendered notebook pages (positions in lab section).
pub const NOTEBOOK_DEFAULT_WEIGHT: u32 = 50;
/// Default domain for rendered notebook pages.
pub const NOTEBOOK_DEFAULT_DOMAIN: &str = "Lab";

/// Resolve the content directory, returning an error if missing.
pub fn require_content_dir(root: &Path) -> Result<PathBuf, Error> {
    let content = root.join(CONTENT_DIR);
    if content.is_dir() {
        Ok(content)
    } else {
        Err(Error::Config(format!(
            "{CONTENT_DIR}/ directory not found at {}",
            root.display()
        )))
    }
}

/// Strip a root prefix from a path, returning the original if stripping fails.
///
/// Common pattern: display paths relative to root for readable diagnostics.
#[must_use]
pub fn rel_to<'a>(path: &'a Path, root: &Path) -> &'a Path {
    path.strip_prefix(root).unwrap_or(path)
}

/// Walk a directory recursively, yielding markdown file entries sorted by name.
#[must_use = "walk iterators are lazy; consume or collect them"]
pub fn walk_markdown_files(dir: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
}

/// Walk a directory recursively, yielding markdown and HTML file entries sorted by name.
#[must_use = "walk iterators are lazy; consume or collect them"]
pub fn walk_content_files(dir: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "md" || ext == "html")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_to_strips_prefix() {
        let root = Path::new("/home/user/project");
        let full = Path::new("/home/user/project/src/main.rs");
        assert_eq!(rel_to(full, root), Path::new("src/main.rs"));
    }

    #[test]
    fn rel_to_returns_original_on_mismatch() {
        let root = Path::new("/home/user/project");
        let unrelated = Path::new("/tmp/other.rs");
        assert_eq!(rel_to(unrelated, root), unrelated);
    }

    #[test]
    fn require_content_dir_succeeds_when_present() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(CONTENT_DIR)).unwrap();
        assert!(require_content_dir(dir.path()).is_ok());
    }

    #[test]
    fn require_content_dir_fails_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(require_content_dir(dir.path()).is_err());
    }

    #[test]
    fn constants_are_consistent() {
        assert!(NOTEBOOK_OUTPUT.starts_with(CONTENT_DIR));
        assert!(
            Path::new(CAS_MANIFEST)
                .extension()
                .is_some_and(|ext| ext == "json")
        );
        assert!(
            Path::new(ENTITY_GRAPH_JSON)
                .extension()
                .is_some_and(|ext| ext == "json")
        );
    }
}
