// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    cli::{Cli, Command},
    commands, commands_depot, discovery,
    error::Error,
    model, paths, petaltongue, tower,
};
use std::path::{Path, PathBuf};

/// Dispatch commands that don't require `config.toml`.
/// Returns `Some(result)` if handled, `None` if the caller should continue.
pub fn dispatch_standalone(cli: &Cli) -> Option<Result<(), Error>> {
    match &cli.command {
        Some(Command::Discover) => Some(commands::discover()),
        Some(Command::Nucleus {
            profile,
            probe,
            ribocipher,
        }) => Some(commands::nucleus(profile, *probe, *ribocipher)),
        Some(Command::DepotVerify {
            checksums,
            depot,
            arch,
            partial,
            list_arches,
        }) => Some(dispatch_depot(
            checksums,
            depot.as_ref(),
            arch.as_ref(),
            *partial,
            *list_arches,
        )),
        Some(Command::PtRender {
            path,
            modality,
            socket,
        }) => Some(run_pt_render(path, modality.as_deref(), socket.as_deref())),
        Some(Command::PtStatus) => Some(run_pt_status()),
        Some(Command::TowerStatus) => Some(run_tower_status()),
        Some(Command::PtViz {
            name,
            format,
            socket,
        }) => Some(run_pt_viz(name, format, socket.as_deref())),
        Some(Command::BuildViz { socket }) => Some(run_build_viz(socket.as_deref())),
        _ => None,
    }
}

fn dispatch_depot(
    checksums: &Path,
    depot: Option<&PathBuf>,
    arch: Option<&String>,
    partial: bool,
    list_arches: bool,
) -> Result<(), Error> {
    if list_arches {
        return commands_depot::list_arches(checksums);
    }
    let depot = depot
        .ok_or_else(|| Error::Config("--depot is required when not using --list-arches".into()))?;
    let arch = arch
        .ok_or_else(|| Error::Config("--arch is required when not using --list-arches".into()))?;
    commands_depot::verify(checksums, depot, arch, partial)
}

/// Run petalTongue graph render via IPC.
fn run_pt_render(
    path: &str,
    modality: Option<&str>,
    socket_override: Option<&str>,
) -> Result<(), Error> {
    let endpoint =
        discovery::resolve_primal_endpoint("petaltongue", "PETALTONGUE_SOCKET", socket_override)?;

    let mut client = petaltongue::PetalTongueClient::connect(&endpoint)?;

    let graph = load_entity_graph_for_render(&client_root())?;
    let session_id = format!("sporePrint-render-{path}");
    let result = client.render_graph(&session_id, &graph, modality)?;

    println!("sporePrint: petalTongue render.graph");
    println!("  Session: {}", result.content_path);
    println!("  Format: {}", result.format);
    println!("  Latency: {}ms", result.latency_ms);
    println!("  Data length: {} bytes", result.data.len());
    if let Some(meta) = &result.metadata {
        println!("  Metadata: {meta}");
    }
    println!();

    if result.data.len() <= 4000 {
        println!("{}", result.data);
    } else {
        println!("{}...", &result.data[..4000]);
        println!("  (truncated, {} total bytes)", result.data.len());
    }

    Ok(())
}

/// Load the entity graph JSON for rendering.
fn load_entity_graph_for_render(root: &Path) -> Result<serde_json::Value, Error> {
    let graph_path = root.join(paths::ENTITY_GRAPH_JSON);
    if graph_path.is_file() {
        let content =
            std::fs::read_to_string(&graph_path).map_err(|e| Error::io(&graph_path, e))?;
        serde_json::from_str(&content)
            .map_err(|e| Error::Config(format!("parse entity-graph.json: {e}")))
    } else {
        Ok(serde_json::json!({"nodes": [], "edges": []}))
    }
}

/// Resolve the sporePrint root from cwd.
fn client_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Run petalTongue status check (socket discovery + method probing).
fn run_pt_status() -> Result<(), Error> {
    println!("sporePrint: petalTongue backend status");
    println!();

    let status = petaltongue::status()?;
    println!("  {status}");
    println!();

    if status.health.is_some() && status.render_graph && status.viz_export {
        println!("  RESULT: ✅ petalTongue backend OPERATIONAL");
    } else if status.health.is_some() {
        println!("  RESULT: ⚠️  petalTongue backend PARTIAL");
    } else {
        println!("  RESULT: ❌ petalTongue backend UNREACHABLE");
    }

    Ok(())
}

/// Run Tower P1 readiness probe.
#[allow(clippy::unnecessary_wraps)]
fn run_tower_status() -> Result<(), Error> {
    let status = tower::probe_tower_status(None);
    tower::print_tower_status(&status);
    Ok(())
}

/// Request a visualization from petalTongue via IPC.
fn run_pt_viz(name: &str, format: &str, socket_override: Option<&str>) -> Result<(), Error> {
    let viz_format = match format {
        "svg" => petaltongue::VizFormat::Svg,
        "scene-json" => petaltongue::VizFormat::SceneJson,
        other => {
            return Err(Error::Config(format!(
                "unknown viz format '{other}' — use 'svg' or 'scene-json'"
            )));
        }
    };

    let endpoint =
        discovery::resolve_primal_endpoint("petaltongue", "PETALTONGUE_SOCKET", socket_override)?;

    let mut client = petaltongue::PetalTongueClient::connect(&endpoint)?;
    let result = client.viz(name, viz_format)?;

    println!("sporePrint: petalTongue viz");
    println!("  Name: {name}");
    println!("  Format: {format}");
    println!("  Latency: {}ms", result.latency_ms);
    println!("  Body length: {} bytes", result.body.len());
    println!();
    println!("{}", result.body);

    Ok(())
}

pub fn run_build_viz(socket_override: Option<&str>) -> Result<(), Error> {
    let root = client_root();
    let content_dir = paths::require_content_dir(&root)?;

    let viz_names = commands::scan_viz_embeds(&content_dir);

    if viz_names.is_empty() {
        println!("spore-validate: no viz_embed shortcodes found in content");
        return Ok(());
    }

    println!(
        "spore-validate: building {} visualization(s) via petalTongue",
        viz_names.len()
    );

    let endpoint =
        discovery::resolve_primal_endpoint("petaltongue", "PETALTONGUE_SOCKET", socket_override)?;

    let mut client = petaltongue::PetalTongueClient::connect(&endpoint)?;

    let output_dir = root.join(paths::VIZ_OUTPUT_DIR);
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| Error::Config(format!("create {}/: {e}", paths::VIZ_OUTPUT_DIR)))?;

    let mut success = 0u32;
    let mut skipped = 0u32;

    for name in &viz_names {
        match client.viz(name, petaltongue::VizFormat::Svg) {
            Ok(result) => {
                let out_path = output_dir.join(format!("{name}.svg"));
                std::fs::write(&out_path, &result.body).map_err(|e| Error::io(&out_path, e))?;
                println!(
                    "  ✅ {name}.svg ({} bytes, {}ms)",
                    result.body.len(),
                    result.latency_ms
                );
                success += 1;
            }
            Err(e) => {
                let existing = output_dir.join(format!("{name}.svg"));
                if existing.exists() {
                    println!("  ⚠️  {name}: petalTongue error ({e}), keeping existing SVG");
                } else {
                    println!("  ❌ {name}: {e} (no existing SVG to fall back on)");
                }
                skipped += 1;
            }
        }
    }

    println!("\n  Generated: {success}, Skipped: {skipped}");
    Ok(())
}

/// Dispatch commands that require a parsed `config.toml`.
pub fn dispatch_with_config(cli: &Cli, root: &Path) -> Result<(), Error> {
    let config_path = root.join(paths::CONFIG_FILE);
    let config = model::parse_config(&config_path)?;

    match &cli.command {
        None | Some(Command::Validate { .. }) => {
            let (check, strict, verbose) = match &cli.command {
                Some(Command::Validate {
                    check,
                    strict,
                    verbose,
                }) => (*check, *strict, *verbose),
                _ => (false, false, false),
            };
            commands::validate(root, &config, check, strict, verbose)
        }
        Some(Command::Refresh {
            repos_root,
            write,
            source,
        }) => commands::refresh(&config_path, &config, repos_root, *write, source.as_deref()),
        Some(Command::RenderNotebooks {
            dirs,
            springs,
            discover,
        }) => {
            let effective_springs = if *discover {
                discover_springs_root(root).or_else(|| springs.clone())
            } else {
                springs.clone()
            };
            commands::render_notebooks(root, dirs, effective_springs.as_deref());
            Ok(())
        }
        Some(Command::FetchRefresh { write, source }) => {
            commands::fetch_refresh(root, &config_path, &config, *write, source.as_deref())
        }
        Some(Command::CheckLinks) => commands::check_links(root),
        Some(Command::Provenance {
            verify,
            diff,
            write,
        }) => commands::provenance(root, *verify, *diff, *write),
        Some(Command::Graph { emit }) => commands::graph(root, &config, *emit),
        Some(Command::Certify { emit }) => commands::certify(root, &config, *emit),
        Some(Command::Discover) => commands::discover(),
        Some(
            Command::Nucleus { .. }
            | Command::DepotVerify { .. }
            | Command::PtRender { .. }
            | Command::PtStatus
            | Command::TowerStatus
            | Command::PtViz { .. }
            | Command::BuildViz { .. },
        ) => unreachable!("handled by dispatch_standalone"),
        Some(Command::CasManifest { public_dir, emit }) => {
            commands::cas_manifest(root, public_dir, *emit)
        }
        Some(Command::CasPush {
            public_dir,
            socket,
            generate,
        }) => commands::cas_push(root, public_dir, socket.as_deref(), *generate),
    }
}

/// Walk up from `start` looking for a `.gate` file, then derive the springs root.
fn discover_springs_root(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        if dir.join(paths::GATE_MARKER).is_file() {
            let springs = dir.join(paths::SPRINGS_DIR);
            if springs.is_dir() {
                return Some(springs);
            }
            return None;
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn discover_springs_root_finds_gate_marker() {
        let dir = tempfile::tempdir().unwrap();
        let gate_root = dir.path();
        std::fs::write(gate_root.join(paths::GATE_MARKER), "").unwrap();
        std::fs::create_dir(gate_root.join(paths::SPRINGS_DIR)).unwrap();
        let nested = gate_root.join("infra/sporePrint");
        std::fs::create_dir_all(&nested).unwrap();

        let result = discover_springs_root(&nested);
        assert_eq!(result, Some(gate_root.join(paths::SPRINGS_DIR)));
    }

    #[test]
    fn discover_springs_root_none_without_springs_dir() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(paths::GATE_MARKER), "").unwrap();

        let result = discover_springs_root(dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn discover_springs_root_none_without_gate_marker() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(paths::SPRINGS_DIR)).unwrap();

        let result = discover_springs_root(dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn load_entity_graph_reads_json() {
        let dir = tempfile::tempdir().unwrap();
        let graph_dir = dir.path().join("static/graph");
        std::fs::create_dir_all(&graph_dir).unwrap();
        std::fs::write(
            graph_dir.join("entity-graph.json"),
            r#"{"nodes":[{"id":"beardog"}],"edges":[]}"#,
        )
        .unwrap();

        let value = load_entity_graph_for_render(dir.path()).unwrap();
        let nodes = value["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0]["id"].as_str().unwrap(), "beardog");
    }

    #[test]
    fn load_entity_graph_returns_empty_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let value = load_entity_graph_for_render(dir.path()).unwrap();
        assert_eq!(value["nodes"].as_array().unwrap().len(), 0);
        assert_eq!(value["edges"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn load_entity_graph_errors_on_invalid_json() {
        let dir = tempfile::tempdir().unwrap();
        let graph_dir = dir.path().join("static/graph");
        std::fs::create_dir_all(&graph_dir).unwrap();
        std::fs::write(graph_dir.join("entity-graph.json"), "not json").unwrap();

        let result = load_entity_graph_for_render(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_standalone_returns_none_for_validate() {
        let cli = Cli {
            root: PathBuf::from("."),
            command: Some(Command::Validate {
                check: false,
                strict: false,
                verbose: false,
            }),
        };
        assert!(dispatch_standalone(&cli).is_none());
    }

    #[test]
    fn dispatch_standalone_returns_none_for_no_command() {
        let cli = Cli {
            root: PathBuf::from("."),
            command: None,
        };
        assert!(dispatch_standalone(&cli).is_none());
    }
}
