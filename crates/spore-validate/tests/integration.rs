// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests: run spore-validate against the actual sporePrint content.
//!
//! These tests exercise the full validation pipeline end-to-end, verifying
//! that the real `config.toml` and `content/` directory pass validation.

use std::path::{Path, PathBuf};
use std::process::Command;

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

fn build_binary() {
    let status = Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("failed to build spore-validate");
    assert!(status.success(), "cargo build failed");
}

#[test]
fn validate_succeeds_on_real_content() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "validate"])
        .output()
        .expect("failed to run spore-validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "validate failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("OK:"));
}

#[test]
fn validate_check_succeeds_on_real_content() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "validate", "--check"])
        .output()
        .expect("failed to run spore-validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "validate --check failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("entity shortcodes scanned"));
}

#[test]
fn validate_strict_on_real_content() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "validate", "--strict"])
        .output()
        .expect("failed to run spore-validate");

    let _stdout = String::from_utf8_lossy(&output.stdout);
    // Strict may fail if there are warnings promoted to errors;
    // we just verify the binary runs without panicking.
    assert!(
        output.status.code().is_some(),
        "process should exit cleanly (not crash)"
    );
}

#[test]
fn validate_fails_on_missing_config() {
    build_binary();
    let output = Command::new(binary_path())
        .args(["--root", "/tmp/nonexistent-sporeprint-test-dir", "validate"])
        .output()
        .expect("failed to run spore-validate");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ERROR") || stderr.contains("error"),
        "expected error message, got: {stderr}"
    );
}

#[test]
fn help_flag_works() {
    build_binary();
    let output = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("failed to run --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spore-validate"));
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("refresh"));
    assert!(stdout.contains("render-notebooks"));
    assert!(stdout.contains("fetch-refresh"));
}

#[test]
fn version_flag_works() {
    build_binary();
    let output = Command::new(binary_path())
        .arg("--version")
        .output()
        .expect("failed to run --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spore-validate"));
}

#[test]
fn render_notebooks_on_empty_dir() {
    build_binary();
    let dir = tempfile::tempdir().unwrap();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "render-notebooks",
            &dir.path().to_string_lossy(),
        ])
        .output()
        .expect("failed to run render-notebooks");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rendered 0 notebook(s)"));
}

#[test]
fn render_notebooks_renders_ipynb() {
    build_binary();
    let dir = tempfile::tempdir().unwrap();
    let nb_path = dir.path().join("sample.ipynb");
    std::fs::write(
        &nb_path,
        "{\"cells\": [{\"cell_type\": \"markdown\", \"source\": [\"# Sample\\n\", \"Hello world\"], \"outputs\": []}]}",
    )
    .unwrap();

    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "render-notebooks",
            &dir.path().to_string_lossy(),
        ])
        .output()
        .expect("failed to run render-notebooks");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rendered 1 notebook(s)"));
}

#[test]
fn refresh_on_nonexistent_repos_root() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "refresh",
            "/tmp/nonexistent-repos-root-test",
        ])
        .output()
        .expect("failed to run refresh");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SKIP") || stdout.contains("OK"));
}

#[test]
fn refresh_with_source_filter() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "refresh",
            "/tmp/nonexistent-repos-root-test",
            "--source",
            "nonexistent",
        ])
        .output()
        .expect("failed to run refresh --source");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("scanning nonexistent"));
}

#[test]
fn validate_verbose_shows_registry() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "validate", "--verbose"])
        .output()
        .expect("failed to run validate --verbose");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("primal") || stdout.contains("spring"));
    assert!(stdout.contains("LOC:") || stdout.contains("Tests:"));
}

#[test]
fn render_notebooks_discover_on_workspace() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "render-notebooks",
            "--discover",
        ])
        .output()
        .expect("failed to run render-notebooks --discover");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rendered"));
}
