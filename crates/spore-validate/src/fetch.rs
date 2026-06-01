// SPDX-License-Identifier: AGPL-3.0-or-later

//! Upstream repo fetching for metric refresh.
//!
//! Architecture: trait-based VCS abstraction (`VcsBackend`) enables:
//! - Production: real git operations via `GitBackend`
//! - Testing: in-memory mock via `MockBackend`
//! - Future: Forgejo API, temporal sync, or any other fetch mechanism
//!
//! The fetch module has *self-knowledge only* — it reads `sources.toml` for
//! source declarations and discovers remotes from declared origins, never
//! assuming a specific forge.

use crate::error::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ── Source model ─────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SourcesFile {
    pub sources: HashMap<String, Source>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Source {
    pub repo: String,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    #[serde(default)]
    pub private: bool,
}

/// Default forge URL prefix used when a source has no explicit `origin`.
/// Configurable via `SPOREPRINT_FORGE_URL` environment variable.
/// Falls back to GitHub (extracellular shadow) only as last resort.
fn default_forge_url() -> String {
    std::env::var("SPOREPRINT_FORGE_URL").unwrap_or_else(|_| "https://github.com".to_string())
}

impl Source {
    /// Resolve the clone URL — prefers explicit `origin`, then configured
    /// forge, then GitHub shadow.
    pub fn clone_url(&self) -> String {
        self.origin
            .clone()
            .unwrap_or_else(|| format!("{}/{}.git", default_forge_url(), self.repo))
    }
}

// ── VCS trait ────────────────────────────────────────────────────────

/// Abstraction over version control operations.
///
/// Production uses `GitBackend` (shells out to `git`).
/// Tests use `MockBackend` (in-memory, no I/O).
pub trait VcsBackend {
    fn clone_repo(&self, url: &str, target: &Path) -> Result<(), Error>;
    fn pull_repo(&self, target: &Path) -> Result<(), Error>;
    fn is_repo(&self, target: &Path) -> bool;
}

/// Real git backend — shells out to the `git` binary.
pub struct GitBackend;

impl VcsBackend for GitBackend {
    fn clone_repo(&self, url: &str, target: &Path) -> Result<(), Error> {
        if let Some(parent) = target.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let status = std::process::Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--quiet",
                url,
                &target.to_string_lossy(),
            ])
            .status()
            .map_err(|e| Error::Git(format!("git clone spawn failed: {e}")))?;
        if status.success() {
            Ok(())
        } else {
            Err(Error::Git(format!(
                "clone failed for {url} (may be private or unreachable)"
            )))
        }
    }

    fn pull_repo(&self, target: &Path) -> Result<(), Error> {
        let status = std::process::Command::new("git")
            .args([
                "-C",
                &target.to_string_lossy(),
                "pull",
                "--ff-only",
                "--quiet",
            ])
            .status()
            .map_err(|e| Error::Git(format!("git pull spawn failed: {e}")))?;
        if status.success() {
            Ok(())
        } else {
            Err(Error::Git(format!(
                "git pull failed for {}",
                target.display()
            )))
        }
    }

    fn is_repo(&self, target: &Path) -> bool {
        target.join(".git").is_dir()
    }
}

// ── Fetch orchestration ──────────────────────────────────────────────

/// Outcome of a single source fetch operation.
#[derive(Debug)]
pub enum FetchOutcome {
    Cloned { key: String, kind: String },
    Pulled { key: String, kind: String },
    Skipped { key: String, reason: String },
}

impl std::fmt::Display for FetchOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cloned { key, kind } => write!(f, "  OK: {key} [{kind}] (cloned)"),
            Self::Pulled { key, kind } => write!(f, "  OK: {key} [{kind}] (updated)"),
            Self::Skipped { key, reason } => write!(f, "  SKIP: {key} -- {reason}"),
        }
    }
}

/// Fetch sources using the provided VCS backend.
pub fn fetch_sources(
    sources: &SourcesFile,
    clone_root: &Path,
    source_filter: Option<&str>,
    vcs: &dyn VcsBackend,
) -> Vec<FetchOutcome> {
    let _ = std::fs::create_dir_all(clone_root);
    let mut outcomes = Vec::new();

    let mut keys: Vec<&String> = sources.sources.keys().collect();
    keys.sort_unstable();

    let has_pat = std::env::var("SPOREPRINT_REFRESH_PAT").is_ok();

    for key in &keys {
        if source_filter.is_some_and(|f| key.as_str() != f) {
            continue;
        }

        let source = &sources.sources[key.as_str()];

        if source.private && !has_pat {
            outcomes.push(FetchOutcome::Skipped {
                key: (*key).clone(),
                reason: "private repo, no SPOREPRINT_REFRESH_PAT".into(),
            });
            continue;
        }

        let target = clone_root.join(&source.repo);

        let kind_label = source.kind.as_deref().unwrap_or("repo").to_string();
        let result = if vcs.is_repo(&target) {
            vcs.pull_repo(&target).map(|()| FetchOutcome::Pulled {
                key: (*key).clone(),
                kind: kind_label.clone(),
            })
        } else {
            let url = source.clone_url();
            vcs.clone_repo(&url, &target)
                .map(|()| FetchOutcome::Cloned {
                    key: (*key).clone(),
                    kind: kind_label,
                })
        };

        match result {
            Ok(outcome) => outcomes.push(outcome),
            Err(e) => outcomes.push(FetchOutcome::Skipped {
                key: (*key).clone(),
                reason: e.to_string(),
            }),
        }
    }

    outcomes
}

/// Parse `sources.toml` from the sporePrint root.
pub fn parse_sources(sporeprint_root: &Path) -> Result<SourcesFile, Error> {
    let sources_path = sporeprint_root.join("sources.toml");
    let text = std::fs::read_to_string(&sources_path).map_err(|e| Error::io(&sources_path, e))?;
    let sources: SourcesFile = toml::from_str(&text)?;
    Ok(sources)
}

/// High-level fetch-and-refresh using real git.
pub fn fetch_and_refresh(sporeprint_root: &Path, source_filter: Option<&str>) -> Vec<String> {
    let sources = match parse_sources(sporeprint_root) {
        Ok(s) => s,
        Err(e) => return vec![format!("ERROR: {e}")],
    };

    let clone_root = clone_dir();
    let outcomes = fetch_sources(&sources, &clone_root, source_filter, &GitBackend);

    let mut messages: Vec<String> = outcomes.iter().map(ToString::to_string).collect();
    messages.push(format!("---\nRepos staged in {}", clone_root.display()));
    messages
}

/// Staging directory for fetched repos.
pub fn clone_dir() -> PathBuf {
    std::env::temp_dir().join("sporeprint-refresh")
}

// ── Mock backend for testing ─────────────────────────────────────────

#[cfg(test)]
pub struct MockBackend {
    pub clone_results: HashMap<String, Result<(), String>>,
    pub pull_results: HashMap<String, Result<(), String>>,
    pub existing_repos: Vec<PathBuf>,
}

#[cfg(test)]
impl MockBackend {
    pub fn new() -> Self {
        Self {
            clone_results: HashMap::new(),
            pull_results: HashMap::new(),
            existing_repos: Vec::new(),
        }
    }

    pub fn with_clone_success(mut self, url: &str) -> Self {
        self.clone_results.insert(url.to_string(), Ok(()));
        self
    }

    pub fn with_clone_failure(mut self, url: &str, reason: &str) -> Self {
        self.clone_results
            .insert(url.to_string(), Err(reason.to_string()));
        self
    }

    pub fn with_existing_repo(mut self, path: PathBuf) -> Self {
        self.existing_repos.push(path);
        self
    }
}

#[cfg(test)]
impl VcsBackend for MockBackend {
    fn clone_repo(&self, url: &str, _target: &Path) -> Result<(), Error> {
        match self.clone_results.get(url) {
            Some(Ok(())) => Ok(()),
            Some(Err(reason)) => Err(Error::Git(reason.clone())),
            None => Ok(()),
        }
    }

    fn pull_repo(&self, target: &Path) -> Result<(), Error> {
        let key = target.to_string_lossy().to_string();
        match self.pull_results.get(&key) {
            Some(Ok(())) => Ok(()),
            Some(Err(reason)) => Err(Error::Git(reason.clone())),
            None => Ok(()),
        }
    }

    fn is_repo(&self, target: &Path) -> bool {
        self.existing_repos.iter().any(|p| p == target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_dir_is_deterministic() {
        let a = clone_dir();
        let b = clone_dir();
        assert_eq!(a, b);
        assert!(a.ends_with("sporeprint-refresh"));
    }

    #[test]
    fn sources_file_parses_minimal() {
        let toml_str = r#"
[sources.test]
repo = "org/repo"
"#;
        let sf: SourcesFile = toml::from_str(toml_str).unwrap();
        assert!(sf.sources.contains_key("test"));
        assert_eq!(sf.sources["test"].repo, "org/repo");
    }

    #[test]
    fn sources_file_parses_with_origin() {
        let toml_str = r#"
[sources.test]
repo = "org/repo"
origin = "ssh://git@git.primals.eco:2222/ecoPrimals/repo.git"
"#;
        let sf: SourcesFile = toml::from_str(toml_str).unwrap();
        assert_eq!(
            sf.sources["test"].origin.as_deref(),
            Some("ssh://git@git.primals.eco:2222/ecoPrimals/repo.git")
        );
    }

    #[test]
    fn source_clone_url_uses_origin_when_present() {
        let s = Source {
            repo: "org/repo".into(),
            origin: Some("ssh://custom.git".into()),
            kind: None,
            private: false,
        };
        assert_eq!(s.clone_url(), "ssh://custom.git");
    }

    #[test]
    fn source_clone_url_falls_back_to_github() {
        let s = Source {
            repo: "ecoPrimals/bearDog".into(),
            origin: None,
            kind: None,
            private: false,
        };
        assert_eq!(s.clone_url(), "https://github.com/ecoPrimals/bearDog.git");
    }

    #[test]
    fn mock_backend_clones_successfully() {
        let mock = MockBackend::new().with_clone_success("https://github.com/org/repo.git");

        let sources: SourcesFile = toml::from_str(
            r#"
[sources.myrepo]
repo = "org/repo"
"#,
        )
        .unwrap();

        let dir = tempfile::tempdir().unwrap();
        let outcomes = fetch_sources(&sources, dir.path(), None, &mock);
        assert_eq!(outcomes.len(), 1);
        assert!(matches!(outcomes[0], FetchOutcome::Cloned { .. }));
    }

    #[test]
    fn mock_backend_reports_clone_failure() {
        let mock =
            MockBackend::new().with_clone_failure("https://github.com/org/repo.git", "auth denied");

        let sources: SourcesFile = toml::from_str(
            r#"
[sources.myrepo]
repo = "org/repo"
"#,
        )
        .unwrap();

        let dir = tempfile::tempdir().unwrap();
        let outcomes = fetch_sources(&sources, dir.path(), None, &mock);
        assert_eq!(outcomes.len(), 1);
        assert!(matches!(outcomes[0], FetchOutcome::Skipped { .. }));
        if let FetchOutcome::Skipped { reason, .. } = &outcomes[0] {
            assert!(reason.contains("auth denied"));
        }
    }

    #[test]
    fn mock_backend_pulls_existing_repo() {
        let dir = tempfile::tempdir().unwrap();
        let repo_path = dir.path().join("org/repo");
        let mock = MockBackend::new().with_existing_repo(repo_path);

        let sources: SourcesFile = toml::from_str(
            r#"
[sources.myrepo]
repo = "org/repo"
"#,
        )
        .unwrap();

        let outcomes = fetch_sources(&sources, dir.path(), None, &mock);
        assert_eq!(outcomes.len(), 1);
        assert!(matches!(outcomes[0], FetchOutcome::Pulled { .. }));
    }

    #[test]
    fn source_filter_limits_fetch() {
        let mock = MockBackend::new();
        let sources: SourcesFile = toml::from_str(
            r#"
[sources.alpha]
repo = "org/alpha"
[sources.beta]
repo = "org/beta"
"#,
        )
        .unwrap();

        let dir = tempfile::tempdir().unwrap();
        let outcomes = fetch_sources(&sources, dir.path(), Some("alpha"), &mock);
        assert_eq!(outcomes.len(), 1);
        assert!(matches!(&outcomes[0], FetchOutcome::Cloned { key, .. } if key == "alpha"));
    }

    #[test]
    fn parse_sources_returns_error_on_missing_file() {
        let result = parse_sources(Path::new("/tmp/nonexistent-sporeprint-root"));
        assert!(result.is_err());
    }

    #[test]
    fn fetch_and_refresh_handles_missing_sources() {
        let dir = tempfile::tempdir().unwrap();
        let messages = fetch_and_refresh(dir.path(), None);
        assert!(messages[0].contains("ERROR"));
    }
}
