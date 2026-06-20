// SPDX-License-Identifier: AGPL-3.0-or-later

//! Depot-related command handlers: verification, inventory, and discovery.
//!
//! Extracted from `commands.rs` for cohesion — all functions here interact with
//! `plasmidBin` checksums and the binary depot.

use crate::{depot, error::Error};
use std::path::{Path, PathBuf};

/// Print depot state as part of the `discover` command output.
///
/// Locates `checksums.toml` by environment variable or workspace walk,
/// then reports architectures, binary counts, and manifest freshness.
pub fn print_discovery() {
    println!();
    println!("  DEPOT:");

    let Some(path) = discover_checksums_path() else {
        println!("    (plasmidBin/checksums.toml not found — set PLASMIDBIN_CHECKSUMS or place in workspace)");
        return;
    };

    println!("    manifest: {}", path.display());

    let Ok(checksums) = depot::parse_checksums(&path) else {
        println!("    (failed to parse checksums.toml)");
        return;
    };

    for (arch, entries) in &checksums {
        println!("    {arch}: {} binaries", entries.len());
    }

    if let Ok(metadata) = std::fs::metadata(&path) {
        use std::time::SystemTime;
        if let Ok(modified) = metadata.modified() {
            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap_or_default();
            let hours = age.as_secs() / 3600;
            if hours < 24 {
                println!("    freshness: {hours}h ago ✅");
            } else {
                let days = hours / 24;
                println!("    freshness: {days}d ago ⚠️");
            }
        }
    }
}

/// Find `checksums.toml` by checking env var, then walking up to workspace.
fn discover_checksums_path() -> Option<PathBuf> {
    discover_checksums_from(
        std::env::var("PLASMIDBIN_CHECKSUMS").ok().as_deref(),
        &std::env::current_dir().ok()?,
    )
}

/// Inner logic separated from env access for testability.
fn discover_checksums_from(env_path: Option<&str>, start_dir: &Path) -> Option<PathBuf> {
    if let Some(path) = env_path {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }

    let mut dir = start_dir.to_path_buf();
    loop {
        let candidate = dir.join("infra/plasmidBin/checksums.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        let candidate = dir.join("plasmidBin/checksums.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// List available architectures and binary inventory from `checksums.toml`.
pub fn list_arches(checksums_path: &Path) -> Result<(), Error> {
    println!("spore-validate: depot inventory");
    println!("  checksums: {}", checksums_path.display());
    println!();

    let checksums = depot::parse_checksums(checksums_path)?;

    if checksums.is_empty() {
        println!("  (no architectures found)");
        return Ok(());
    }

    for (arch, entries) in &checksums {
        #[allow(clippy::cast_precision_loss)]
        let total_size: f64 =
            entries.values().map(|e| e.size).sum::<u64>() as f64 / (1024.0 * 1024.0);
        println!("  {arch}");
        println!("    binaries: {}", entries.len());
        println!("    total size: {total_size:.1} MB");
        let mut names: Vec<&str> = entries.keys().map(String::as_str).collect();
        names.sort_unstable();
        println!("    primals: {}", names.join(", "));
        println!();
    }

    Ok(())
}

/// Verify depot binary integrity against BLAKE3 checksums.
///
/// Returns `Ok(())` when all checks pass, `Err` with failure counts otherwise.
/// In `partial` mode, missing binaries are warnings (pass if all present verify).
pub fn verify(
    checksums_path: &Path,
    depot_dir: &Path,
    arch: &str,
    partial: bool,
) -> Result<(), Error> {
    println!("spore-validate: depot integrity verification");
    println!("  checksums: {}", checksums_path.display());
    println!("  depot:     {}", depot_dir.display());
    println!("  arch:      {arch}");
    if partial {
        println!("  mode:      partial (missing binaries are warnings, not errors)");
    }
    println!();

    let checksums = depot::parse_checksums(checksums_path)?;

    if !depot_dir.is_dir() {
        return Err(Error::Config(format!(
            "depot directory not found: {}",
            depot_dir.display()
        )));
    }

    let result = depot::verify_depot(&checksums, arch, depot_dir)?;
    println!("  Verifying {} binaries for {}:", result.total(), result.arch);

    let mut hard_failures = 0usize;
    let mut missing_count = 0usize;

    for (primal, status) in &result.entries {
        match status {
            depot::VerifyStatus::Match => {
                println!("    ✅ {primal}");
            }
            depot::VerifyStatus::HashMismatch { expected, actual } => {
                println!("    ❌ {primal} — BLAKE3 mismatch");
                println!("         expected: {expected}");
                println!("         actual:   {actual}");
                hard_failures += 1;
            }
            depot::VerifyStatus::SizeMismatch { expected, actual } => {
                println!("    ❌ {primal} — size mismatch (expected {expected}, got {actual})");
                hard_failures += 1;
            }
            depot::VerifyStatus::Missing => {
                let icon = if partial { "⚠️" } else { "❌" };
                println!("    {icon} {primal} — not found in depot");
                missing_count += 1;
            }
            depot::VerifyStatus::ReadError(e) => {
                println!("    ❌ {primal} — read error: {e}");
                hard_failures += 1;
            }
        }
    }

    println!();
    println!(
        "  RESULT: {}/{} verified{}",
        result.match_count(),
        result.total(),
        if missing_count > 0 {
            format!(" ({missing_count} missing)")
        } else {
            String::new()
        }
    );

    let pass = if partial {
        result.all_present_valid()
    } else {
        hard_failures == 0 && missing_count == 0
    };

    if pass {
        println!("  ✅ DEPOT INTEGRITY VERIFIED");
        Ok(())
    } else {
        let effective_failures = hard_failures + if partial { 0 } else { missing_count };
        println!("  ❌ {effective_failures} INTEGRITY FAILURE(S)");
        Err(Error::ValidationFailed {
            error_count: effective_failures,
            warning_count: if partial { missing_count } else { 0 },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_from_env_var_when_file_exists() {
        let dir = tempfile::tempdir().unwrap();
        let checksums = dir.path().join("checksums.toml");
        std::fs::write(&checksums, "[test]\n").unwrap();

        let result = discover_checksums_from(Some(checksums.to_str().unwrap()), dir.path());
        assert_eq!(result, Some(checksums));
    }

    #[test]
    fn discover_from_env_var_ignores_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let result = discover_checksums_from(Some("/nonexistent/path.toml"), dir.path());
        assert_eq!(result, None);
    }

    #[test]
    fn discover_walks_up_to_infra_plasmibin() {
        let dir = tempfile::tempdir().unwrap();
        let depot_dir = dir.path().join("infra/plasmidBin");
        std::fs::create_dir_all(&depot_dir).unwrap();
        let checksums = depot_dir.join("checksums.toml");
        std::fs::write(&checksums, "[test]\n").unwrap();

        let nested = dir.path().join("a/b/c");
        std::fs::create_dir_all(&nested).unwrap();

        let result = discover_checksums_from(None, &nested);
        assert_eq!(result, Some(checksums));
    }

    #[test]
    fn discover_walks_up_to_plasmibin() {
        let dir = tempfile::tempdir().unwrap();
        let depot_dir = dir.path().join("plasmidBin");
        std::fs::create_dir_all(&depot_dir).unwrap();
        let checksums = depot_dir.join("checksums.toml");
        std::fs::write(&checksums, "[test]\n").unwrap();

        let nested = dir.path().join("sub");
        std::fs::create_dir_all(&nested).unwrap();

        let result = discover_checksums_from(None, &nested);
        assert_eq!(result, Some(checksums));
    }

    #[test]
    fn discover_returns_none_when_no_checksums_found() {
        let dir = tempfile::tempdir().unwrap();
        let result = discover_checksums_from(None, dir.path());
        assert_eq!(result, None);
    }

    #[test]
    fn verify_full_mode_fails_on_missing() {
        let dir = tempfile::tempdir().unwrap();
        let checksums_path = dir.path().join("checksums.toml");
        std::fs::write(
            &checksums_path,
            "[test-arch]\nfoo = { blake3 = \"abc\", size = 5 }\n",
        )
        .unwrap();

        let depot = dir.path().join("depot");
        std::fs::create_dir_all(&depot).unwrap();

        let result = verify(&checksums_path, &depot, "test-arch", false);
        assert!(result.is_err());
    }

    #[test]
    fn verify_partial_mode_passes_on_missing() {
        let dir = tempfile::tempdir().unwrap();
        let checksums_path = dir.path().join("checksums.toml");
        std::fs::write(
            &checksums_path,
            "[test-arch]\nfoo = { blake3 = \"abc\", size = 5 }\n",
        )
        .unwrap();

        let depot = dir.path().join("depot");
        std::fs::create_dir_all(&depot).unwrap();

        let result = verify(&checksums_path, &depot, "test-arch", true);
        assert!(result.is_ok());
    }

    #[test]
    fn list_arches_empty_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let checksums_path = dir.path().join("checksums.toml");
        std::fs::write(&checksums_path, "").unwrap();

        let result = list_arches(&checksums_path);
        assert!(result.is_ok());
    }
}
