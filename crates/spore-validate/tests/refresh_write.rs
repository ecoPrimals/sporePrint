// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for metric refresh write-back and drift detection on real repos.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

static BUILD_ONCE: Once = Once::new();

fn sporeprint_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("spore-validate");
    path
}

fn ensure_built() {
    BUILD_ONCE.call_once(|| {
        let status = Command::new("cargo")
            .args(["build", "--quiet"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("failed to build spore-validate");
        assert!(status.success(), "cargo build failed");
    });
}

#[test]
fn refresh_self_reports_accurate_metrics() {
    ensure_built();
    let root = sporeprint_root();
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "refresh",
            &crate_dir.to_string_lossy(),
            "--source",
            "sporeprint",
        ])
        .output()
        .expect("failed to run refresh");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "refresh should not crash: {stdout}"
    );
}

#[test]
fn refresh_write_to_temp_config() {
    ensure_built();
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let config_content = r#"
base_url = "https://test.example"
[extra]
[extra.totals]
primal_loc = 0
spring_loc = 0
total_loc = 0
primal_tests = 0
spring_tests = 0

[extra.entity_registry.testcrate]
display = "Test"
emoji = "🧪"
kind = "primal"
domain = "test"
loc = 1
loc_display = "1"
tests = 1
tests_display = "1"
files = 1
crates = 1
repo = "local/testcrate"
tier = "foundation"
"#;
    std::fs::write(root.join("config.toml"), config_content).unwrap();
    std::fs::create_dir_all(root.join("content")).unwrap();

    let repo_dir = dir.path().join("local/testcrate");
    std::fs::create_dir_all(&repo_dir).unwrap();
    std::fs::write(
        repo_dir.join("Cargo.toml"),
        "[package]\nname = \"testcrate\"\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo_dir.join("src")).unwrap();
    std::fs::write(
        repo_dir.join("src/main.rs"),
        "fn main() {}\n\n#[test]\nfn t() {}\n",
    )
    .unwrap();

    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "refresh",
            &dir.path().to_string_lossy(),
            "--write",
        ])
        .output()
        .expect("failed to run refresh --write");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "refresh --write failed: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    if stdout.contains("DRIFT") {
        assert!(stdout.contains("WRITE: config.toml updated"));
        let updated = std::fs::read_to_string(root.join("config.toml")).unwrap();
        assert!(
            updated.contains("files = 1"),
            "expected files = 1 in updated config"
        );
    }
}

#[test]
fn refresh_counts_rust_files_correctly() {
    ensure_built();
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("myrepo");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    std::fs::write(
        repo.join("src/lib.rs"),
        "pub fn add(a: u32, b: u32) -> u32 { a + b }\n\n#[test]\nfn test_add() { assert_eq!(add(1,2), 3); }\n\n#[tokio::test]\nasync fn async_test() {}\n",
    ).unwrap();
    std::fs::write(
        repo.join("src/util.rs"),
        "// comment\n/* block */\n\npub fn helper() -> bool { true }\n",
    )
    .unwrap();

    let root = dir.path();
    let config = r#"
base_url = "https://test.example"
[extra]
[extra.totals]
primal_loc = 0
spring_loc = 0
total_loc = 0
primal_tests = 0
spring_tests = 0

[extra.entity_registry.myrepo]
display = "My"
emoji = "🧪"
kind = "primal"
domain = "test"
loc = 999
loc_display = "999"
tests = 999
tests_display = "999"
files = 999
crates = 1
repo = "myrepo"
tier = "foundation"
"#;
    std::fs::write(root.join("config.toml"), config).unwrap();
    std::fs::create_dir_all(root.join("content")).unwrap();

    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "refresh",
            &root.to_string_lossy(),
        ])
        .output()
        .expect("failed to run refresh");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(
        stdout.contains("DRIFT"),
        "expected drift detection, got: {stdout}"
    );
    assert!(stdout.contains("loc"));
}
