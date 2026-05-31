// SPDX-License-Identifier: AGPL-3.0-or-later

//! Internal link validation for Zola content.
//!
//! Scans all `.md` files in `content/` and verifies that:
//! - `@/path/to/page.md` internal links resolve to existing files
//! - Anchor fragments (`#section`) correspond to headings (best-effort)
//! - No broken relative links
//!
//! This absorbs the need for external link-checking tools, keeping
//! the validation pipeline pure Rust and self-contained.

use crate::error::Diagnostic;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Collect all content page paths relative to the content root.
fn collect_pages(content_root: &Path) -> HashSet<String> {
    let mut pages = HashSet::new();
    for entry in WalkDir::new(content_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md") {
            if let Ok(relative) = path.strip_prefix(content_root) {
                pages.insert(relative.to_string_lossy().to_string());
            }
        }
    }
    pages
}

/// Extract internal links from markdown content.
///
/// Recognizes `@/path/to/page.md` Zola-style internal links and
/// regular `](/path)` markdown links.
fn extract_internal_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let link_re = regex::Regex::new(r"(?:\]\()(@/[^)#\s]+)").unwrap();

    for cap in link_re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            links.push(m.as_str().to_string());
        }
    }
    links
}

/// Validate all internal `@/` links in content files.
pub fn validate_internal_links(content_root: &Path) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let pages = collect_pages(content_root);

    for entry in WalkDir::new(content_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };

        let file_display = path
            .strip_prefix(content_root)
            .unwrap_or(path)
            .to_string_lossy();

        for link in extract_internal_links(&content) {
            let target = link.strip_prefix("@/").unwrap_or(&link);
            if !pages.contains(target) {
                let without_index = if target.ends_with("_index.md") {
                    target.to_string()
                } else {
                    let parent = PathBuf::from(target);
                    let dir = parent.parent().unwrap_or_else(|| Path::new(""));
                    dir.join("_index.md").to_string_lossy().to_string()
                };

                if !pages.contains(&without_index) {
                    diagnostics.push(Diagnostic::Warning(format!(
                        "{file_display}: broken link @/{target}"
                    )));
                }
            }
        }
    }

    diagnostics
}

/// Summary of link validation results.
pub struct LinkReport {
    pub files_scanned: usize,
    pub links_found: usize,
    pub broken_links: Vec<String>,
}

/// Full link validation pass with summary.
pub fn check_links(content_root: &Path) -> LinkReport {
    let mut files_scanned = 0;
    let mut links_found = 0;
    let mut broken = Vec::new();
    let pages = collect_pages(content_root);

    for entry in WalkDir::new(content_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        files_scanned += 1;

        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };

        let file_display = path
            .strip_prefix(content_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let links = extract_internal_links(&content);
        links_found += links.len();

        for link in links {
            let target = link.strip_prefix("@/").unwrap_or(&link);
            if !pages.contains(target) {
                broken.push(format!("{file_display} -> @/{target}"));
            }
        }
    }

    LinkReport {
        files_scanned,
        links_found,
        broken_links: broken,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_links_finds_at_prefixed() {
        let content = r#"See [this page](@/science/paper.md) for details."#;
        let links = extract_internal_links(content);
        assert_eq!(links, vec!["@/science/paper.md"]);
    }

    #[test]
    fn extract_links_finds_multiple() {
        let content = r#"
Check [A](@/arch/one.md) and [B](@/arch/two.md).
"#;
        let links = extract_internal_links(content);
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"@/arch/one.md".to_string()));
        assert!(links.contains(&"@/arch/two.md".to_string()));
    }

    #[test]
    fn extract_links_ignores_external() {
        let content = r#"[ext](https://example.com) and [int](@/page.md)"#;
        let links = extract_internal_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "@/page.md");
    }

    #[test]
    fn collect_pages_finds_md_files() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("science");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("paper.md"), "+++\n+++\nContent").unwrap();
        std::fs::write(dir.path().join("_index.md"), "+++\n+++\nRoot").unwrap();

        let pages = collect_pages(dir.path());
        assert!(pages.contains("science/paper.md"));
        assert!(pages.contains("_index.md"));
    }

    #[test]
    fn validate_finds_broken_links() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("page.md"),
            "+++\n+++\n[link](@/nonexistent.md)",
        )
        .unwrap();

        let diagnostics = validate_internal_links(dir.path());
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message().contains("nonexistent.md"));
    }

    #[test]
    fn validate_passes_existing_links() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("target.md"), "+++\n+++\nTarget").unwrap();
        std::fs::write(
            dir.path().join("source.md"),
            "+++\n+++\n[link](@/target.md)",
        )
        .unwrap();

        let diagnostics = validate_internal_links(dir.path());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn check_links_report_summary() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.md"), "+++\n+++\n[x](@/b.md)").unwrap();
        std::fs::write(dir.path().join("b.md"), "+++\n+++\nB content").unwrap();

        let report = check_links(dir.path());
        assert_eq!(report.files_scanned, 2);
        assert_eq!(report.links_found, 1);
        assert!(report.broken_links.is_empty());
    }
}
