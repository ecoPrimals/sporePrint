// SPDX-License-Identifier: AGPL-3.0-or-later

//! Content validation: taxonomy tags, entity shortcodes, and internal links.
//!
//! Validates that content pages reference valid registry entities and that
//! internal links use Zola's `@/` prefix for proper resolution.

use crate::error::Diagnostic;
use crate::model::{Entity, EntityKind, MaturityLevel};
use crate::paths;
use regex::Regex;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

/// Regex matching all entity shortcode variants: `entity`, `entity_metrics`, `entity_stat`.
static ENTITY_SHORTCODE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"\{\{\s*entity(?:_metrics|_stat)?\(\s*name\s*=\s*"([^"]+)"\s*(?:,\s*stat\s*=\s*"[^"]*"\s*)?\)\s*\}\}"#,
    )
    .expect("static regex")
});

/// Validate taxonomy tags in front matter reference valid registry keys.
pub fn validate_taxonomies(
    root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let registry_keys: HashSet<&str> = registry.keys().map(String::as_str).collect();
    let mut referenced_keys: HashSet<String> = HashSet::new();

    for entry in paths::walk_markdown_files(content_dir) {
        let path = entry.path();
        if path.file_name().is_some_and(|n| n == "_index.md") {
            continue;
        }
        let Some(fm) = extract_front_matter(path) else {
            continue;
        };
        let rel = paths::rel_to(path, root);

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
#[must_use]
fn normalize_key(key: &str) -> Cow<'_, str> {
    if key
        .bytes()
        .all(|b| b.is_ascii_lowercase() && b != b' ' && b != b'-')
    {
        Cow::Borrowed(key)
    } else {
        Cow::Owned(key.to_lowercase().replace([' ', '-'], ""))
    }
}

/// Scan prose for entity shortcodes and validate registry keys.
pub fn check_integrity(
    root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let shortcode_re = &*ENTITY_SHORTCODE_RE;
    let registry_keys: HashSet<&str> = registry.keys().map(String::as_str).collect();
    let mut shortcode_count: u32 = 0;
    let mut broken = Vec::new();

    for entry in paths::walk_markdown_files(content_dir) {
        let path = entry.path();
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let rel = paths::rel_to(path, root);

        for cap in shortcode_re.captures_iter(&text) {
            let raw_name = &cap[1];
            let key = normalize_key(raw_name);
            shortcode_count += 1;
            if !registry_keys.contains(key.as_ref()) {
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
    static BARE_MD_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\]\(([^@\)][^:\)]*\.md)\)").expect("static regex"));
    let bare_md_re = &*BARE_MD_RE;
    let mut count: u32 = 0;

    for entry in paths::walk_markdown_files(content_dir) {
        let path = entry.path();
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let rel = paths::rel_to(path, root);

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

/// Audit: pages that reference entities via shortcodes but don't list them
/// in `[taxonomies]` front matter. Emits warnings for potential tagging gaps.
pub fn audit_taxonomy_coverage(
    _root: &Path,
    content_dir: &Path,
    registry: &HashMap<String, Entity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let shortcode_re = &*ENTITY_SHORTCODE_RE;

    let mut gap_count: u32 = 0;

    for entry in paths::walk_markdown_files(content_dir) {
        let path = entry.path();
        if path.file_name().is_some_and(|n| n == "_index.md") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };

        let fm_entities = extract_taxonomy_entities(path);

        let body = strip_front_matter(&text);

        let mut shortcode_entities: HashSet<String> = HashSet::new();
        for cap in shortcode_re.captures_iter(body) {
            let key = normalize_key(&cap[1]);
            if let Some(entity) = registry.get(key.as_ref()) {
                if entity.kind.has_taxonomy() {
                    shortcode_entities.insert(key.into_owned());
                }
            }
        }

        for entity_key in &shortcode_entities {
            if !fm_entities.contains(entity_key.as_str()) {
                gap_count += 1;
            }
        }
    }

    if gap_count > 0 {
        diagnostics.push(Diagnostic::warning(format!(
            "taxonomy-audit: {gap_count} entity shortcode(s) not reflected in page taxonomies"
        )));
    }
}

/// Extract entity keys listed in a page's `[taxonomies]` section.
fn extract_taxonomy_entities(path: &Path) -> HashSet<String> {
    let mut keys = HashSet::new();
    let Some(fm) = extract_front_matter(path) else {
        return keys;
    };
    let Some(taxonomies) = fm.get("taxonomies").and_then(|v| v.as_table()) else {
        return keys;
    };
    for (_tax_name, tax_val) in taxonomies {
        if let Some(arr) = tax_val.as_array() {
            for item in arr {
                if let Some(s) = item.as_str() {
                    keys.insert(normalize_key(s).into_owned());
                }
            }
        }
    }
    keys
}

/// Strip TOML front matter delimiters from content, returning just the body.
#[must_use]
fn strip_front_matter(text: &str) -> &str {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("+++") {
        return text;
    }
    let after_open = &trimmed[3..];
    after_open
        .find("+++")
        .map_or(text, |pos| &after_open[pos + 3..])
}

/// Scan content for maturity shortcode usage and validate levels.
///
/// Reports warnings for unknown maturity levels that don't match
/// the `MaturityLevel` enum.
pub fn validate_maturity_levels(content_dir: &Path, diagnostics: &mut Vec<Diagnostic>) {
    static MATURITY_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"\{\{[\s]*maturity\s*\(\s*level\s*=\s*"([^"]+)""#).unwrap());

    for entry in paths::walk_markdown_files(content_dir) {
        let path = entry.path().to_path_buf();
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };

        for cap in MATURITY_RE.captures_iter(&text) {
            if let Some(m) = cap.get(1) {
                let level_str = m.as_str();
                if MaturityLevel::from_str_loose(level_str).is_none() {
                    diagnostics.push(Diagnostic::warning(format!(
                        "unknown maturity level '{level_str}' in {}",
                        paths::rel_to(&path, content_dir).display()
                    )));
                }
            }
        }
    }
}

/// Extract TOML front matter from a Zola content file.
#[must_use]
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
        assert!(!diags.iter().any(Diagnostic::is_error));
    }

    #[test]
    fn strip_front_matter_returns_body() {
        let text = "+++\ntitle = \"T\"\n+++\nBody here";
        let body = strip_front_matter(text);
        assert!(body.contains("Body here"));
        assert!(!body.contains("title"));
    }

    #[test]
    fn strip_front_matter_no_delimiters() {
        let text = "Just plain text";
        assert_eq!(strip_front_matter(text), text);
    }

    #[test]
    fn audit_taxonomy_finds_gap() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        let file = content.join("test.md");
        std::fs::write(
            &file,
            "+++\ntitle = \"T\"\n+++\n{{ entity(name=\"beardog\") }} is great\n",
        )
        .unwrap();

        let mut registry = HashMap::new();
        registry.insert("beardog".to_string(), test_entity(EntityKind::Primal));

        let mut diags = Vec::new();
        audit_taxonomy_coverage(root, &content, &registry, &mut diags);
        assert!(diags.iter().any(|d| d.message().contains("taxonomy-audit")));
    }

    #[test]
    fn audit_taxonomy_no_gap_when_tagged() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        let file = content.join("test.md");
        std::fs::write(
            &file,
            "+++\ntitle = \"T\"\n[taxonomies]\nprimals = [\"beardog\"]\n+++\n{{ entity(name=\"beardog\") }} tagged\n",
        )
        .unwrap();

        let mut registry = HashMap::new();
        registry.insert("beardog".to_string(), test_entity(EntityKind::Primal));

        let mut diags = Vec::new();
        audit_taxonomy_coverage(root, &content, &registry, &mut diags);
        assert!(!diags.iter().any(|d| d.message().contains("taxonomy-audit")));
    }

    fn test_entity(kind: EntityKind) -> Entity {
        Entity {
            display: "Test".into(),
            emoji: "🧪".into(),
            kind,
            description: None,
            domain: None,
            loc: None,
            loc_display: None,
            tests: None,
            tests_display: None,
            files: None,
            crates: None,
            repo: None,
            tier: None,
            composes: None,
            capabilities: None,
            page: None,
            edges: None,
        }
    }

    #[test]
    fn check_integrity_valid_shortcodes() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        std::fs::write(
            content.join("test.md"),
            "+++\ntitle = \"T\"\n+++\n{{ entity(name=\"beardog\") }} rocks\n",
        )
        .unwrap();

        let mut registry = HashMap::new();
        registry.insert("beardog".to_string(), test_entity(EntityKind::Primal));

        let mut diags = Vec::new();
        check_integrity(root, &content, &registry, &mut diags);
        assert!(!diags.iter().any(Diagnostic::is_error));
        assert!(
            diags
                .iter()
                .any(|d| d.message().contains("1 entity shortcodes scanned"))
        );
    }

    #[test]
    fn check_integrity_detects_unknown_entity() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        std::fs::write(
            content.join("test.md"),
            "+++\ntitle = \"T\"\n+++\n{{ entity(name=\"nonexistent\") }} hmm\n",
        )
        .unwrap();

        let registry = HashMap::new();
        let mut diags = Vec::new();
        check_integrity(root, &content, &registry, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.is_error() && d.message().contains("nonexistent"))
        );
    }

    #[test]
    fn check_integrity_normalizes_names() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        std::fs::write(
            content.join("test.md"),
            "+++\ntitle = \"T\"\n+++\n{{ entity(name=\"Bear-Dog\") }} normalized\n",
        )
        .unwrap();

        let mut registry = HashMap::new();
        registry.insert("beardog".to_string(), test_entity(EntityKind::Primal));

        let mut diags = Vec::new();
        check_integrity(root, &content, &registry, &mut diags);
        assert!(!diags.iter().any(Diagnostic::is_error));
    }

    #[test]
    fn check_integrity_handles_entity_metrics_shortcode() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let content = root.join("content");
        std::fs::create_dir_all(&content).unwrap();
        std::fs::write(
            content.join("test.md"),
            "+++\ntitle = \"T\"\n+++\n{{ entity_metrics(name=\"songbird\") }}\n",
        )
        .unwrap();

        let mut registry = HashMap::new();
        registry.insert("songbird".to_string(), test_entity(EntityKind::Primal));

        let mut diags = Vec::new();
        check_integrity(root, &content, &registry, &mut diags);
        assert!(!diags.iter().any(Diagnostic::is_error));
    }

    #[test]
    fn validate_maturity_levels_catches_unknown() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("test.md"),
            r#"+++
title = "Test"
+++

{{ maturity(level="implemented") }} works fine.
{{ maturity(level="bogus") }} should warn.
"#,
        )
        .unwrap();

        let mut diags = Vec::new();
        validate_maturity_levels(dir.path(), &mut diags);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message().contains("bogus"));
    }

    #[test]
    fn validate_maturity_levels_accepts_all_valid() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("test.md"),
            r#"+++
title = "Test"
+++

{{ maturity(level="implemented") }}
{{ maturity(level="reproduced") }}
{{ maturity(level="certified") }}
{{ maturity(level="architectural") }}
{{ maturity(level="planned") }}
{{ maturity(level="unaudited") }}
"#,
        )
        .unwrap();

        let mut diags = Vec::new();
        validate_maturity_levels(dir.path(), &mut diags);
        assert!(diags.is_empty());
    }
}
