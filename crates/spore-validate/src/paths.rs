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
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// Transport connect timeout for CAS push and HTTP fetch.
pub const TRANSPORT_CONNECT_TIMEOUT: Duration = Duration::from_secs(15);

/// Transport I/O (read/write) timeout for CAS push and HTTP fetch.
pub const TRANSPORT_IO_TIMEOUT: Duration = Duration::from_secs(30);

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
