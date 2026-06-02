// SPDX-License-Identifier: AGPL-3.0-or-later

//! Certification manifest — sporePrint as its own guideStone.
//!
//! Applies guideStone's five verification properties to data publication:
//! 1. Deterministic — same config + content = same manifest
//! 2. Reference-traceable — every metric traces to a repo
//! 3. Self-verifying — manifest hash proves graph integrity
//! 4. Environment-agnostic — pure Rust, no external tools
//! 5. Tolerance-documented — drift is declared, not hidden

use crate::graph;
use crate::model::{Config, EntityKind};
use crate::time::today_utc;
use serde::Serialize;
use std::path::Path;
use walkdir::WalkDir;

/// The certification manifest — a self-verifying summary of all published claims.
///
/// Emits both `schema_version`/`merkle_root` (primalSpring expectation) and
/// `version`/`graph_merkle` (legacy) for backward compatibility.
#[derive(Debug, Serialize)]
pub struct CertificationManifest {
    pub schema_version: &'static str,
    pub version: &'static str,
    pub generated: String,
    pub entity_count: usize,
    pub primal_count: usize,
    pub spring_count: usize,
    pub edge_count: usize,
    pub merkle_root: String,
    pub graph_merkle: String,
    pub content_pages: usize,
    pub total_loc: u64,
    pub total_tests: u64,
    pub validation_errors: usize,
    pub measured_date: String,
    pub drift_tolerance: &'static str,
}

/// Build a certification manifest from the current config and content state.
pub fn build_manifest(config: &Config, root: &Path, validation_errors: usize) -> CertificationManifest {
    let registry = &config.extra.entity_registry;

    let entity_count = registry.len();
    let primal_count = registry.values().filter(|e| e.kind == EntityKind::Primal).count();
    let spring_count = registry.values().filter(|e| e.kind == EntityKind::Spring).count();

    let entity_graph = graph::build_graph(registry);
    let edge_count = entity_graph.stats.edge_count;

    let graph_merkle = compute_graph_merkle(registry);
    let content_pages = count_content_pages(&root.join("content"));

    let total_loc = config.extra.totals.total_loc;
    let total_tests = config
        .extra
        .totals
        .total_tests
        .unwrap_or(config.extra.totals.primal_tests + config.extra.totals.spring_tests);

    let measured_date = config
        .extra
        .totals
        .measured_date
        .clone()
        .unwrap_or_else(today_utc);

    let merkle_root = graph_merkle.clone();

    CertificationManifest {
        schema_version: "1.0.0",
        version: "1.0.0",
        generated: format!("{}T00:00:00Z", today_utc()),
        entity_count,
        primal_count,
        spring_count,
        edge_count,
        merkle_root,
        graph_merkle,
        content_pages,
        total_loc,
        total_tests,
        validation_errors,
        measured_date,
        drift_tolerance: "5%/30d",
    }
}

/// Compute a BLAKE3 Merkle root of the entity graph.
///
/// Deterministic: edges are sorted by (source, target, relation) before hashing.
/// This means the same graph always produces the same hash regardless of
/// `HashMap` iteration order.
fn compute_graph_merkle(
    registry: &std::collections::HashMap<String, crate::model::Entity>,
) -> String {
    let mut edge_strings: Vec<String> = Vec::new();

    let mut keys: Vec<&str> = registry.keys().map(String::as_str).collect();
    keys.sort_unstable();

    for key in &keys {
        if let Some(ref edges) = registry[*key].edges {
            for edge in edges {
                edge_strings.push(format!("{}:{}:{}", key, edge.target, edge.relation));
            }
        }
    }

    edge_strings.sort_unstable();

    let mut hasher = blake3::Hasher::new();
    for edge_str in &edge_strings {
        hasher.update(edge_str.as_bytes());
        hasher.update(b"\n");
    }

    format!("blake3:{}", hasher.finalize().to_hex())
}

/// Count markdown content pages (excluding _index.md section files).
fn count_content_pages(content_dir: &Path) -> usize {
    if !content_dir.is_dir() {
        return 0;
    }

    WalkDir::new(content_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .filter(|e| e.path().file_name().is_some_and(|n| n != "_index.md"))
        .count()
}

/// Emit the certification manifest as JSON.
pub fn emit_manifest(manifest: &CertificationManifest, output_path: &Path) -> std::io::Result<()> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(manifest).map_err(std::io::Error::other)?;
    std::fs::write(output_path, json)
}

/// Validate an existing manifest against current state.
/// Returns a list of drift descriptions (empty = all good).
pub fn validate_manifest(
    existing: &Path,
    current: &CertificationManifest,
) -> Result<Vec<String>, std::io::Error> {
    let text = std::fs::read_to_string(existing)?;
    let stored: StoredManifest = serde_json::from_str(&text).map_err(std::io::Error::other)?;

    let mut drifts = Vec::new();

    if stored.effective_merkle() != current.graph_merkle {
        drifts.push(format!(
            "graph_merkle: stored={}, current={}",
            stored.effective_merkle(), current.graph_merkle
        ));
    }
    if stored.entity_count != current.entity_count {
        drifts.push(format!(
            "entity_count: stored={}, current={}",
            stored.entity_count, current.entity_count
        ));
    }
    if stored.edge_count != current.edge_count {
        drifts.push(format!(
            "edge_count: stored={}, current={}",
            stored.edge_count, current.edge_count
        ));
    }
    if stored.content_pages != current.content_pages {
        drifts.push(format!(
            "content_pages: stored={}, current={}",
            stored.content_pages, current.content_pages
        ));
    }

    Ok(drifts)
}

/// Subset of manifest fields for comparison (deserialized from existing file).
///
/// The emitted manifest contains both `merkle_root` (primalSpring expectation)
/// and `graph_merkle` (legacy). We read both and prefer `merkle_root`.
#[derive(serde::Deserialize)]
struct StoredManifest {
    merkle_root: Option<String>,
    graph_merkle: Option<String>,
    entity_count: usize,
    edge_count: usize,
    content_pages: usize,
}

impl StoredManifest {
    fn effective_merkle(&self) -> &str {
        self.merkle_root
            .as_deref()
            .or(self.graph_merkle.as_deref())
            .unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn graph_merkle_is_deterministic() {
        let reg = HashMap::new();
        let hash1 = compute_graph_merkle(&reg);
        let hash2 = compute_graph_merkle(&reg);
        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("blake3:"));
    }

    #[test]
    fn count_content_pages_excludes_index() {
        let dir = tempfile::tempdir().unwrap();
        let content = dir.path().join("content");
        std::fs::create_dir_all(&content).unwrap();
        std::fs::write(content.join("_index.md"), "+++\n+++\n").unwrap();
        std::fs::write(content.join("page.md"), "+++\n+++\nBody").unwrap();
        std::fs::write(content.join("other.md"), "+++\n+++\nBody").unwrap();

        assert_eq!(count_content_pages(&content), 2);
    }

    #[test]
    fn manifest_serializes_to_json() {
        let m = CertificationManifest {
            schema_version: "1.0.0",
            version: "1.0.0",
            generated: "2026-06-01T00:00:00Z".into(),
            entity_count: 5,
            primal_count: 3,
            spring_count: 2,
            edge_count: 10,
            merkle_root: "blake3:abc123".into(),
            graph_merkle: "blake3:abc123".into(),
            content_pages: 50,
            total_loc: 100_000,
            total_tests: 5_000,
            validation_errors: 0,
            measured_date: "2026-06-01".into(),
            drift_tolerance: "5%/30d",
        };
        let json = serde_json::to_string_pretty(&m).unwrap();
        assert!(json.contains("\"schema_version\": \"1.0.0\""));
        assert!(json.contains("\"merkle_root\""));
        assert!(json.contains("blake3:abc123"));
    }
}
