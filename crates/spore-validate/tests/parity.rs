// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parity validation: petalTongue content-direct vs Zola output.
//!
//! These tests verify that the petalTongue content-direct backend produces
//! structurally equivalent output to Zola for sporePrint content.
//!
//! Run with: `cargo test --test parity`
//! Requires: petalTongue server running on `$PETALTONGUE_PORT` (default 8080)
//!
//! Skip with: `cargo test --test parity -- --ignored` (or set `SKIP_PARITY=1`)

use std::path::{Path, PathBuf};

fn sporeprint_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn pt_base() -> String {
    let port = std::env::var("PETALTONGUE_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("http://localhost:{port}")
}

fn server_available() -> bool {
    std::net::TcpStream::connect(format!(
        "127.0.0.1:{}",
        std::env::var("PETALTONGUE_PORT").unwrap_or_else(|_| "8080".to_string())
    ))
    .is_ok()
}

#[test]
#[ignore = "requires petalTongue server running"]
fn parity_content_pages_serve() {
    if !server_available() {
        eprintln!("SKIP: petalTongue not running");
        return;
    }

    let pages = [
        "/architecture/PRIMAL_CATALOG",
        "/architecture/ECOSYSTEM_ARCHITECTURE",
        "/architecture/KDERM_DIDERM_ARCHITECTURE",
        "/architecture/NUCLEUS_ARCHITECTURE",
        "/primals/beardog",
    ];

    let base = pt_base();
    let client = ureq::agent();

    for page in &pages {
        let url = format!("{base}{page}");
        let resp = client.get(&url).call();
        assert!(
            resp.is_ok(),
            "Page {page} should be served (got error: {:?})",
            resp.err()
        );
        let status = resp.unwrap().status();
        assert_eq!(status, 200, "Page {page} returned {status}");
    }
}

#[test]
#[ignore = "requires petalTongue server running"]
fn parity_entity_shortcodes_resolve() {
    if !server_available() {
        return;
    }

    let base = pt_base();
    let client = ureq::agent();

    let url = format!("{base}/architecture/NUCLEUS_ARCHITECTURE");
    let body = client.get(&url).call().unwrap().into_string().unwrap();

    assert!(
        !body.contains("⚠️"),
        "NUCLEUS page has unresolved entity shortcodes"
    );
    assert!(
        body.contains("beardog") || body.contains("BearDog"),
        "NUCLEUS page should reference BearDog"
    );
}

#[test]
#[ignore = "requires petalTongue server running"]
fn parity_description_modality() {
    if !server_available() {
        return;
    }

    let base = pt_base();
    let client = ureq::agent();

    let url = format!("{base}/architecture/PRIMAL_CATALOG?modality=description");
    let body = client.get(&url).call().unwrap().into_string().unwrap();

    assert!(
        body.starts_with("Document:"),
        "Description modality should start with 'Document:'"
    );
}

#[test]
#[ignore = "requires petalTongue server running"]
fn parity_viz_endpoints() {
    if !server_available() {
        return;
    }

    let base = pt_base();
    let client = ureq::agent();

    let vizs = ["entity-graph", "kderm-topology", "nucleus-composition"];

    for viz in &vizs {
        let svg_url = format!("{base}/viz/{viz}");
        let resp = client.get(&svg_url).call().unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.into_string().unwrap();
        assert!(body.contains("<svg"), "Viz {viz} should return SVG");

        let json_url = format!("{base}/viz/{viz}?format=scene-json");
        let resp = client.get(&json_url).call().unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.into_string().unwrap();
        assert!(
            body.contains("\"nodes\""),
            "Viz {viz} scene-json should have nodes"
        );
    }
}

#[test]
#[ignore = "requires petalTongue server running"]
fn parity_static_assets() {
    if !server_available() {
        return;
    }

    let base = pt_base();
    let client = ureq::agent();

    let assets = [
        "/js/viz-hydrate.js",
        "/wasm/petal_tongue_wasm.js",
        "/graph/entity-graph.json",
    ];

    for asset in &assets {
        let url = format!("{base}{asset}");
        let resp = client.get(&url).call();
        assert!(resp.is_ok(), "Asset {asset} should be served");
        assert_eq!(resp.unwrap().status(), 200, "Asset {asset} should be 200");
    }
}

#[test]
#[ignore = "requires petalTongue server and Zola build"]
fn parity_structural_comparison() {
    if !server_available() {
        return;
    }

    let zola_dir = sporeprint_root().join("public");
    if !zola_dir.is_dir() {
        eprintln!("SKIP: No Zola build at {}", zola_dir.display());
        return;
    }

    let base = pt_base();
    let client = ureq::agent();

    // Compare H2 heading counts between petalTongue and Zola for a known page
    let pt_url = format!("{base}/architecture/PRIMAL_CATALOG");
    let pt_body = client.get(&pt_url).call().unwrap().into_string().unwrap();
    let pt_h2 = pt_body.matches("<h2").count();

    let zola_file = zola_dir.join("architecture/primal-catalog/index.html");
    if zola_file.is_file() {
        let zola_body = std::fs::read_to_string(&zola_file).unwrap();
        let zola_h2 = zola_body.matches("<h2").count();

        if pt_h2 > 0 && zola_h2 > 0 {
            assert_eq!(
                pt_h2, zola_h2,
                "H2 count mismatch: petalTongue={pt_h2}, Zola={zola_h2}"
            );
        }
    }
}
