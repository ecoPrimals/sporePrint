// SPDX-License-Identifier: AGPL-3.0-or-later

//! Metric refresh: compare registry values against actual repo contents.
//!
//! Scans source repos for LOC, test counts, file counts, and crate counts,
//! then compares against the stored registry values to detect drift.

use crate::error::Error;
use crate::model::{Entity, EntityKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;
use walkdir::WalkDir;

/// A single metric that has drifted from its registered value.
pub struct Drift {
    pub key: String,
    pub field: &'static str,
    pub registered: u64,
    pub actual: u64,
}

/// Result of scanning repos for metric drift.
pub struct RefreshResult {
    pub drifts: Vec<Drift>,
    pub missing_repos: Vec<String>,
    pub scanned: u32,
}

/// Scan repos and compare metrics against the registry.
pub fn scan(
    registry: &HashMap<String, Entity>,
    repos_root: &Path,
    source_filter: Option<&str>,
) -> RefreshResult {
    let mut drifts = Vec::new();
    let mut missing_repos = Vec::new();
    let mut scanned = 0u32;

    let mut keys: Vec<&str> = registry.keys().map(String::as_str).collect();
    keys.sort_unstable();

    for key in keys {
        if source_filter.is_some_and(|f| key != f) {
            continue;
        }

        let entity = &registry[key];
        let Some(repo) = &entity.repo else {
            continue;
        };

        if !matches!(entity.kind, EntityKind::Primal | EntityKind::Spring) && entity.loc.is_none() {
            continue;
        }

        let Some(repo_path) = find_repo(repos_root, repo) else {
            missing_repos.push(format!("{key} ({repo})"));
            continue;
        };

        scanned += 1;
        let metrics = count_metrics(&repo_path);

        let mut push_drift = |field: &'static str, registered: u64, actual: u64| {
            if registered != actual {
                drifts.push(Drift {
                    key: key.to_string(),
                    field,
                    registered,
                    actual,
                });
            }
        };

        if let Some(v) = entity.loc {
            push_drift("loc", v, metrics.loc);
        }
        if let Some(v) = entity.tests {
            push_drift("tests", v, metrics.tests);
        }
        if let Some(v) = entity.files {
            push_drift("files", u64::from(v), metrics.files);
        }
        if let Some(v) = entity.crates {
            push_drift("crates", u64::from(v), metrics.crates);
        }
    }

    RefreshResult {
        drifts,
        missing_repos,
        scanned,
    }
}

/// Format a number with comma separators for display.
fn format_display(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Write drifted metrics back to `config.toml`, preserving formatting.
pub fn write_updates(config_path: &Path, drifts: &[Drift]) -> Result<(), Error> {
    let text = std::fs::read_to_string(config_path).map_err(|e| Error::io(config_path, e))?;

    let mut doc: DocumentMut = text.parse()?;

    let registry = doc
        .get_mut("extra")
        .and_then(|e| e.get_mut("entity_registry"))
        .ok_or_else(|| Error::Config("missing [extra.entity_registry] in config.toml".into()))?;

    for drift in drifts {
        let entity = registry.get_mut(&drift.key).ok_or_else(|| {
            Error::Config(format!("entity '{}' not found in registry", drift.key))
        })?;

        let actual_i64 = i64::try_from(drift.actual).unwrap_or(i64::MAX);

        match drift.field {
            "loc" | "tests" => set_metric_pair(entity, drift.field, actual_i64),
            "files" | "crates" => {
                entity[drift.field] = toml_edit::value(actual_i64);
            }
            _ => {}
        }
    }

    update_totals(&mut doc);

    std::fs::write(config_path, doc.to_string()).map_err(|e| Error::io(config_path, e))
}

/// Set a numeric metric and its display companion in a TOML table.
fn set_metric_pair(table: &mut toml_edit::Item, key: &str, value: i64) {
    table[key] = toml_edit::value(value);
    let display_key = format!("{key}_display");
    table[&display_key] = toml_edit::value(format_display(value.unsigned_abs()));
}

fn update_totals(doc: &mut DocumentMut) {
    let Some(registry) = doc.get("extra").and_then(|e| e.get("entity_registry")) else {
        return;
    };

    let mut primal_loc = 0i64;
    let mut spring_loc = 0i64;
    let mut primal_tests = 0i64;
    let mut spring_tests = 0i64;

    if let Some(table) = registry.as_table_like() {
        for (_key, entity) in table.iter() {
            let kind = entity
                .get("kind")
                .and_then(toml_edit::Item::as_str)
                .unwrap_or("");
            let loc = entity
                .get("loc")
                .and_then(toml_edit::Item::as_integer)
                .unwrap_or(0);
            let tests = entity
                .get("tests")
                .and_then(toml_edit::Item::as_integer)
                .unwrap_or(0);

            match kind {
                "primal" => {
                    primal_loc += loc;
                    primal_tests += tests;
                }
                "spring" => {
                    spring_loc += loc;
                    spring_tests += tests;
                }
                _ => {}
            }
        }
    }

    let total_loc = primal_loc + spring_loc;
    let total_tests = primal_tests + spring_tests;

    if let Some(totals) = doc.get_mut("extra").and_then(|e| e.get_mut("totals")) {
        set_metric_pair(totals, "primal_loc", primal_loc);
        set_metric_pair(totals, "spring_loc", spring_loc);
        set_metric_pair(totals, "total_loc", total_loc);
        set_metric_pair(totals, "primal_tests", primal_tests);
        set_metric_pair(totals, "spring_tests", spring_tests);
        set_metric_pair(totals, "total_tests", total_tests);

        let date_str = today_utc();
        if !date_str.is_empty() {
            totals["measured_date"] = toml_edit::value(date_str);
        }
    }
}

use crate::time::today_utc;

/// Resolve a repo path from its `org/name` string.
///
/// Discovery order (runtime, capability-based):
///   1. Canonical: `root/org/name`
///   2. Any immediate subdirectory of root that contains `name/`
///   3. Flat: `root/name`
///
/// No hardcoded directory names — discovers at runtime by walking
/// the filesystem. A primal has self-knowledge only and discovers
/// structure by probing, not by assuming.
fn find_repo(root: &Path, repo_ref: &str) -> Option<PathBuf> {
    let candidate = root.join(repo_ref);
    if candidate.is_dir() {
        return Some(candidate);
    }

    let name = repo_ref.rsplit('/').next().unwrap_or(repo_ref);

    // Walk immediate subdirectories — discover, don't assume.
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            if !entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                continue;
            }
            let candidate = entry.path().join(name);
            if candidate.is_dir() {
                return Some(candidate);
            }
        }
    }

    let candidate = root.join(name);
    if candidate.is_dir() {
        return Some(candidate);
    }

    None
}

struct Metrics {
    loc: u64,
    tests: u64,
    files: u64,
    crates: u64,
}

fn count_metrics(repo_path: &Path) -> Metrics {
    let mut loc = 0u64;
    let mut tests = 0u64;
    let mut files = 0u64;
    let mut crates = 0u64;

    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_entry(|e| !is_hidden_or_target(e))
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }

        if path.file_name().is_some_and(|n| n == "Cargo.toml") {
            crates += 1;
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }

        files += 1;

        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };

        let (file_loc, file_tests) = count_file(&content);
        loc += file_loc;
        tests += file_tests;
    }

    if crates > 1 {
        let root_cargo = repo_path.join("Cargo.toml");
        if let Ok(content) = std::fs::read_to_string(&root_cargo) {
            if content.contains("[workspace]") {
                crates -= 1;
            }
        }
    }

    Metrics {
        loc,
        tests,
        files,
        crates,
    }
}

fn is_hidden_or_target(entry: &walkdir::DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    name.starts_with('.') || name == "target" || name == "node_modules"
}

/// Count source lines (non-blank, non-comment) and test annotations.
pub fn count_file(content: &str) -> (u64, u64) {
    let mut loc = 0u64;
    let mut tests = 0u64;
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if in_block_comment {
            if let Some(pos) = trimmed.find("*/") {
                in_block_comment = false;
                let remainder = trimmed[pos + 2..].trim();
                if !remainder.is_empty() {
                    loc += 1;
                }
            }
            continue;
        }

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if trimmed.starts_with("/*") {
            in_block_comment = !trimmed.contains("*/");
            continue;
        }

        loc += 1;

        if trimmed == "#[test]"
            || trimmed.starts_with("#[tokio::test")
            || trimmed.starts_with("#[async_std::test")
        {
            tests += 1;
        }
    }

    (loc, tests)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_file_basic() {
        let src = r#"
use std::io;

fn main() {
    println!("hello");
}

#[test]
fn it_works() {
    assert!(true);
}

// A comment
/* block comment */

#[test]
fn another() {}
"#;
        let (loc, tests) = count_file(src);
        assert_eq!(tests, 2);
        assert!(loc >= 8, "expected at least 8 LOC, got {loc}");
    }

    #[test]
    fn block_comments_skipped() {
        let src = "/*\n multi \n line \n*/\nfn real() {}";
        let (loc, _) = count_file(src);
        assert_eq!(loc, 1);
    }

    #[test]
    fn format_display_adds_commas() {
        assert_eq!(format_display(0), "0");
        assert_eq!(format_display(999), "999");
        assert_eq!(format_display(1_000), "1,000");
        assert_eq!(format_display(1_234_567), "1,234,567");
    }

    #[test]
    fn find_repo_canonical_path() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let repo = root.join("ecoPrimals/bearDog");
        std::fs::create_dir_all(&repo).unwrap();

        let found = find_repo(root, "ecoPrimals/bearDog");
        assert_eq!(found, Some(repo));
    }

    #[test]
    fn find_repo_flat_fallback() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let repo = root.join("bearDog");
        std::fs::create_dir_all(&repo).unwrap();

        let found = find_repo(root, "ecoPrimals/bearDog");
        assert_eq!(found, Some(repo));
    }

    #[test]
    fn find_repo_returns_none_for_missing() {
        let dir = tempfile::tempdir().unwrap();
        let found = find_repo(dir.path(), "ecoPrimals/nonexistent");
        assert_eq!(found, None);
    }

    #[test]
    fn today_utc_is_valid_format() {
        let date = today_utc();
        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
        assert_eq!(&date[7..8], "-");
    }

    #[test]
    fn count_metrics_on_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let m = count_metrics(dir.path());
        assert_eq!(m.loc, 0);
        assert_eq!(m.tests, 0);
        assert_eq!(m.files, 0);
        assert_eq!(m.crates, 0);
    }

    #[test]
    fn count_metrics_finds_rs_files() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path().join("myrepo");
        let src = repo.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "fn foo() {}\n#[test]\nfn t() {}\n").unwrap();
        std::fs::write(repo.join("Cargo.toml"), "[package]\nname=\"x\"").unwrap();

        let m = count_metrics(&repo);
        assert_eq!(m.files, 1);
        assert_eq!(m.crates, 1);
        assert_eq!(m.tests, 1);
        assert!(m.loc >= 1);
    }

    #[test]
    fn count_metrics_skips_hidden_and_target() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path().join("myrepo");
        let hidden = repo.join(".git");
        let target = repo.join("target");
        std::fs::create_dir_all(&hidden).unwrap();
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(hidden.join("file.rs"), "fn hidden() {}").unwrap();
        std::fs::write(target.join("file.rs"), "fn built() {}").unwrap();

        let m = count_metrics(&repo);
        assert_eq!(m.files, 0);
        assert_eq!(m.loc, 0);
    }

    #[test]
    fn write_updates_modifies_config() {
        let dir = tempfile::tempdir().unwrap();
        let cfg = dir.path().join("config.toml");
        let content = r#"
[extra.entity_registry.testPrimal]
kind = "primal"
loc = 100
loc_display = "100"
tests = 50
tests_display = "50"
files = 5
crates = 1

[extra.totals]
primal_loc = 100
primal_loc_display = "100"
spring_loc = 0
spring_loc_display = "0"
total_loc = 100
total_loc_display = "100"
primal_tests = 50
primal_tests_display = "50"
spring_tests = 0
spring_tests_display = "0"
total_tests = 50
total_tests_display = "50"
measured_date = "2026-01-01"
"#;
        std::fs::write(&cfg, content).unwrap();

        let drifts = vec![Drift {
            key: "testPrimal".into(),
            field: "loc",
            registered: 100,
            actual: 200,
        }];
        write_updates(&cfg, &drifts).unwrap();

        let updated = std::fs::read_to_string(&cfg).unwrap();
        assert!(updated.contains("loc = 200"));
    }

    #[test]
    fn find_repo_walks_subdirectories() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let nested = root.join("infra").join("bearDog");
        std::fs::create_dir_all(&nested).unwrap();

        let found = find_repo(root, "ecoPrimals/bearDog");
        assert_eq!(found, Some(nested));
    }
}
