// SPDX-License-Identifier: AGPL-3.0-or-later

//! Content validation: taxonomy tags, entity shortcodes, and internal links.
//!
//! Validates that content pages reference valid registry entities and that
//! internal links use Zola's `@/` prefix for proper resolution.

use crate::error::Diagnostic;
use crate::model::{Entity, EntityKind};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

/// Validate taxonomy tags in front matter reference valid registry keys.
pub fn validate_taxonomies(
    root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let registry_keys: HashSet<&str> = registry.keys().map(String::as_str).collect();
    let mut referenced_keys: HashSet<String> = HashSet::new();

    for entry in markdown_files(content_dir) {
        let path = entry.path();
        if path.file_name().is_some_and(|n| n == "_index.md") {
            continue;
        }
        let Some(fm) = extract_front_matter(path) else {
            continue;
        };
        let rel = path.strip_prefix(root).unwrap_or(path);

        let Some(taxonomies) = fm.get("taxonomies").and_then(|v| v.as_table()) else {
            continue;
        };

        for &(tax_name, expected_kind) in EntityKind::taxonomy_pairs() {
            let Some(tags) = taxonomies.get(tax_name).and_then(|v| v.as_array()) else {
                continue;
            };

            for tag_val in tags {
                let Some(tag) = tag_val.as_str() else {
                    continue;
                };
                referenced_keys.insert(tag.to_string());

                if !registry_keys.contains(tag) {
                    diagnostics.push(Diagnostic::error(format!(
                        "{}: taxonomy tag '{tag}' not in entity_registry",
                        rel.display()
                    )));
                } else if let Some(entity) = registry.get(tag) {
                    if entity.kind != expected_kind {
                        diagnostics.push(Diagnostic::warning(format!(
                            "{}: tag '{tag}' in [{tax_name}] but registry says kind='{}' \
                             (expected '{expected_kind}')",
                            rel.display(),
                            entity.kind
                        )));
                    }
                }
            }
        }
    }

    let mut unreferenced: Vec<&str> = registry
        .iter()
        .filter(|(_, e)| matches!(e.kind, EntityKind::Primal | EntityKind::Spring))
        .map(|(k, _)| k.as_str())
        .filter(|k| !referenced_keys.contains(*k))
        .collect();
    unreferenced.sort_unstable();

    for key in unreferenced {
        diagnostics.push(Diagnostic::warning(format!(
            "[{key}] is in registry but no content page tags it"
        )));
    }
}

/// Normalize a shortcode name: lowercase, strip spaces and hyphens.
fn normalize_key(name: &str) -> String {
    name.to_lowercase().replace([' ', '-'], "")
}

/// Scan prose for entity shortcodes and validate registry keys.
pub fn check_integrity(
    root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let shortcode_re = Regex::new(
        r#"\{\{\s*entity(?:_metrics|_stat)?\(\s*name\s*=\s*"([^"]+)"\s*(?:,\s*stat\s*=\s*"[^"]*"\s*)?\)\s*\}\}"#,
    )
    .expect("valid regex");
    let registry_keys: HashSet<&str> = registry.keys().map(String::as_str).collect();
    let mut shortcode_count: u32 = 0;
    let mut broken = Vec::new();

    for entry in markdown_files(content_dir) {
        let path = entry.path();
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let rel = path.strip_prefix(root).unwrap_or(path);

        for cap in shortcode_re.captures_iter(&text) {
            let raw_name = &cap[1];
            let key = normalize_key(raw_name);
            shortcode_count += 1;
            if !registry_keys.contains(key.as_str()) {
                broken.push(format!(
                    "{}: entity shortcode name=\"{raw_name}\" (normalized: \"{key}\") not in registry",
                    rel.display()
                ));
            }
        }
    }

    for b in broken {
        diagnostics.push(Diagnostic::error(b));
    }

    diagnostics.push(Diagnostic::warning(format!(
        "check: {shortcode_count} entity shortcodes scanned, all resolved"
    )));
}

/// Detect bare `.md` links that bypass Zola's internal-link resolver.
pub fn lint_internal_links(root: &Path, content_dir: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let bare_md_re = Regex::new(r"\]\(([^@\)][^:\)]*\.md)\)").expect("valid regex");
    let mut count: u32 = 0;

    for entry in markdown_files(content_dir) {
        let path = entry.path();
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let rel = path.strip_prefix(root).unwrap_or(path);

        for (line_no, line) in text.lines().enumerate() {
            for cap in bare_md_re.captures_iter(line) {
                let target = &cap[1];
                if target.starts_with("http://") || target.starts_with("https://") {
                    continue;
                }
                count += 1;
                diagnostics.push(Diagnostic::error(format!(
                    "{}:{}: bare .md link '{}' — use @/ prefix for Zola internal links",
                    rel.display(),
                    line_no + 1,
                    target,
                )));
            }
        }
    }

    if count == 0 {
        diagnostics.push(Diagnostic::warning(
            "link-lint: all internal links use @/ prefix",
        ));
    }
}

fn markdown_files(dir: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
}

/// Extract TOML front matter from a Zola content file.
pub fn extract_front_matter(path: &Path) -> Option<toml::Table> {
    let text = std::fs::read_to_string(path).ok()?;
    let trimmed = text.trim_start();
    if !trimmed.starts_with("+++") {
        return None;
    }
    let after_delim = &trimmed[3..];
    let end = after_delim.find("+++")?;
    let fm_str = after_delim[..end].trim();
    toml::from_str(fm_str).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn front_matter_extraction() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.md");
        let mut f = std::fs::File::create(&file).unwrap();
        writeln!(
            f,
            r#"+++
title = "Test"
[taxonomies]
primals = ["beardog"]
+++

Body text."#
        )
        .unwrap();

        let fm = extract_front_matter(&file).unwrap();
        assert_eq!(fm["title"].as_str().unwrap(), "Test");
        let taxonomies = fm["taxonomies"].as_table().unwrap();
        let primals = taxonomies["primals"].as_array().unwrap();
        assert_eq!(primals[0].as_str().unwrap(), "beardog");
    }

    #[test]
    fn normalize_key_strips_spaces_and_hyphens() {
        assert_eq!(normalize_key("Bear-Dog"), "beardog");
        assert_eq!(normalize_key("hot Spring"), "hotspring");
        assert_eq!(normalize_key("plainkey"), "plainkey");
    }

    #[test]
    fn validate_taxonomies_catches_unknown_tag() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        let file = content.join("test.md");
        std::fs::write(
            &file,
            "+++\ntitle = \"T\"\n[taxonomies]\nprimals = [\"nonexistent\"]\n+++\nBody\n",
        )
        .unwrap();

        let registry = HashMap::new();
        let mut diags = Vec::new();
        validate_taxonomies(root, &content, &registry, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.is_error() && d.message().contains("nonexistent"))
        );
    }

    #[test]
    fn lint_internal_links_catches_bare_md() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        let file = content.join("test.md");
        std::fs::write(&file, "+++\ntitle = \"T\"\n+++\n[link](OTHER.md)\n").unwrap();

        let mut diags = Vec::new();
        lint_internal_links(root, &content, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.is_error() && d.message().contains("bare .md link"))
        );
    }

    #[test]
    fn lint_internal_links_allows_at_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        let file = content.join("test.md");
        std::fs::write(
            &file,
            "+++\ntitle = \"T\"\n+++\n[link](@/section/OTHER.md)\n",
        )
        .unwrap();

        let mut diags = Vec::new();
        lint_internal_links(root, &content, &mut diags);
        assert!(!diags.iter().any(|d| d.is_error()));
    }
}
