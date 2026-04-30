use crate::model::{Entity, EntityKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Drift {
    pub key: String,
    pub field: &'static str,
    pub registered: u64,
    pub actual: u64,
}

pub struct RefreshResult {
    pub drifts: Vec<Drift>,
    pub missing_repos: Vec<String>,
    pub scanned: u32,
}

pub fn scan(
    registry: &HashMap<String, Entity>,
    repos_root: &Path,
) -> RefreshResult {
    let mut drifts = Vec::new();
    let mut missing_repos = Vec::new();
    let mut scanned = 0u32;

    let mut keys: Vec<&String> = registry.keys().collect();
    keys.sort();

    for key in keys {
        let entity = &registry[key];
        let Some(repo) = &entity.repo else {
            continue;
        };

        if !matches!(entity.kind, EntityKind::Primal | EntityKind::Spring) {
            if entity.loc.is_none() {
                continue;
            }
        }

        let repo_name = repo.rsplit('/').next().unwrap_or(repo);
        let Some(repo_path) = find_repo(repos_root, repo_name) else {
            missing_repos.push(format!("{key} ({repo})"));
            continue;
        };

        scanned += 1;
        let metrics = count_metrics(&repo_path);

        if let Some(registered_loc) = entity.loc {
            if registered_loc != metrics.loc {
                drifts.push(Drift {
                    key: key.clone(),
                    field: "loc",
                    registered: registered_loc,
                    actual: metrics.loc,
                });
            }
        }

        if let Some(registered_tests) = entity.tests {
            if registered_tests != metrics.tests {
                drifts.push(Drift {
                    key: key.clone(),
                    field: "tests",
                    registered: registered_tests,
                    actual: metrics.tests,
                });
            }
        }

        if let Some(registered_files) = entity.files {
            if u64::from(registered_files) != metrics.files {
                drifts.push(Drift {
                    key: key.clone(),
                    field: "files",
                    registered: u64::from(registered_files),
                    actual: metrics.files,
                });
            }
        }

        if let Some(registered_crates) = entity.crates {
            if u64::from(registered_crates) != metrics.crates {
                drifts.push(Drift {
                    key: key.clone(),
                    field: "crates",
                    registered: u64::from(registered_crates),
                    actual: metrics.crates,
                });
            }
        }
    }

    RefreshResult {
        drifts,
        missing_repos,
        scanned,
    }
}

fn find_repo(root: &Path, name: &str) -> Option<PathBuf> {
    for subdir in &["primals", "springs", "infra", "gardens", ""] {
        let candidate = if subdir.is_empty() {
            root.join(name)
        } else {
            root.join(subdir).join(name)
        };
        if candidate.is_dir() {
            return Some(candidate);
        }
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

        if !path.extension().is_some_and(|ext| ext == "rs") {
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

    // Workspace Cargo.toml isn't a crate itself, only member crates count.
    // If there's a top-level Cargo.toml with [workspace], subtract 1 for
    // the workspace root.
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
fn count_file(content: &str) -> (u64, u64) {
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

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("//") {
            continue;
        }

        if trimmed.starts_with("/*") {
            in_block_comment = true;
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
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
}
