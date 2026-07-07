// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = "sporePrint validation CLI — entity registry, content integrity, and metric sync."]

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod cas;
mod cas_push;
mod certify;
mod commands;
mod commands_depot;
mod content;
mod depot;
mod discovery;
mod error;
mod fetch;
mod graph;
mod http;
mod ipc;
mod links;
mod model;
mod notebook;
mod nucleus;
mod nucleus_display;
mod paths;
mod petaltongue;
mod provenance;
mod refresh;
mod registry;
mod report;
mod time;
mod totals;
mod tower;

use error::Error;

#[derive(Parser)]
#[command(
    name = "spore-validate",
    about = "Typed validation tooling for the sporePrint entity registry and content",
    version
)]
struct Cli {
    /// Path to sporePrint root
    #[arg(short, long, default_value = ".", global = true)]
    root: PathBuf,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Validate entity registry, totals, and content taxonomies (default)
    Validate {
        /// Also scan prose for entity shortcode integrity
        #[arg(long)]
        check: bool,

        /// Treat warnings as errors
        #[arg(long)]
        strict: bool,

        /// Print full entity registry report
        #[arg(long)]
        verbose: bool,
    },

    /// Compare registry metrics against actual repo contents
    Refresh {
        /// Root directory containing ecoPrimals checkout
        repos_root: PathBuf,

        /// Write updated metrics back to config.toml
        #[arg(long)]
        write: bool,

        /// Refresh only this entity ID
        #[arg(long)]
        source: Option<String>,
    },

    /// Render Jupyter notebooks to Zola-compatible markdown pages
    RenderNotebooks {
        /// Directories containing .ipynb files (optional if --discover)
        #[arg(required_unless_present = "discover")]
        dirs: Vec<PathBuf>,

        /// Root of springs/ directory to auto-discover spring notebooks
        #[arg(long)]
        springs: Option<PathBuf>,

        /// Auto-discover notebooks from ecosystem workspace (walks up to .gate)
        #[arg(long)]
        discover: bool,
    },

    /// Fetch upstream repos and refresh metrics
    FetchRefresh {
        /// Write updated metrics back to config.toml
        #[arg(long)]
        write: bool,

        /// Refresh only this source ID
        #[arg(long)]
        source: Option<String>,
    },

    /// Validate internal links in content/ (pure Rust, no external tools)
    CheckLinks,

    /// Generate or verify BLAKE3 content-addressed manifest
    Provenance {
        /// Verify existing manifest instead of generating
        #[arg(long)]
        verify: bool,

        /// Show per-page diff against previous manifest
        #[arg(long)]
        diff: bool,

        /// Write manifest to content-manifest.toml
        #[arg(long)]
        write: bool,
    },

    /// Build and emit the entity graph (renvois de choses)
    Graph {
        /// Emit JSON graph to static/graph/entity-graph.json
        #[arg(long)]
        emit: bool,
    },

    /// Certify the site — emit or validate the guideStone manifest
    Certify {
        /// Write certification manifest to static/certification/manifest.json
        #[arg(long)]
        emit: bool,
    },

    /// Generate CAS manifest for build output (`NestGate` integration)
    CasManifest {
        /// Path to Zola build output (default: public/)
        #[arg(long, default_value = "public")]
        public_dir: PathBuf,

        /// Write manifest to static/cas/build-manifest.json
        #[arg(long)]
        emit: bool,
    },

    /// Show self-capabilities and discover peer primals
    Discover,

    /// Validate running NUCLEUS against a deployment profile
    Nucleus {
        /// Path to the NUCLEUS profile TOML (e.g., profiles/flockgate-wan.toml)
        #[arg(long)]
        profile: PathBuf,

        /// Send `health.ping` IPC to each discovered socket (verifies responsiveness)
        #[arg(long)]
        probe: bool,

        /// Test riboCipher mito-beacon signal acceptance (requires --probe)
        #[arg(long)]
        ribocipher: bool,
    },

    /// Verify depot binary integrity against BLAKE3 checksums
    DepotVerify {
        /// Path to checksums.toml (plasmidBin manifest)
        #[arg(long)]
        checksums: PathBuf,

        /// Path to local depot directory containing binaries
        #[arg(long, required_unless_present = "list_arches")]
        depot: Option<PathBuf>,

        /// Target architecture to verify (e.g., x86_64-unknown-linux-musl)
        #[arg(long, required_unless_present = "list_arches")]
        arch: Option<String>,

        /// Pass if all present binaries verify (allow incomplete depot)
        #[arg(long)]
        partial: bool,

        /// List available architectures and binary counts from checksums
        #[arg(long)]
        list_arches: bool,
    },

    /// Push build artifacts to `NestGate` CAS (content-addressed storage)
    CasPush {
        /// Path to Zola build output (default: public/)
        #[arg(long, default_value = "public")]
        public_dir: PathBuf,

        /// Override `NestGate` socket path (default: auto-discover)
        #[arg(long, env = "NESTGATE_SOCKET")]
        socket: Option<String>,

        /// Generate manifest on-the-fly instead of reading from disk
        #[arg(long)]
        generate: bool,
    },

    /// Render content via petalTongue IPC (backend wiring validation)
    PtRender {
        /// Content path to render (e.g., `architecture/PRIMAL_CATALOG`)
        path: String,

        /// Output modality: omit for full HTML, "description" for summary
        #[arg(long)]
        modality: Option<String>,

        /// Override petalTongue socket path (default: auto-discover)
        #[arg(long, env = "PETALTONGUE_SOCKET")]
        socket: Option<String>,
    },

    /// Show petalTongue backend status (socket discovery + method probing)
    PtStatus,

    /// Probe Tower primals for P1 method availability
    TowerStatus,

    /// Request a visualization from petalTongue (SVG or scene-JSON)
    PtViz {
        /// Visualization name (e.g., "entity-graph", "kderm-topology")
        name: String,

        /// Output format: "svg" (default) or "scene-json"
        #[arg(long, default_value = "svg")]
        format: String,

        /// Override petalTongue socket path (default: auto-discover)
        #[arg(long, env = "PETALTONGUE_SOCKET")]
        socket: Option<String>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("  ERROR: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    let root = cli.root.canonicalize().unwrap_or_else(|_| cli.root.clone());

    if let Some(result) = dispatch_standalone(&cli) {
        return result;
    }

    let config_path = root.join(paths::CONFIG_FILE);
    let config = model::parse_config(&config_path)?;

    match cli.command {
        None | Some(Command::Validate { .. }) => {
            let (check, strict, verbose) = match &cli.command {
                Some(Command::Validate {
                    check,
                    strict,
                    verbose,
                }) => (*check, *strict, *verbose),
                _ => (false, false, false),
            };
            commands::validate(&root, &config, check, strict, verbose)
        }
        Some(Command::Refresh {
            repos_root,
            write,
            source,
        }) => commands::refresh(&config_path, &config, &repos_root, write, source.as_deref()),
        Some(Command::RenderNotebooks {
            dirs,
            springs,
            discover,
        }) => {
            let effective_springs = if discover {
                discover_springs_root(&root).or(springs)
            } else {
                springs
            };
            commands::render_notebooks(&root, &dirs, effective_springs.as_deref());
            Ok(())
        }
        Some(Command::FetchRefresh { write, source }) => {
            commands::fetch_refresh(&root, &config_path, &config, write, source.as_deref())
        }
        Some(Command::CheckLinks) => commands::check_links(&root),
        Some(Command::Provenance {
            verify,
            diff,
            write,
        }) => commands::provenance(&root, verify, diff, write),
        Some(Command::Graph { emit }) => commands::graph(&root, &config, emit),
        Some(Command::Certify { emit }) => commands::certify(&root, &config, emit),
        Some(Command::Discover) => commands::discover(),
        Some(
            Command::Nucleus { .. }
            | Command::DepotVerify { .. }
            | Command::PtRender { .. }
            | Command::PtStatus
            | Command::TowerStatus
            | Command::PtViz { .. },
        ) => {
            unreachable!("handled above")
        }
        Some(Command::CasManifest { public_dir, emit }) => {
            commands::cas_manifest(&root, &public_dir, emit)
        }
        Some(Command::CasPush {
            public_dir,
            socket,
            generate,
        }) => commands::cas_push(&root, &public_dir, socket.as_deref(), generate),
    }
}

/// Dispatch commands that don't require `config.toml`.
/// Returns `Some(result)` if handled, `None` if the caller should continue.
fn dispatch_standalone(cli: &Cli) -> Option<Result<(), Error>> {
    match &cli.command {
        Some(Command::Discover) => Some(commands::discover()),
        Some(Command::Nucleus {
            profile,
            probe,
            ribocipher,
        }) => Some(run_nucleus(profile, *probe, *ribocipher)),
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

/// Run NUCLEUS profile validation.
fn run_nucleus(profile_path: &Path, probe: bool, ribocipher: bool) -> Result<(), Error> {
    let profile = nucleus::parse_profile(profile_path)?;
    let result = nucleus::validate_profile(&profile, probe, ribocipher);

    nucleus_display::print_result(&profile, &result, profile_path);

    if result.passed() {
        println!("  RESULT: ✅ NUCLEUS COMPLIANT");
        Ok(())
    } else {
        println!("  RESULT: ❌ NUCLEUS NON-COMPLIANT");
        Err(Error::Config("NUCLEUS validation failed".into()))
    }
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

/// Walk up from `start` looking for a `.gate` file, then derive the springs root.
fn discover_springs_root(start: &std::path::Path) -> Option<PathBuf> {
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
