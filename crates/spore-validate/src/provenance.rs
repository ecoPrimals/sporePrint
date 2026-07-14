// SPDX-License-Identifier: AGPL-3.0-or-later

//! Content-addressed provenance for sporePrint pages.
//!
//! Every published page gets a BLAKE3 hash computed from its content bytes.
//! The manifest (`content-manifest.toml`) is the first step toward full
//! provenance trio integration: rhizoCrypt DAG tracking, loamSpine ledger
//! entries, and sweetGrass PROV-O attribution braids.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentManifest {
    pub generated: String,
    pub root_hash: String,
    pub page_count: usize,
    pub pages: BTreeMap<String, PageEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageEntry {
    pub blake3: String,
    pub size_bytes: u64,
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trails: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub companions: Vec<String>,
}

pub fn generate_manifest(content_dir: &Path) -> ContentManifest {
    let mut pages = BTreeMap::new();
    let mut root_hasher = blake3::Hasher::new();

    for entry in crate::paths::walk_content_files(content_dir) {
        let path = entry.path();
        let rel = crate::paths::rel_to(path, content_dir)
            .to_string_lossy()
            .to_string();

        if let Ok(bytes) = fs::read(path) {
            let hash = blake3::hash(&bytes);
            let hash_hex = hash.to_hex().to_string();

            root_hasher.update(rel.as_bytes());
            root_hasher.update(hash.as_bytes());

            let title = extract_title(&bytes);
            let (section, maturity, trails, companions) = extract_semantic_metadata(&rel, &bytes);

            pages.insert(
                rel,
                PageEntry {
                    blake3: hash_hex,
                    size_bytes: bytes.len() as u64,
                    title,
                    section,
                    maturity,
                    trails,
                    companions,
                },
            );
        }
    }

    let root_hash = root_hasher.finalize().to_hex().to_string();

    ContentManifest {
        generated: crate::time::now_iso8601(),
        root_hash,
        page_count: pages.len(),
        pages,
    }
}

pub fn diff_manifests(
    old_path: &Path,
    new_manifest: &ContentManifest,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let old_manifest = load_manifest(old_path);

    let mut new_pages = Vec::new();
    let mut changed = Vec::new();
    let mut removed = Vec::new();

    if let Some(old) = &old_manifest {
        for (path, entry) in &new_manifest.pages {
            match old.pages.get(path) {
                None => new_pages.push(path.clone()),
                Some(old_entry) if old_entry.blake3 != entry.blake3 => {
                    changed.push(path.clone());
                }
                _ => {}
            }
        }

        for path in old.pages.keys() {
            if !new_manifest.pages.contains_key(path) {
                removed.push(path.clone());
            }
        }
    } else {
        new_pages.extend(new_manifest.pages.keys().cloned());
    }

    (new_pages, changed, removed)
}

pub fn write_manifest(manifest: &ContentManifest, output: &Path) -> std::io::Result<()> {
    let toml_str =
        toml::to_string_pretty(manifest).map_err(|e| std::io::Error::other(e.to_string()))?;
    fs::write(output, toml_str)
}

fn load_manifest(path: &Path) -> Option<ContentManifest> {
    let text = fs::read_to_string(path).ok()?;
    toml::from_str(&text).ok()
}

fn extract_semantic_metadata(
    rel_path: &str,
    bytes: &[u8],
) -> (Option<String>, Option<String>, Vec<String>, Vec<String>) {
    let section = rel_path.split('/').next().map(String::from);

    let Ok(text) = std::str::from_utf8(bytes) else {
        return (section, None, Vec::new(), Vec::new());
    };
    let Some(fm) = parse_front_matter(text) else {
        return (section, None, Vec::new(), Vec::new());
    };

    let maturity = fm
        .get("extra")
        .and_then(|e| e.get("maturity"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let trails = fm
        .get("taxonomies")
        .and_then(|t| t.get("trails"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let companions = fm
        .get("extra")
        .and_then(|e| e.get("companions"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| c.get("url").and_then(|u| u.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();

    (section, maturity, trails, companions)
}

fn parse_front_matter(text: &str) -> Option<toml::Table> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("+++") {
        return None;
    }
    let after_delim = &trimmed[3..];
    let end = after_delim.find("+++")?;
    let fm_str = after_delim[..end].trim();
    toml::from_str(fm_str).ok()
}

fn extract_title(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("title") && trimmed.contains('=') {
            let value = trimmed.split('=').nth(1)?.trim();
            return Some(value.trim_matches('"').to_string());
        }
    }
    None
}

pub fn manifest_path(root: &Path) -> PathBuf {
    root.join(crate::paths::CONTENT_MANIFEST)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn blake3_deterministic() {
        let data = b"# Test page\nHello world";
        let h1 = blake3::hash(data).to_hex().to_string();
        let h2 = blake3::hash(data).to_hex().to_string();
        assert_eq!(h1, h2);
    }

    #[test]
    fn generate_manifest_empty_dir() {
        let dir = TempDir::new().unwrap();
        let m = generate_manifest(dir.path());
        assert_eq!(m.page_count, 0);
        assert!(m.pages.is_empty());
    }

    #[test]
    fn generate_manifest_with_pages() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("test.md"),
            "+++\ntitle = \"Test\"\n+++\nContent",
        )
        .unwrap();
        fs::write(dir.path().join("other.md"), "# Other").unwrap();

        let m = generate_manifest(dir.path());
        assert_eq!(m.page_count, 2);
        assert!(m.pages.contains_key("test.md"));
        assert!(m.pages.contains_key("other.md"));
        assert_eq!(m.pages["test.md"].title.as_deref(), Some("Test"));
    }

    #[test]
    fn diff_detects_new_pages() {
        let dir = TempDir::new().unwrap();
        let old_path = dir.path().join("old.toml");

        let old = ContentManifest {
            generated: "test".into(),
            root_hash: "abc".into(),
            page_count: 0,
            pages: BTreeMap::new(),
        };
        let toml_str = toml::to_string_pretty(&old).unwrap();
        fs::write(&old_path, toml_str).unwrap();

        let mut new_pages = BTreeMap::new();
        new_pages.insert(
            "new.md".to_string(),
            PageEntry {
                blake3: "def".into(),
                size_bytes: 10,
                title: None,
                section: None,
                maturity: None,
                trails: Vec::new(),
                companions: Vec::new(),
            },
        );
        let new_manifest = ContentManifest {
            generated: "test".into(),
            root_hash: "xyz".into(),
            page_count: 1,
            pages: new_pages,
        };

        let (added, changed, removed) = diff_manifests(&old_path, &new_manifest);
        assert_eq!(added, vec!["new.md"]);
        assert!(changed.is_empty());
        assert!(removed.is_empty());
    }
}
