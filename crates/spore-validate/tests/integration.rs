// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests: run spore-validate against the actual sporePrint content.
//!
//! These tests exercise the full validation pipeline end-to-end, verifying
//! that the real `config.toml` and `content/` directory pass validation.

mod common;

use common::{binary_path, ensure_built, sporeprint_root};
use std::process::{Command, Output};

/// Run the spore-validate binary with the given arguments against the real root.
fn run(args: &[&str]) -> Output {
    ensure_built();
    let root = sporeprint_root();
    Command::new(binary_path())
        .args(["--root", &root.to_string_lossy()])
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run spore-validate {}: {e}", args.join(" ")))
}

/// Run the binary with raw args (no implicit --root).
fn run_raw(args: &[&str]) -> Output {
    ensure_built();
    Command::new(binary_path())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run spore-validate {}: {e}", args.join(" ")))
}

fn stdout_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context} failed:\nstdout: {}\nstderr: {}",
        stdout_of(output),
        stderr_of(output)
    );
}

// ── Validate ─────────────────────────────────────────────────────────

#[test]
fn validate_succeeds_on_real_content() {
    let output = run(&["validate"]);
    assert_success(&output, "validate");
    assert!(stdout_of(&output).contains("OK:"));
}

#[test]
fn validate_check_succeeds_on_real_content() {
    let output = run(&["validate", "--check"]);
    assert_success(&output, "validate --check");
    assert!(stdout_of(&output).contains("entity shortcodes scanned"));
}

#[test]
fn validate_strict_on_real_content() {
    let output = run(&["validate", "--strict"]);
    assert!(
        output.status.code().is_some(),
        "process should exit cleanly (not crash)"
    );
}

#[test]
fn validate_fails_on_missing_config() {
    let output = run_raw(&["--root", "/tmp/nonexistent-sporeprint-test-dir", "validate"]);
    assert!(!output.status.success());
    let stderr = stderr_of(&output);
    assert!(
        stderr.contains("ERROR") || stderr.contains("error"),
        "expected error message, got: {stderr}"
    );
}

#[test]
fn validate_verbose_shows_registry() {
    let output = run(&["validate", "--verbose"]);
    assert_success(&output, "validate --verbose");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("primal") || stdout.contains("spring"));
    assert!(stdout.contains("LOC:") || stdout.contains("Tests:"));
}

// ── CLI Meta ─────────────────────────────────────────────────────────

#[test]
fn help_flag_works() {
    let output = run_raw(&["--help"]);
    assert!(output.status.success());
    let stdout = stdout_of(&output);
    assert!(stdout.contains("spore-validate"));
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("refresh"));
    assert!(stdout.contains("render-notebooks"));
    assert!(stdout.contains("fetch-refresh"));
}

#[test]
fn version_flag_works() {
    let output = run_raw(&["--version"]);
    assert!(output.status.success());
    assert!(stdout_of(&output).contains("spore-validate"));
}

// ── Notebooks ────────────────────────────────────────────────────────

#[test]
fn render_notebooks_on_empty_dir() {
    let dir = tempfile::tempdir().unwrap();
    let output = run(&["render-notebooks", &dir.path().to_string_lossy()]);
    assert_success(&output, "render-notebooks (empty)");
    assert!(stdout_of(&output).contains("Rendered 0 notebook(s)"));
}

#[test]
fn render_notebooks_renders_ipynb() {
    let dir = tempfile::tempdir().unwrap();
    let out_dir = tempfile::tempdir().unwrap();
    let nb_path = dir.path().join("test_fixture.ipynb");
    std::fs::write(
        &nb_path,
        r##"{"cells": [{"cell_type": "markdown", "source": ["# Sample\n", "Hello world"], "outputs": []}]}"##,
    )
    .unwrap();

    let output = Command::new(binary_path())
        .args(["--root", &sporeprint_root().to_string_lossy()])
        .args(["render-notebooks", &dir.path().to_string_lossy()])
        .env("SPOREPRINT_NOTEBOOK_OUTPUT", out_dir.path())
        .output()
        .expect("render-notebooks (ipynb)");
    assert_success(&output, "render-notebooks (ipynb)");
    assert!(stdout_of(&output).contains("Rendered 1 notebook(s)"));
}

#[test]
fn render_notebooks_discover_on_workspace() {
    let tmp = tempfile::tempdir().unwrap();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy()])
        .args(["render-notebooks", "--discover"])
        .env("SPOREPRINT_NOTEBOOK_OUTPUT", tmp.path())
        .output()
        .expect("render-notebooks --discover");
    assert_success(&output, "render-notebooks --discover");
    assert!(stdout_of(&output).contains("Rendered"));
}

// ── Refresh ──────────────────────────────────────────────────────────

#[test]
fn refresh_on_nonexistent_repos_root() {
    let output = run(&["refresh", "/tmp/nonexistent-repos-root-test"]);
    assert_success(&output, "refresh (nonexistent root)");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("SKIP") || stdout.contains("OK"));
}

#[test]
fn refresh_with_source_filter() {
    let output = run(&[
        "refresh",
        "/tmp/nonexistent-repos-root-test",
        "--source",
        "nonexistent",
    ]);
    assert_success(&output, "refresh --source");
    assert!(stdout_of(&output).contains("scanning nonexistent"));
}

// ── Links ────────────────────────────────────────────────────────────

#[test]
fn check_links_on_real_content() {
    let output = run(&["check-links"]);
    assert_success(&output, "check-links");
    assert!(stdout_of(&output).contains("checking internal links"));
}

// ── Graph ────────────────────────────────────────────────────────────

#[test]
fn graph_without_emit_runs_cleanly() {
    let output = run(&["graph"]);
    assert_success(&output, "graph");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("nodes"));
    assert!(stdout.contains("edges"));
}

#[test]
fn graph_emit_produces_json() {
    let dir = tempfile::tempdir().unwrap();
    let temp_root = dir.path();

    std::fs::copy(
        sporeprint_root().join("config.toml"),
        temp_root.join("config.toml"),
    )
    .unwrap();
    std::fs::create_dir_all(temp_root.join("static/graph")).unwrap();

    let output = run_raw(&["--root", &temp_root.to_string_lossy(), "graph", "--emit"]);
    assert_success(&output, "graph --emit");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("EMIT:"));
    assert!(temp_root.join("static/graph/entity-graph.json").exists());
}

// ── Certify ──────────────────────────────────────────────────────────

#[test]
fn certify_without_emit_validates() {
    let output = run(&["certify"]);
    assert_success(&output, "certify");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("entities:"));
    assert!(stdout.contains("content pages:"));
}

#[test]
fn certify_emit_produces_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let temp_root = dir.path();

    std::fs::copy(
        sporeprint_root().join("config.toml"),
        temp_root.join("config.toml"),
    )
    .unwrap();
    std::fs::create_dir_all(temp_root.join("content")).unwrap();
    std::fs::create_dir_all(temp_root.join("static/certification")).unwrap();

    let output = run_raw(&["--root", &temp_root.to_string_lossy(), "certify", "--emit"]);
    assert_success(&output, "certify --emit");
    assert!(stdout_of(&output).contains("entities:"));
    assert!(
        temp_root
            .join("static/certification/manifest.json")
            .exists()
    );
}

// ── Provenance ───────────────────────────────────────────────────────
//
// Write + verify are sequential operations on the same manifest file.
// Combined into a single test to eliminate the parallel write race.

#[test]
fn provenance_write_and_verify() {
    let output = run(&["provenance", "--write"]);
    assert_success(&output, "provenance --write");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("BLAKE3"));
    assert!(sporeprint_root().join("content-manifest.toml").exists());

    let output = run(&["provenance", "--verify"]);
    assert_success(&output, "provenance --verify");
}

// ── CAS Manifest ─────────────────────────────────────────────────────

#[test]
fn cas_manifest_on_temp_dir() {
    let dir = tempfile::tempdir().unwrap();
    let public = dir.path().join("public");
    std::fs::create_dir(&public).unwrap();
    std::fs::write(public.join("index.html"), "<html>test</html>").unwrap();
    std::fs::write(public.join("style.css"), "body{}").unwrap();

    let output = run(&["cas-manifest", "--public-dir", &public.to_string_lossy()]);
    assert_success(&output, "cas-manifest");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("files: 2"));
    assert!(stdout.contains("HTML pages: 1"));
    assert!(stdout.contains("build hash: blake3:"));
}

#[test]
fn cas_manifest_emit_writes_json() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let public = root.join("public");
    std::fs::create_dir(&public).unwrap();
    std::fs::write(public.join("page.html"), "<h1>hi</h1>").unwrap();
    std::fs::create_dir_all(root.join("static/cas")).unwrap();
    std::fs::copy(
        sporeprint_root().join("config.toml"),
        root.join("config.toml"),
    )
    .unwrap();

    let output = run_raw(&["--root", &root.to_string_lossy(), "cas-manifest", "--emit"]);
    assert_success(&output, "cas-manifest --emit");
    assert!(stdout_of(&output).contains("EMIT:"));

    let manifest_path = root.join("static/cas/build-manifest.json");
    assert!(manifest_path.exists());

    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    assert_eq!(manifest["page_count"], 1);
    assert!(
        manifest["build_hash"]
            .as_str()
            .unwrap()
            .starts_with("blake3:")
    );
}

// ── CAS Push ─────────────────────────────────────────────────────────

#[test]
fn cas_push_fails_without_socket() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let public_dir = root.join("public");
    std::fs::create_dir_all(&public_dir).unwrap();
    std::fs::write(public_dir.join("index.html"), "<html>test</html>").unwrap();
    std::fs::copy(
        sporeprint_root().join("config.toml"),
        root.join("config.toml"),
    )
    .unwrap();

    let output = run_raw(&[
        "--root",
        &root.to_string_lossy(),
        "cas-push",
        "--generate",
        "--socket",
        "/tmp/nonexistent-nestgate-integration-test.sock",
    ]);

    assert!(
        !output.status.success(),
        "cas-push should fail when socket doesn't exist"
    );
    let stderr = stderr_of(&output);
    assert!(
        stderr.contains("failed to connect"),
        "expected connection error, got: {stderr}"
    );
}

#[test]
fn cas_push_requires_manifest_or_generate() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let public_dir = root.join("public");
    std::fs::create_dir_all(&public_dir).unwrap();
    std::fs::write(public_dir.join("page.html"), "<html/>").unwrap();
    std::fs::copy(
        sporeprint_root().join("config.toml"),
        root.join("config.toml"),
    )
    .unwrap();

    let output = run_raw(&["--root", &root.to_string_lossy(), "cas-push"]);
    assert!(!output.status.success());
    let stderr = stderr_of(&output);
    assert!(
        stderr.contains("CAS manifest not found")
            || stderr.contains("NestGate socket not found")
            || stderr.contains("nestgate socket not found"),
        "expected manifest or socket error, got: {stderr}"
    );
}

// ── Discovery ────────────────────────────────────────────────────────

#[test]
fn discover_shows_self_capabilities() {
    let output = run_raw(&["discover"]);
    assert_success(&output, "discover");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("SELF: sporePrint"));
    assert!(stdout.contains("cas-push"));
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("PEERS:"));
}

#[test]
fn discover_does_not_require_config() {
    let output = run_raw(&["--root", "/nonexistent/path", "discover"]);
    assert!(
        output.status.success(),
        "discover should work without config.toml"
    );
}

// ── Depot Verify ─────────────────────────────────────────────────────

#[test]
fn depot_verify_passes_correct_binaries() {
    let dir = tempfile::tempdir().unwrap();
    let depot = dir.path().join("depot");
    std::fs::create_dir(&depot).unwrap();

    let content = b"test binary content for depot verify";
    let hash = blake3::hash(content).to_hex().to_string();
    std::fs::write(depot.join("test_primal"), content).unwrap();

    let checksums_path = dir.path().join("checksums.toml");
    let checksums_content = format!(
        "[test-arch]\ntest_primal = {{ blake3 = \"{hash}\", size = {} }}\n",
        content.len()
    );
    std::fs::write(&checksums_path, checksums_content).unwrap();

    let output = run_raw(&[
        "depot-verify",
        "--checksums",
        checksums_path.to_str().unwrap(),
        "--depot",
        depot.to_str().unwrap(),
        "--arch",
        "test-arch",
    ]);

    assert_success(&output, "depot-verify");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("DEPOT INTEGRITY VERIFIED"));
    assert!(stdout.contains("1/1 verified"));
}

#[test]
fn depot_verify_fails_on_hash_mismatch() {
    let dir = tempfile::tempdir().unwrap();
    let depot = dir.path().join("depot");
    std::fs::create_dir(&depot).unwrap();
    std::fs::write(depot.join("bad_primal"), b"wrong content").unwrap();

    let checksums_path = dir.path().join("checksums.toml");
    let checksums_content = format!(
        "[test-arch]\nbad_primal = {{ blake3 = \"{}\", size = {} }}\n",
        "0".repeat(64),
        b"wrong content".len()
    );
    std::fs::write(&checksums_path, checksums_content).unwrap();

    let output = run_raw(&[
        "depot-verify",
        "--checksums",
        checksums_path.to_str().unwrap(),
        "--depot",
        depot.to_str().unwrap(),
        "--arch",
        "test-arch",
    ]);

    assert!(!output.status.success());
    let stdout = stdout_of(&output);
    assert!(stdout.contains("BLAKE3 mismatch"));
    assert!(stdout.contains("INTEGRITY FAILURE"));
}

#[test]
fn depot_verify_partial_mode_passes_with_missing() {
    let dir = tempfile::tempdir().unwrap();
    let depot = dir.path().join("depot");
    std::fs::create_dir(&depot).unwrap();

    let content = b"partial depot test";
    let hash = blake3::hash(content).to_hex().to_string();
    std::fs::write(depot.join("present"), content).unwrap();

    let checksums_path = dir.path().join("checksums.toml");
    let checksums_content = format!(
        "[test-arch]\npresent = {{ blake3 = \"{hash}\", size = {} }}\nmissing = {{ blake3 = \"abc\", size = 99 }}\n",
        content.len()
    );
    std::fs::write(&checksums_path, checksums_content).unwrap();

    let output = run_raw(&[
        "depot-verify",
        "--checksums",
        checksums_path.to_str().unwrap(),
        "--depot",
        depot.to_str().unwrap(),
        "--arch",
        "test-arch",
        "--partial",
    ]);

    assert_success(&output, "depot-verify --partial");
    let stdout = stdout_of(&output);
    assert!(stdout.contains("DEPOT INTEGRITY VERIFIED"));
    assert!(stdout.contains("1 missing"));
}

#[test]
fn depot_verify_does_not_require_config() {
    let output = run_raw(&[
        "--root",
        "/nonexistent/path",
        "depot-verify",
        "--checksums",
        "/nonexistent/checksums.toml",
        "--depot",
        "/nonexistent/depot",
        "--arch",
        "x86_64",
    ]);

    assert!(
        !output.status.success(),
        "should fail on missing checksums file (not config.toml)"
    );
    let stderr = stderr_of(&output);
    assert!(
        stderr.contains("checksums.toml") || stderr.contains("No such file"),
        "expected checksums file error, got: {stderr}"
    );
}

// ── NUCLEUS ──────────────────────────────────────────────────────────

#[test]
fn nucleus_ribocipher_flag_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let profile_path = dir.path().join("ribo-test.toml");
    std::fs::write(
        &profile_path,
        r#"[profile]
name = "ribo-test"

[primals]
fake_primal = { required = false, role = "test" }

[health]
min_healthy = 0
critical = []
"#,
    )
    .unwrap();

    let output = run_raw(&[
        "nucleus",
        "--profile",
        profile_path.to_str().unwrap(),
        "--probe",
        "--ribocipher",
    ]);

    assert!(
        stdout_of(&output).contains("NUCLEUS"),
        "should produce NUCLEUS output, got: {}",
        stdout_of(&output)
    );
}
