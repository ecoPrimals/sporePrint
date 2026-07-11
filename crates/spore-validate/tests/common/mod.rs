// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared test harness helpers for integration, parity, and `refresh_write` tests.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

static BUILD_ONCE: Once = Once::new();

/// Root of the sporePrint repository (two levels up from the crate).
pub fn sporeprint_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Path to the built `spore-validate` debug binary.
pub fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("spore-validate");
    path
}

/// Ensure the binary is built exactly once per test run.
pub fn ensure_built() {
    BUILD_ONCE.call_once(|| {
        let status = Command::new("cargo")
            .args(["build", "--quiet"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("failed to build spore-validate");
        assert!(status.success(), "cargo build failed");
    });
}
