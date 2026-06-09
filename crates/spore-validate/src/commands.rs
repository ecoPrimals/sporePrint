// SPDX-License-Identifier: AGPL-3.0-or-later

//! Subcommand handler implementations for the `spore-validate` CLI.
//!
//! Each public function corresponds to one CLI subcommand. Shared utilities
//! (like `resolve_public_dir`) live here as private helpers.

use crate::{
    cas, cas_push, certify, content, discovery, error::Diagnostic, error::Error, fetch, graph,
    links, model, notebook, paths, provenance, refresh, registry, report, totals,
};
use std::path::{Path, PathBuf};

/// Resolve the transport endpoint for `NestGate` communication.
///
/// Priority order (transport injection pattern):
/// 1. CLI `--socket` flag (explicit override)
/// 2. `TRANSPORT_ENDPOINT` env var (launcher/Songbird injection — canonical JSON)
/// 3. Socket discovery (legacy `NESTGATE_SOCKET` / XDG / fallback probing)
fn resolve_transport_endpoint(
    socket_override: Option<&str>,
) -> Result<cas_push::TransportEndpoint, Error> {
    if let Some(s) = socket_override {
        return Ok(cas_push::TransportEndpoint::Uds { path: s.into() });
    }

    if let Ok(json) = std::env::var("TRANSPORT_ENDPOINT") {
        return serde_json::from_str(&json).map_err(|e| {
            Error::Config(format!("TRANSPORT_ENDPOINT parse error: {e}"))
        });
    }

    let socket_path = cas_push::discover_socket()?;
    Ok(cas_push::TransportEndpoint::Uds { path: socket_path })
}

/// Resolve a possibly-relative public directory to an absolute path.
fn resolve_public_dir(root: &Path, public_dir: &Path) -> Result<PathBuf, Error> {
    let dir = if public_dir.is_absolute() {
        public_dir.to_path_buf()
    } else {
        root.join(public_dir)
    };
    if !dir.is_dir() {
        return Err(Error::Config(format!(
            "build output directory not found: {}. Run `zola build` first.",
            dir.display()
        )));
    }
    Ok(dir)
}

pub fn validate(
    root: &Path,
    config: &model::Config,
    check: bool,
    strict: bool,
    verbose: bool,
) -> Result<(), Error> {
    println!("spore-validate: checking sporePrint entity registry...");

    if verbose {
        print!("{}", report::format_registry(&config.extra.entity_registry));
        print!("{}", report::format_totals(&config.extra.totals));
    }

    let mut diagnostics = Vec::new();

    registry::validate(&config.extra.entity_registry, &mut diagnostics);
    graph::validate_edges(&config.extra.entity_registry, &mut diagnostics);
    totals::validate(
        &config.extra.entity_registry,
        &config.extra.totals,
        &mut diagnostics,
    );

    let content_dir = root.join(paths::CONTENT_DIR);
    if content_dir.is_dir() {
        content::validate_taxonomies(
            root,
            &content_dir,
            &config.extra.entity_registry,
            &mut diagnostics,
        );
        content::lint_internal_links(root, &content_dir, &mut diagnostics);

        if check {
            content::check_integrity(
                root,
                &content_dir,
                &config.extra.entity_registry,
                &mut diagnostics,
            );
            let link_warnings = links::validate_internal_links(&content_dir);
            diagnostics.extend(link_warnings);
        }
    }

    if strict {
        for diag in &mut diagnostics {
            diag.promote_to_error();
        }
    }

    let warnings: Vec<&Diagnostic> = diagnostics.iter().filter(|d| !d.is_error()).collect();
    let errors: Vec<&Diagnostic> = diagnostics.iter().filter(|d| d.is_error()).collect();

    for w in &warnings {
        println!("  WARN:  {}", w.message());
    }
    for e in &errors {
        println!("  ERROR: {}", e.message());
    }

    let summary = report::summarize(config);

    if errors.is_empty() {
        println!(
            "  OK: {} entities ({} primals, {} springs), {} warning(s), 0 errors",
            summary.entity_count,
            summary.primal_count,
            summary.spring_count,
            warnings.len()
        );
        Ok(())
    } else {
        println!(
            "\n  {} error(s), {} warning(s)",
            errors.len(),
            warnings.len()
        );
        Err(Error::ValidationFailed {
            error_count: errors.len(),
            warning_count: warnings.len(),
        })
    }
}

pub fn refresh(
    config_path: &Path,
    config: &model::Config,
    repos_root: &Path,
    write: bool,
    source: Option<&str>,
) -> Result<(), Error> {
    let repos_root = repos_root
        .canonicalize()
        .unwrap_or_else(|_| repos_root.to_path_buf());

    if let Some(name) = source {
        println!(
            "spore-validate refresh: scanning {name} in {}...",
            repos_root.display()
        );
    } else {
        println!(
            "spore-validate refresh: scanning repos in {}...",
            repos_root.display()
        );
    }

    let result = refresh::scan(&config.extra.entity_registry, &repos_root, source);

    for repo in &result.missing_repos {
        println!("  SKIP: {repo} -- repo not found");
    }

    if result.drifts.is_empty() {
        println!(
            "  OK: {} repos scanned, all metrics match registry",
            result.scanned
        );
        return Ok(());
    }

    println!();
    for d in &result.drifts {
        let pct = drift_pct(d.registered, d.actual);
        println!(
            "  DRIFT: [{}] {} -- registered: {}, actual: {} ({pct})",
            d.key, d.field, d.registered, d.actual
        );
    }
    println!(
        "\n  {} repos scanned, {} metric(s) drifted",
        result.scanned,
        result.drifts.len()
    );

    if write {
        refresh::write_updates(config_path, &result.drifts)?;
        println!(
            "  WRITE: config.toml updated with {} metric(s)",
            result.drifts.len()
        );
    }

    Ok(())
}

/// Precision loss from u64→f64 is intentional — only 1 decimal place needed.
#[allow(clippy::cast_precision_loss)]
fn drift_pct(registered: u64, actual: u64) -> String {
    if registered == 0 {
        return "new".to_string();
    }
    let diff = actual as f64 - registered as f64;
    let base = registered as f64;
    format!("{:+.1}%", diff / base * 100.0)
}

pub fn render_notebooks(root: &Path, dirs: &[PathBuf], springs: Option<&Path>) {
    println!("spore-validate: rendering notebooks to Zola markdown...");

    let (count, messages) = notebook::render_notebooks(root, dirs, springs);

    for msg in &messages {
        println!("  {msg}");
    }

    println!("\n  Rendered {count} notebook(s)");
}

pub fn check_links(root: &Path) -> Result<(), Error> {
    let content_root = paths::require_content_dir(root)?;

    println!("spore-validate: checking internal links...");
    let report = links::check_links(&content_root);

    if report.broken_links.is_empty() {
        println!(
            "  OK: {} files, {} internal links, 0 broken",
            report.files_scanned, report.links_found
        );
        Ok(())
    } else {
        for link in &report.broken_links {
            println!("  BROKEN: {link}");
        }
        println!(
            "\n  {} files, {} links, {} broken",
            report.files_scanned,
            report.links_found,
            report.broken_links.len()
        );
        Err(Error::ValidationFailed {
            error_count: report.broken_links.len(),
            warning_count: 0,
        })
    }
}

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
            println!("  WARN: no existing manifest at {}", manifest_path.display());
            println!("  Run with --write to create one");
            return Ok(());
        }
        let (new_pages, changed, removed) =
            provenance::diff_manifests(&manifest_path, &manifest);

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
        let (new_pages, changed, removed) =
            provenance::diff_manifests(&manifest_path, &manifest);
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

pub fn graph(root: &Path, config: &model::Config, emit: bool) -> Result<(), Error> {
    println!("spore-validate: building entity graph (renvois de choses)...");

    let mut diagnostics = Vec::new();
    graph::validate_edges(&config.extra.entity_registry, &mut diagnostics);

    for d in &diagnostics {
        println!("  ERROR: {}", d.message());
    }

    if !diagnostics.is_empty() {
        return Err(Error::ValidationFailed {
            error_count: diagnostics.len(),
            warning_count: 0,
        });
    }

    let entity_graph = graph::build_graph(&config.extra.entity_registry);

    println!(
        "  {} nodes, {} edges ({} declared + {} inverse)",
        entity_graph.stats.node_count,
        entity_graph.stats.edge_count,
        entity_graph.stats.declared_edges,
        entity_graph.stats.inverse_edges,
    );

    if emit {
        let output_path = root.join(paths::ENTITY_GRAPH_JSON);
        graph::emit_graph_json(&entity_graph, &output_path)
            .map_err(|e| Error::io(&output_path, e))?;
        println!("  EMIT: {}", output_path.display());
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

pub fn fetch_refresh(
    root: &Path,
    config_path: &Path,
    config: &model::Config,
    write: bool,
    source: Option<&str>,
) -> Result<(), Error> {
    println!("spore-validate: fetching upstream repos...");

    let result = fetch::fetch_and_refresh(root, source)?;
    for outcome in &result.outcomes {
        println!("{outcome}");
    }
    println!("---\nRepos staged in {}", result.clone_root.display());

    println!("\nspore-validate: scanning for metric drift...");
    refresh(config_path, config, &result.clone_root, write, source)
}

pub fn cas_push(
    root: &Path,
    public_dir: &Path,
    socket_override: Option<&str>,
    generate: bool,
) -> Result<(), Error> {
    let dir = resolve_public_dir(root, public_dir)?;

    let endpoint = resolve_transport_endpoint(socket_override)?;

    println!("spore-validate: CAS push to NestGate ({endpoint:?})");

    let manifest = if generate {
        println!("  generating manifest on-the-fly...");
        let m = cas::generate_manifest(&dir);
        println!("  {} files, {} pages", m.files.len(), m.page_count);
        cas_push::StoredManifest {
            build_id: m.build_id,
            build_hash: m.build_hash,
            page_count: m.page_count,
            total_bytes: m.total_bytes,
            files: m
                .files
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        cas_push::StoredEntry {
                            hash: v.hash,
                            size: v.size,
                            content_type: v.content_type,
                        },
                    )
                })
                .collect(),
        }
    } else {
        let manifest_path = root.join(paths::CAS_MANIFEST);
        if !manifest_path.exists() {
            return Err(Error::Config(format!(
                "CAS manifest not found at {}. Run `cas-manifest --emit` first, or use --generate.",
                manifest_path.display()
            )));
        }
        println!("  reading manifest: {}", manifest_path.display());
        cas_push::read_manifest(&manifest_path)?
    };

    println!(
        "  build: {} ({} files, {} pages, {} bytes)",
        &manifest.build_hash[..20.min(manifest.build_hash.len())],
        manifest.files.len(),
        manifest.page_count,
        manifest.total_bytes
    );

    let result = cas_push::push_manifest(&manifest, &dir, &endpoint)?;

    println!("  ---");
    println!("  stored:       {} files", result.stored);
    println!("  deduplicated: {} files (already in CAS)", result.deduplicated);
    if result.errors > 0 {
        println!("  errors:       {} files", result.errors);
    }
    #[allow(clippy::cast_precision_loss)]
    let kb = result.total_bytes_transferred as f64 / 1024.0;
    println!("  transferred:  {kb:.1} KB");
    println!("  elapsed:      {} ms", result.elapsed_ms);

    if result.errors > 0 {
        #[allow(clippy::cast_possible_truncation)]
        let count = result.errors as usize;
        Err(Error::ValidationFailed {
            error_count: count,
            warning_count: 0,
        })
    } else {
        Ok(())
    }
}

pub fn cas_manifest(root: &Path, public_dir: &Path, emit: bool) -> Result<(), Error> {
    let dir = resolve_public_dir(root, public_dir)?;

    println!(
        "spore-validate: generating CAS manifest for {}",
        dir.display()
    );

    let manifest = cas::generate_manifest(&dir);

    println!("  files: {}", manifest.files.len());
    println!("  HTML pages: {}", manifest.page_count);
    #[allow(clippy::cast_precision_loss)]
    let size_kb = manifest.total_bytes as f64 / 1024.0;
    println!("  total size: {} bytes ({size_kb:.1} KB)", manifest.total_bytes);
    println!("  build hash: {}", manifest.build_hash);

    if emit {
        let output_path = root.join(paths::CAS_MANIFEST);
        cas::emit_manifest(&manifest, &output_path)?;
        println!("  EMIT: {}", output_path.display());
    }

    Ok(())
}

#[allow(clippy::unnecessary_wraps)] // uniform handler signature for main.rs dispatch
pub fn discover() -> Result<(), Error> {
    println!("spore-validate: capability discovery");
    println!();

    let self_caps = &discovery::SELF;
    println!("  SELF: {} v{}", self_caps.primal_id, self_caps.version);
    println!("  capabilities:");
    for cap in self_caps.capabilities {
        println!("    [{:>9}] {} — {}", cap.category, cap.name, cap.description);
    }

    println!();
    println!("  TRANSPORT:");
    if let Ok(json) = std::env::var("TRANSPORT_ENDPOINT") {
        match serde_json::from_str::<cas_push::TransportEndpoint>(&json) {
            Ok(ep) => println!("    injected: {ep:?}"),
            Err(e) => println!("    TRANSPORT_ENDPOINT parse error: {e}"),
        }
    } else {
        println!("    (no TRANSPORT_ENDPOINT — will use socket discovery)");
    }

    println!();
    println!("  PEERS:");

    let peers = discovery::discover_peers();
    if peers.is_empty() {
        println!("    (none discovered — set NESTGATE_SOCKET or PETALTONGUE_SOCKET)");
    } else {
        for peer in &peers {
            println!("    {} ({})", peer.primal_id, peer.socket_path.as_deref().unwrap_or("?"));
            for cap in &peer.capabilities {
                println!("      - {cap}");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_transport_cli_override_produces_uds() {
        let ep = resolve_transport_endpoint(Some("/custom/path.sock")).unwrap();
        match ep {
            cas_push::TransportEndpoint::Uds { path } => {
                assert_eq!(path, "/custom/path.sock");
            }
            _ => panic!("expected UDS endpoint from CLI override"),
        }
    }

    #[test]
    fn resolve_public_dir_absolute_path_passthrough() {
        let dir = std::env::temp_dir();
        let result = resolve_public_dir(Path::new("/irrelevant"), &dir);
        assert_eq!(result.unwrap(), dir);
    }

    #[test]
    fn resolve_public_dir_relative_joins_root() {
        let tmp = tempfile::tempdir().unwrap();
        let sub = tmp.path().join("public");
        std::fs::create_dir(&sub).unwrap();
        let result = resolve_public_dir(tmp.path(), Path::new("public"));
        assert_eq!(result.unwrap(), sub);
    }

    #[test]
    fn resolve_public_dir_missing_dir_returns_error() {
        let tmp = tempfile::tempdir().unwrap();
        let result = resolve_public_dir(tmp.path(), Path::new("nonexistent"));
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("not found"), "{msg}");
    }

    #[test]
    fn resolve_transport_cli_takes_priority() {
        let ep = resolve_transport_endpoint(Some("/override.sock")).unwrap();
        match ep {
            cas_push::TransportEndpoint::Uds { path } => assert_eq!(path, "/override.sock"),
            _ => panic!("CLI override should always produce UDS"),
        }
    }

    #[test]
    fn transport_endpoint_tcp_deserializes() {
        let json = r#"{"transport":"tcp","host":"10.0.0.1","port":8080}"#;
        let ep: cas_push::TransportEndpoint = serde_json::from_str(json).unwrap();
        match ep {
            cas_push::TransportEndpoint::Tcp { host, port } => {
                assert_eq!(host, "10.0.0.1");
                assert_eq!(port, 8080);
            }
            _ => panic!("expected TCP"),
        }
    }

    #[test]
    fn transport_endpoint_mesh_relay_deserializes() {
        let json = r#"{"transport":"mesh_relay","peer_id":"eastGate","capability":"content"}"#;
        let ep: cas_push::TransportEndpoint = serde_json::from_str(json).unwrap();
        match ep {
            cas_push::TransportEndpoint::MeshRelay { peer_id, capability } => {
                assert_eq!(peer_id, "eastGate");
                assert_eq!(capability, "content");
            }
            _ => panic!("expected MeshRelay"),
        }
    }

    #[test]
    fn transport_endpoint_invalid_json_errors() {
        let result: Result<cas_push::TransportEndpoint, _> =
            serde_json::from_str("not valid json");
        assert!(result.is_err());
    }
}
