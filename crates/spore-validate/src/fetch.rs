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

/// Detected forge type — determines archive URL pattern.
///
/// Discovered from the configured forge URL rather than hardcoded.
/// GitHub uses a different archive path than Forgejo/Gitea API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ForgeKind {
    GitHub,
    Forgejo,
}

/// Detect forge type from URL. GitHub-like forges use direct archive paths;
/// Forgejo/Gitea uses the `/api/v1/repos/` pattern.
fn detect_forge_kind(forge_url: &str) -> ForgeKind {
    if forge_url.contains("github.com") || forge_url.contains("github.io") {
        ForgeKind::GitHub
    } else {
        ForgeKind::Forgejo
    }
}

impl Source {
    /// Resolve the clone URL — prefers explicit `origin`, then configured
    /// forge, then GitHub shadow.
    pub fn clone_url(&self) -> String {
        self.origin.as_deref().map_or_else(
            || format!("{}/{}.git", default_forge_url(), self.repo),
            str::to_string,
        )
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
///
/// Used as the default when `git` is available on PATH. For sovereign
/// infrastructure (Forgejo on LAN), prefer `ForgeArchiveBackend` which
/// eliminates the external git dependency entirely.
pub struct GitBackend;

impl GitBackend {
    /// Returns true if `git` is available on PATH.
    pub fn available() -> bool {
        std::process::Command::new("git")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
    }
}

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

/// Forge archive backend — downloads tar.gz archives via HTTP.
///
/// Eliminates the `git` binary dependency entirely. Works with Forgejo
/// and GitHub archive APIs. For sovereign Forgejo on LAN, no TLS needed.
///
/// Archive URL patterns:
/// - Forgejo: `{base}/api/v1/repos/{owner}/{repo}/archive/main.tar.gz`
/// - GitHub: `{base}/{owner}/{repo}/archive/refs/heads/main.tar.gz`
pub struct ForgeArchiveBackend;

impl ForgeArchiveBackend {
    /// Convert a clone URL to an archive download URL.
    ///
    /// Forge type is detected from the configured forge URL — no hardcoded
    /// forge assumptions. GitHub uses direct paths; Forgejo uses API routes.
    fn archive_url(clone_url: &str) -> String {
        let url = clone_url.trim_end_matches(".git");
        let forge_base = default_forge_url();

        match detect_forge_kind(&forge_base) {
            ForgeKind::GitHub => format!("{url}/archive/refs/heads/main.tar.gz"),
            ForgeKind::Forgejo => {
                let path = url.strip_prefix(&forge_base).unwrap_or(url);
                format!("{forge_base}/api/v1/repos{path}/archive/main.tar.gz")
            }
        }
    }

    /// Download and extract a tar.gz archive to the target directory.
    ///
    /// Uses `std::net::TcpStream` for plain HTTP (sovereign Forgejo on LAN).
    /// Does NOT support HTTPS — for extracellular forges, use `GitBackend`.
    fn download_and_extract(url: &str, target: &Path) -> Result<(), Error> {
        let body = http_get_body(url)?;

        // Decompress gzip → tar
        let decompressed = gzip_decompress(&body)?;

        // Create target and extract
        let _ = std::fs::create_dir_all(target);
        extract_tar(&decompressed, target);

        Ok(())
    }
}

impl VcsBackend for ForgeArchiveBackend {
    fn clone_repo(&self, url: &str, target: &Path) -> Result<(), Error> {
        let archive = Self::archive_url(url);
        Self::download_and_extract(&archive, target)
    }

    fn pull_repo(&self, target: &Path) -> Result<(), Error> {
        // For archive backend, "pull" means re-download
        let _ = std::fs::remove_dir_all(target);
        // We don't have the URL here — caller should re-clone
        Err(Error::Git(
            "archive backend does not support incremental pull; use clone".into(),
        ))
    }

    fn is_repo(&self, target: &Path) -> bool {
        // Archive extracts don't have .git — check for content
        target.is_dir() && std::fs::read_dir(target).is_ok_and(|mut d| d.next().is_some())
    }
}

/// Minimal HTTP/1.1 GET — plain TCP, no TLS.
///
/// Suitable for sovereign Forgejo on LAN. For HTTPS forges, use `GitBackend`.
fn http_get_body(url: &str) -> Result<Vec<u8>, Error> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let url = url.strip_prefix("http://").ok_or_else(|| {
        Error::Git(format!("ForgeArchiveBackend only supports plain HTTP: {url}"))
    })?;

    let (host_port, path) = url.split_once('/').unwrap_or((url, "/"));
    let path = format!("/{path}");
    let host_port = if host_port.contains(':') {
        host_port.to_string()
    } else {
        format!("{host_port}:80")
    };

    let host = host_port.split(':').next().unwrap_or("");

    let mut stream = TcpStream::connect(&host_port)
        .map_err(|e| Error::Git(format!("TCP connect to {host_port} failed: {e}")))?;

    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(30)))
        .ok();

    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nAccept: */*\r\n\r\n");
    stream
        .write_all(request.as_bytes())
        .map_err(|e| Error::Git(format!("HTTP write failed: {e}")))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|e| Error::Git(format!("HTTP read failed: {e}")))?;

    // Split headers from body (look for \r\n\r\n)
    let header_end = response
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| Error::Git("malformed HTTP response".into()))?;

    let headers = std::str::from_utf8(&response[..header_end]).unwrap_or("");
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        let status_line = headers.lines().next().unwrap_or("unknown");
        return Err(Error::Git(format!("HTTP error: {status_line}")));
    }

    Ok(response[header_end + 4..].to_vec())
}

/// Gzip decompression using `flate2` (pure Rust via `miniz_oxide`).
fn gzip_decompress(data: &[u8]) -> Result<Vec<u8>, Error> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(data);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| Error::Git(format!("gzip decompress failed: {e}")))?;
    Ok(out)
}

/// Minimal tar extraction — reads POSIX tar headers and writes regular files.
fn extract_tar(data: &[u8], target: &Path) {
    let mut pos = 0;
    let bytes = data;

    while pos + 512 <= bytes.len() {
        let header = &bytes[pos..pos + 512];

        // Empty block = end of archive
        if header.iter().all(|&b| b == 0) {
            break;
        }

        // File name (0..100), stripping the top-level archive directory
        let name_end = header[..100].iter().position(|&b| b == 0).unwrap_or(100);
        let raw_name = std::str::from_utf8(&header[..name_end]).unwrap_or("");

        // Size in octal (124..136)
        let size_str = std::str::from_utf8(&header[124..136])
            .unwrap_or("0")
            .trim_matches(|c: char| c == '\0' || c == ' ');
        let size = usize::from_str_radix(size_str, 8).unwrap_or(0);

        // Type flag (156)
        let type_flag = header[156];

        pos += 512; // Move past header

        // Strip first path component (archive name prefix like "repo-main/")
        let rel_path = raw_name
            .find('/')
            .map_or(raw_name, |i| &raw_name[i + 1..]);

        if !rel_path.is_empty() && (type_flag == b'0' || type_flag == 0) {
            // Regular file
            let file_path = target.join(rel_path);
            if let Some(parent) = file_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if pos + size <= bytes.len() {
                let _ = std::fs::write(&file_path, &bytes[pos..pos + size]);
            }
        } else if !rel_path.is_empty() && type_flag == b'5' {
            // Directory
            let _ = std::fs::create_dir_all(target.join(rel_path));
        }

        // Advance past data (rounded to 512-byte boundary)
        pos += (size + 511) & !511;
    }
}

/// Select the best available VCS backend at runtime.
pub fn detect_backend() -> Box<dyn VcsBackend> {
    if GitBackend::available() {
        Box::new(GitBackend)
    } else {
        Box::new(ForgeArchiveBackend)
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

    let mut keys: Vec<&str> = sources.sources.keys().map(String::as_str).collect();
    keys.sort_unstable();

    let has_pat = std::env::var("SPOREPRINT_REFRESH_PAT").is_ok();

    for key in &keys {
        if source_filter.is_some_and(|f| *key != f) {
            continue;
        }

        let source = &sources.sources[*key];

        if source.private && !has_pat {
            outcomes.push(FetchOutcome::Skipped {
                key: (*key).to_string(),
                reason: "private repo, no SPOREPRINT_REFRESH_PAT".into(),
            });
            continue;
        }

        let target = clone_root.join(&source.repo);
        let kind_label = source.kind.as_deref().unwrap_or("repo");
        let key_owned = (*key).to_string();

        let outcome = if vcs.is_repo(&target) {
            match vcs.pull_repo(&target) {
                Ok(()) => FetchOutcome::Pulled {
                    key: key_owned,
                    kind: kind_label.to_string(),
                },
                Err(e) => FetchOutcome::Skipped {
                    key: key_owned,
                    reason: e.to_string(),
                },
            }
        } else {
            let url = source.clone_url();
            match vcs.clone_repo(&url, &target) {
                Ok(()) => FetchOutcome::Cloned {
                    key: key_owned,
                    kind: kind_label.to_string(),
                },
                Err(e) => FetchOutcome::Skipped {
                    key: key_owned,
                    reason: e.to_string(),
                },
            }
        };

        outcomes.push(outcome);
    }

    outcomes
}

/// Parse `sources.toml` from the sporePrint root.
pub fn parse_sources(sporeprint_root: &Path) -> Result<SourcesFile, Error> {
    let sources_path = sporeprint_root.join(crate::paths::SOURCES_FILE);
    let text = std::fs::read_to_string(&sources_path).map_err(|e| Error::io(&sources_path, e))?;
    let sources: SourcesFile = toml::from_str(&text)?;
    Ok(sources)
}

/// Structured result of a fetch-and-refresh cycle.
pub struct FetchResult {
    pub outcomes: Vec<FetchOutcome>,
    pub clone_root: PathBuf,
}

/// High-level fetch-and-refresh using the best available backend.
///
/// Prefers `git` if available on PATH; falls back to HTTP archive download
/// for sovereign Forgejo (plain HTTP only — no TLS).
pub fn fetch_and_refresh(
    sporeprint_root: &Path,
    source_filter: Option<&str>,
) -> Result<FetchResult, Error> {
    let sources = parse_sources(sporeprint_root)?;
    let clone_root = clone_dir();
    let backend = detect_backend();
    let outcomes = fetch_sources(&sources, &clone_root, source_filter, backend.as_ref());
    Ok(FetchResult {
        outcomes,
        clone_root,
    })
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
            Some(Err(reason)) => Err(Error::Git(reason.clone())),
            Some(Ok(())) | None => Ok(()),
        }
    }

    fn pull_repo(&self, target: &Path) -> Result<(), Error> {
        let key = target.to_string_lossy().to_string();
        match self.pull_results.get(&key) {
            Some(Err(reason)) => Err(Error::Git(reason.clone())),
            Some(Ok(())) | None => Ok(()),
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
        let result = fetch_and_refresh(dir.path(), None);
        assert!(result.is_err(), "missing sources.toml should return Err");
    }
}
