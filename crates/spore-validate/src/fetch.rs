use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize)]
struct SourcesFile {
    sources: HashMap<String, Source>,
}

#[derive(Debug, serde::Deserialize)]
struct Source {
    repo: String,
    #[serde(rename = "type")]
    _kind: Option<String>,
    #[serde(default)]
    _private: bool,
}

fn clone_or_pull(repo: &str, target: &Path) -> Result<(), String> {
    if target.join(".git").is_dir() {
        let status = std::process::Command::new("git")
            .args(["-C", &target.to_string_lossy(), "pull", "--ff-only", "--quiet"])
            .status()
            .map_err(|e| format!("git pull failed: {e}"))?;
        if !status.success() {
            return Err(format!("git pull failed for {repo}"));
        }
    } else {
        if let Some(parent) = target.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let url = format!("https://github.com/{repo}.git");
        let status = std::process::Command::new("git")
            .args(["clone", "--depth", "1", "--quiet", &url, &target.to_string_lossy()])
            .status()
            .map_err(|e| format!("git clone failed: {e}"))?;
        if !status.success() {
            return Err(format!("clone failed for {repo} (may be private)"));
        }
    }
    Ok(())
}

pub fn fetch_and_refresh(
    sporeprint_root: &Path,
    source_filter: Option<&str>,
) -> Vec<String> {
    let sources_path = sporeprint_root.join("sources.toml");
    let clone_dir = std::env::temp_dir().join("sporeprint-refresh");
    let _ = std::fs::create_dir_all(&clone_dir);

    let mut messages = Vec::new();

    let sources_text = match std::fs::read_to_string(&sources_path) {
        Ok(t) => t,
        Err(e) => {
            messages.push(format!("ERROR: failed to read sources.toml: {e}"));
            return messages;
        }
    };

    let sources: SourcesFile = match toml::from_str(&sources_text) {
        Ok(s) => s,
        Err(e) => {
            messages.push(format!("ERROR: failed to parse sources.toml: {e}"));
            return messages;
        }
    };

    let mut keys: Vec<&String> = sources.sources.keys().collect();
    keys.sort();

    for key in &keys {
        if source_filter.is_some_and(|f| key.as_str() != f) {
            continue;
        }

        let source = &sources.sources[key.as_str()];
        let target = clone_dir.join(&source.repo);

        match clone_or_pull(&source.repo, &target) {
            Ok(()) => messages.push(format!("  OK: {key} → {}", source.repo)),
            Err(e) => messages.push(format!("  SKIP: {key} — {e}")),
        }
    }

    messages.push(format!("---\nRepos staged in {}", clone_dir.display()));
    messages.push(clone_dir.to_string_lossy().to_string());
    messages
}

pub fn clone_dir() -> PathBuf {
    std::env::temp_dir().join("sporeprint-refresh")
}
