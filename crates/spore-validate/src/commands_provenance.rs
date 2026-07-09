// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{certify, error::Error, graph, model, paths, provenance, registry};
use std::path::Path;

pub fn provenance(root: &Path, verify: bool, diff: bool, write: bool) -> Result<(), Error> {
    let content_dir = paths::require_content_dir(root)?;

    let manifest_path = provenance::manifest_path(root);

    println!("spore-validate: computing BLAKE3 content hashes...");
    let manifest = provenance::generate_manifest(&content_dir);
    println!(
        "  {} pages hashed, root: {}",
        manifest.page_count,
        &manifest.root_hash[..16]
    );

    if verify {
        if !manifest_path.exists() {
            println!(
                "  WARN: no existing manifest at {}",
                manifest_path.display()
            );
            println!("  Run with --write to create one");
            return Ok(());
        }
        let (new_pages, changed, removed) = provenance::diff_manifests(&manifest_path, &manifest);

        if new_pages.is_empty() && changed.is_empty() && removed.is_empty() {
            println!("  OK: all {} pages match manifest", manifest.page_count);
        } else {
            if !new_pages.is_empty() {
                println!("  NEW:     {} page(s)", new_pages.len());
            }
            if !changed.is_empty() {
                println!("  CHANGED: {} page(s)", changed.len());
            }
            if !removed.is_empty() {
                println!("  REMOVED: {} page(s)", removed.len());
            }
            return Err(Error::ValidationFailed {
                error_count: changed.len() + removed.len(),
                warning_count: new_pages.len(),
            });
        }
    }

    if diff {
        let (new_pages, changed, removed) = provenance::diff_manifests(&manifest_path, &manifest);
        for p in &new_pages {
            println!("  + {p}");
        }
        for p in &changed {
            println!("  ~ {p}");
        }
        for p in &removed {
            println!("  - {p}");
        }
        if new_pages.is_empty() && changed.is_empty() && removed.is_empty() {
            println!("  (no changes)");
        }
    }

    if write {
        provenance::write_manifest(&manifest, &manifest_path)
            .map_err(|e| Error::Config(format!("failed to write manifest: {e}")))?;
        println!(
            "  WRITE: content-manifest.toml ({} pages, root {})",
            manifest.page_count,
            &manifest.root_hash[..16]
        );
    }

    Ok(())
}

pub fn certify(root: &Path, config: &model::Config, emit: bool) -> Result<(), Error> {
    println!("spore-validate: certification (guideStone mode)...");

    let mut diagnostics = Vec::new();
    registry::validate(&config.extra.entity_registry, &mut diagnostics);
    graph::validate_edges(&config.extra.entity_registry, &mut diagnostics);
    let validation_errors = diagnostics.iter().filter(|d| d.is_error()).count();

    let manifest = certify::build_manifest(config, root, validation_errors);

    println!("  entities: {}", manifest.entity_count);
    println!("  edges: {}", manifest.edge_count);
    println!("  content pages: {}", manifest.content_pages);
    println!("  graph merkle: {}", manifest.graph_merkle);

    let manifest_path = root.join(paths::CERTIFICATION_MANIFEST);

    if emit {
        certify::emit_manifest(&manifest, &manifest_path)
            .map_err(|e| Error::io(&manifest_path, e))?;
        println!("  EMIT: {}", manifest_path.display());
    } else if manifest_path.exists() {
        match certify::validate_manifest(&manifest_path, &manifest) {
            Ok(drifts) if drifts.is_empty() => {
                println!("  VALID: manifest matches current state");
            }
            Ok(drifts) => {
                println!("  DRIFT detected ({} fields):", drifts.len());
                for d in &drifts {
                    println!("    {d}");
                }
            }
            Err(e) => {
                println!("  WARN: could not read existing manifest: {e}");
            }
        }
    } else {
        println!("  INFO: no existing manifest; use --emit to create one");
    }

    Ok(())
}
