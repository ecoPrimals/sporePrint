use crate::model::{Entity, EntityKind};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

/// Phase 1: validate taxonomy tags in front matter reference valid registry keys.
pub fn validate_taxonomies(
    root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
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

        for (tax_name, expected_kind) in [
            ("primals", EntityKind::Primal),
            ("springs", EntityKind::Spring),
        ] {
            let Some(tags) = taxonomies.get(tax_name).and_then(|v| v.as_array()) else {
                continue;
            };

            for tag_val in tags {
                let Some(tag) = tag_val.as_str() else {
                    continue;
                };
                referenced_keys.insert(tag.to_string());

                if !registry_keys.contains(tag) {
                    errors.push(format!(
                        "{}: taxonomy tag '{tag}' not in entity_registry",
                        rel.display()
                    ));
                } else if let Some(entity) = registry.get(tag) {
                    if entity.kind != expected_kind {
                        warnings.push(format!(
                            "{}: tag '{tag}' in [{tax_name}] but registry says kind='{}' \
                             (expected '{expected_kind}')",
                            rel.display(),
                            entity.kind
                        ));
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
    unreferenced.sort();

    for key in unreferenced {
        warnings.push(format!("[{key}] is in registry but no content page tags it"));
    }
}

/// Normalize a shortcode name the same way the Tera templates do:
/// lowercase, strip spaces and hyphens.
fn normalize_key(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "")
        .replace('-', "")
}

/// Phase 3: scan prose for entity shortcodes and validate registry keys.
///
/// Checks `entity(name="…")`, `entity_metrics(name="…")`, and
/// `entity_stat(name="…")` — matching the normalization the Tera
/// templates apply (lowercase, strip spaces and hyphens).
pub fn check_integrity(
    root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
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
        errors.push(b);
    }

    warnings.push(format!(
        "check: {shortcode_count} entity shortcodes scanned, all resolved"
    ));
}

fn markdown_files(dir: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "md")
        })
}

fn extract_front_matter(path: &Path) -> Option<toml::Table> {
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

    #[test]
    fn front_matter_extraction() {
        use std::io::Write;
        let dir = std::env::temp_dir().join("spore_test_fm");
        let _ = std::fs::create_dir_all(&dir);
        let file = dir.join("test.md");
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

        let _ = std::fs::remove_dir_all(&dir);
    }
}
