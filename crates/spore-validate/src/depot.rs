// SPDX-License-Identifier: AGPL-3.0-or-later

//! Depot integrity verification — validates binary artifacts against BLAKE3
//! checksums from `checksums.toml`.
//!
//! Used post-fetch to prove that depot binaries match the build authority's
//! signed manifest. Supports per-architecture verification and size checks.

use crate::error::Error;
use std::collections::BTreeMap;
use std::path::Path;

/// A single primal's expected checksum and size for a given architecture.
#[derive(Debug, Clone)]
pub struct BinaryEntry {
    pub blake3: String,
    pub size: u64,
}

/// Parsed depot checksums manifest, keyed by architecture then primal name.
pub type DepotChecksums = BTreeMap<String, BTreeMap<String, BinaryEntry>>;

/// Result of verifying a single binary.
#[derive(Debug)]
pub enum VerifyStatus {
    Match,
    HashMismatch { expected: String, actual: String },
    SizeMismatch { expected: u64, actual: u64 },
    Missing,
    ReadError(String),
}

/// Aggregate result of depot verification.
#[derive(Debug)]
pub struct VerifyResult {
    pub arch: String,
    pub entries: Vec<(String, VerifyStatus)>,
}

impl VerifyResult {
    pub fn all_present_valid(&self) -> bool {
        self.entries.iter().all(|(_, s)| {
            matches!(s, VerifyStatus::Match | VerifyStatus::Missing)
        })
    }

    pub fn match_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, s)| matches!(s, VerifyStatus::Match))
            .count()
    }

    pub fn total(&self) -> usize {
        self.entries.len()
    }
}

/// Parse a `checksums.toml` file into a structured manifest.
///
/// Format:
/// ```toml
/// [x86_64-unknown-linux-musl]
/// beardog = { blake3 = "abcd...", size = 12345 }
/// ```
pub fn parse_checksums(path: &Path) -> Result<DepotChecksums, Error> {
    let text = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    parse_checksums_str(&text)
}

fn parse_checksums_str(text: &str) -> Result<DepotChecksums, Error> {
    let table: toml::Value = toml::from_str(text)
        .map_err(|e| Error::Config(format!("checksums.toml parse error: {e}")))?;

    let root = table
        .as_table()
        .ok_or_else(|| Error::Config("checksums.toml root is not a table".into()))?;

    let mut result = BTreeMap::new();

    for (arch, arch_val) in root {
        let arch_table = arch_val
            .as_table()
            .ok_or_else(|| Error::Config(format!("[{arch}] is not a table")))?;

        let mut entries = BTreeMap::new();

        for (primal, entry_val) in arch_table {
            let entry_table = entry_val
                .as_table()
                .ok_or_else(|| Error::Config(format!("{arch}/{primal} is not a table")))?;

            let blake3 = entry_table
                .get("blake3")
                .and_then(toml::Value::as_str)
                .ok_or_else(|| Error::Config(format!("{arch}/{primal} missing blake3 field")))?
                .to_string();

            let size = entry_table
                .get("size")
                .and_then(toml::Value::as_integer)
                .ok_or_else(|| Error::Config(format!("{arch}/{primal} missing size field")))?;

            #[allow(clippy::cast_sign_loss)]
            let size = size as u64;

            entries.insert(primal.clone(), BinaryEntry { blake3, size });
        }

        result.insert(arch.clone(), entries);
    }

    Ok(result)
}

/// Verify all binaries in a local depot directory against checksums for the
/// specified architecture.
///
/// `depot_dir` should contain bare binary files named by primal slug
/// (e.g., `beardog`, `songbird`).
pub fn verify_depot(
    checksums: &DepotChecksums,
    arch: &str,
    depot_dir: &Path,
) -> Result<VerifyResult, Error> {
    let arch_entries = checksums
        .get(arch)
        .ok_or_else(|| Error::Config(format!("architecture '{arch}' not found in checksums")))?;

    let mut entries = Vec::with_capacity(arch_entries.len());

    for (primal, expected) in arch_entries {
        let binary_path = depot_dir.join(primal);
        let status = verify_single_binary(&binary_path, expected);
        entries.push((primal.clone(), status));
    }

    Ok(VerifyResult {
        arch: arch.to_string(),
        entries,
    })
}

fn verify_single_binary(path: &Path, expected: &BinaryEntry) -> VerifyStatus {
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return VerifyStatus::Missing,
        Err(e) => return VerifyStatus::ReadError(e.to_string()),
    };

    if metadata.len() != expected.size {
        return VerifyStatus::SizeMismatch {
            expected: expected.size,
            actual: metadata.len(),
        };
    }

    match compute_blake3(path) {
        Ok(hash) => {
            if hash == expected.blake3 {
                VerifyStatus::Match
            } else {
                VerifyStatus::HashMismatch {
                    expected: expected.blake3.clone(),
                    actual: hash,
                }
            }
        }
        Err(e) => VerifyStatus::ReadError(e),
    }
}

/// Compute BLAKE3 hash of a file using streaming reads (handles large binaries).
fn compute_blake3(path: &Path) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("open: {e}"))?;
    let mut hasher = blake3::Hasher::new();
    hasher.update_reader(file).map_err(|e| format!("read: {e}"))?;
    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CHECKSUMS: &str = r#"
[x86_64-unknown-linux-musl]
beardog = { blake3 = "9a3d81b0d5940fd099f8600a22f4a2540c2e81aa9154846e4f98ef19ccd90a5d", size = 11215648 }
songbird = { blake3 = "06efe0291924fc7b37bf457943c8740e5cddc2e2388f654af840f9a5ccca4e5b", size = 17652800 }

[aarch64-unknown-linux-musl]
beardog = { blake3 = "896a363eadf35486ec93b9a92b6436e47d3a19f1cc6da46f567a45f703326116", size = 8937696 }
"#;

    #[test]
    fn parse_checksums_valid() {
        let result = parse_checksums_str(SAMPLE_CHECKSUMS).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("x86_64-unknown-linux-musl"));
        assert!(result.contains_key("aarch64-unknown-linux-musl"));

        let x86 = &result["x86_64-unknown-linux-musl"];
        assert_eq!(x86.len(), 2);
        assert_eq!(x86["beardog"].size, 11_215_648);
        assert_eq!(
            x86["beardog"].blake3,
            "9a3d81b0d5940fd099f8600a22f4a2540c2e81aa9154846e4f98ef19ccd90a5d"
        );
    }

    #[test]
    fn parse_checksums_empty_arch() {
        let toml = "[x86_64-unknown-linux-musl]\n";
        let result = parse_checksums_str(toml).unwrap();
        assert!(result["x86_64-unknown-linux-musl"].is_empty());
    }

    #[test]
    fn parse_checksums_missing_blake3_errors() {
        let toml = r"
[x86_64-unknown-linux-musl]
beardog = { size = 100 }
";
        let result = parse_checksums_str(toml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing blake3"));
    }

    #[test]
    fn parse_checksums_missing_size_errors() {
        let toml = r#"
[x86_64-unknown-linux-musl]
beardog = { blake3 = "abc123" }
"#;
        let result = parse_checksums_str(toml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing size"));
    }

    #[test]
    fn verify_depot_missing_arch_errors() {
        let checksums = parse_checksums_str(SAMPLE_CHECKSUMS).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let result = verify_depot(&checksums, "riscv64-unknown-linux-musl", dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn verify_depot_missing_binary() {
        let checksums = parse_checksums_str(SAMPLE_CHECKSUMS).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let result = verify_depot(&checksums, "x86_64-unknown-linux-musl", dir.path()).unwrap();
        assert!(result.all_present_valid());
        assert_eq!(result.match_count(), 0);
        assert!(matches!(result.entries[0].1, VerifyStatus::Missing));
    }

    #[test]
    fn verify_depot_size_mismatch() {
        let checksums = parse_checksums_str(SAMPLE_CHECKSUMS).unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("beardog"), b"wrong size").unwrap();
        let result = verify_depot(&checksums, "x86_64-unknown-linux-musl", dir.path()).unwrap();

        let beardog_status = result.entries.iter().find(|(k, _)| k == "beardog").unwrap();
        assert!(matches!(beardog_status.1, VerifyStatus::SizeMismatch { .. }));
    }

    #[test]
    fn verify_depot_correct_binary() {
        let content = b"hello depot verification test";
        let hash = blake3::hash(content).to_hex().to_string();

        let toml_str = format!(
            "[test-arch]\ntest_primal = {{ blake3 = \"{hash}\", size = {} }}\n",
            content.len()
        );

        let checksums = parse_checksums_str(&toml_str).unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_primal"), content).unwrap();

        let result = verify_depot(&checksums, "test-arch", dir.path()).unwrap();
        assert!(result.all_present_valid());
        assert_eq!(result.match_count(), 1);
    }

    #[test]
    fn verify_depot_hash_mismatch() {
        let toml_str = r#"
[test-arch]
test_primal = { blake3 = "0000000000000000000000000000000000000000000000000000000000000000", size = 5 }
"#;
        let checksums = parse_checksums_str(toml_str).unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_primal"), b"hello").unwrap();

        let result = verify_depot(&checksums, "test-arch", dir.path()).unwrap();
        assert!(!result.all_present_valid());

        let status = &result.entries[0].1;
        assert!(matches!(status, VerifyStatus::HashMismatch { .. }));
    }

    #[test]
    fn compute_blake3_matches_library() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test");
        let content = b"sporePrint depot verification";
        std::fs::write(&path, content).unwrap();

        let expected = blake3::hash(content).to_hex().to_string();
        let actual = compute_blake3(&path).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn verify_result_counts() {
        let result = VerifyResult {
            arch: "test".into(),
            entries: vec![
                ("a".into(), VerifyStatus::Match),
                ("b".into(), VerifyStatus::Match),
                ("c".into(), VerifyStatus::Missing),
            ],
        };
        assert_eq!(result.match_count(), 2);
        assert_eq!(result.total(), 3);
        assert!(result.all_present_valid());
    }

    #[test]
    fn verify_result_fails_on_corruption() {
        let result = VerifyResult {
            arch: "test".into(),
            entries: vec![
                ("a".into(), VerifyStatus::Match),
                ("b".into(), VerifyStatus::HashMismatch {
                    expected: "aaa".into(),
                    actual: "bbb".into(),
                }),
            ],
        };
        assert!(!result.all_present_valid());
    }
}
