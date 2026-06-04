// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical path constants and helpers for sporePrint's directory layout.
//!
//! Centralizes all path literals so the layout can evolve without grep-hunting.

use crate::error::Error;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "config.toml";
pub const SOURCES_FILE: &str = "sources.toml";
pub const CONTENT_DIR: &str = "content";
pub const CONTENT_MANIFEST: &str = "content-manifest.toml";
pub const ENTITY_GRAPH_JSON: &str = "static/graph/entity-graph.json";
pub const CERTIFICATION_MANIFEST: &str = "static/certification/manifest.json";
pub const CAS_MANIFEST: &str = "static/cas/build-manifest.json";
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
pub fn rel_to<'a>(path: &'a Path, root: &Path) -> &'a Path {
    path.strip_prefix(root).unwrap_or(path)
}
