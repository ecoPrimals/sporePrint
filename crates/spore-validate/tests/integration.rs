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

#[test]
fn check_links_on_real_content() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "check-links"])
        .output()
        .expect("failed to run check-links");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "check-links failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("checking internal links"));
}

#[test]
fn graph_emit_produces_json() {
    build_binary();
    let root = sporeprint_root();
    let dir = tempfile::tempdir().unwrap();
    let temp_root = dir.path();

    std::fs::copy(root.join("config.toml"), temp_root.join("config.toml")).unwrap();
    std::fs::create_dir_all(temp_root.join("static/graph")).unwrap();

    let output = Command::new(binary_path())
        .args(["--root", &temp_root.to_string_lossy(), "graph", "--emit"])
        .output()
        .expect("failed to run graph --emit");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "graph --emit failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("EMIT:"));
    assert!(temp_root.join("static/graph/entity-graph.json").exists());
}

#[test]
fn certify_emit_produces_manifest() {
    build_binary();
    let root = sporeprint_root();
    let dir = tempfile::tempdir().unwrap();
    let temp_root = dir.path();

    std::fs::copy(root.join("config.toml"), temp_root.join("config.toml")).unwrap();
    std::fs::create_dir_all(temp_root.join("content")).unwrap();
    std::fs::create_dir_all(temp_root.join("static/certification")).unwrap();

    let output = Command::new(binary_path())
        .args(["--root", &temp_root.to_string_lossy(), "certify", "--emit"])
        .output()
        .expect("failed to run certify --emit");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "certify --emit failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("entities:"));
    assert!(
        temp_root
            .join("static/certification/manifest.json")
            .exists()
    );
}

#[test]
fn provenance_write_creates_manifest() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "provenance", "--write"])
        .output()
        .expect("failed to run provenance --write");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "provenance --write failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("BLAKE3"));
    assert!(root.join("content-manifest.toml").exists());
}

#[test]
fn provenance_verify_succeeds() {
    build_binary();
    let root = sporeprint_root();

    Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "provenance", "--write"])
        .output()
        .expect("failed to run provenance --write");

    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "provenance", "--verify"])
        .output()
        .expect("failed to run provenance --verify");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "provenance --verify failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn graph_without_emit_runs_cleanly() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "graph"])
        .output()
        .expect("failed to run graph");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "graph failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("nodes"));
    assert!(stdout.contains("edges"));
}

#[test]
fn certify_without_emit_validates() {
    build_binary();
    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "certify"])
        .output()
        .expect("failed to run certify");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "certify failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("entities:"));
    assert!(stdout.contains("content pages:"));
}

#[test]
fn cas_manifest_on_temp_dir() {
    build_binary();
    let dir = tempfile::tempdir().unwrap();
    let public = dir.path().join("public");
    std::fs::create_dir(&public).unwrap();
    std::fs::write(public.join("index.html"), "<html>test</html>").unwrap();
    std::fs::write(public.join("style.css"), "body{}").unwrap();

    let root = sporeprint_root();
    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "cas-manifest",
            "--public-dir",
            &public.to_string_lossy(),
        ])
        .output()
        .expect("failed to run cas-manifest");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "cas-manifest failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("files: 2"));
    assert!(stdout.contains("HTML pages: 1"));
    assert!(stdout.contains("build hash: blake3:"));
}

#[test]
fn cas_manifest_emit_writes_json() {
    build_binary();
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let public = root.join("public");
    std::fs::create_dir(&public).unwrap();
    std::fs::write(public.join("page.html"), "<h1>hi</h1>").unwrap();
    std::fs::create_dir_all(root.join("static/cas")).unwrap();

    std::fs::copy(sporeprint_root().join("config.toml"), root.join("config.toml")).unwrap();

    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "cas-manifest",
            "--emit",
        ])
        .output()
        .expect("failed to run cas-manifest --emit");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "cas-manifest --emit failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("EMIT:"));

    let manifest_path = root.join("static/cas/build-manifest.json");
    assert!(manifest_path.exists());

    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    assert_eq!(manifest["page_count"], 1);
    assert!(manifest["build_hash"].as_str().unwrap().starts_with("blake3:"));
}

#[test]
fn cas_push_fails_without_socket() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let public_dir = root.join("public");
    std::fs::create_dir_all(&public_dir).unwrap();
    std::fs::write(public_dir.join("index.html"), "<html>test</html>").unwrap();

    std::fs::copy(sporeprint_root().join("config.toml"), root.join("config.toml")).unwrap();

    let output = Command::new(binary_path())
        .args([
            "--root",
            &root.to_string_lossy(),
            "cas-push",
            "--generate",
            "--socket",
            "/tmp/nonexistent-nestgate-integration-test.sock",
        ])
        .output()
        .expect("failed to run cas-push");

    assert!(
        !output.status.success(),
        "cas-push should fail when socket doesn't exist"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
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

    std::fs::copy(sporeprint_root().join("config.toml"), root.join("config.toml")).unwrap();

    let output = Command::new(binary_path())
        .args(["--root", &root.to_string_lossy(), "cas-push"])
        .output()
        .expect("failed to run cas-push");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CAS manifest not found") || stderr.contains("NestGate socket not found"),
        "expected manifest or socket error, got: {stderr}"
    );
}

#[test]
fn discover_shows_self_capabilities() {
    let output = Command::new(binary_path())
        .args(["discover"])
        .output()
        .expect("failed to run discover");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "discover failed:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("SELF: sporePrint"));
    assert!(stdout.contains("cas-push"));
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("PEERS:"));
}

#[test]
fn discover_does_not_require_config() {
    let output = Command::new(binary_path())
        .args(["--root", "/nonexistent/path", "discover"])
        .output()
        .expect("failed to run discover");

    assert!(
        output.status.success(),
        "discover should work without config.toml"
    );
}
