// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![doc = "sporePrint validation CLI — entity registry, content integrity, and metric sync."]

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod cas;
mod cas_push;
mod certify;
mod commands;
mod content;
mod discovery;
mod error;
mod fetch;
mod graph;
mod http;
mod links;
mod model;
mod notebook;
#[allow(dead_code)]
mod nucleus;
mod paths;
mod provenance;
mod refresh;
mod registry;
mod report;
mod time;
mod totals;

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

    // Commands that don't require config.toml
    if matches!(cli.command, Some(Command::Discover)) {
        return commands::discover();
    }
    if let Some(Command::Nucleus { ref profile, probe }) = cli.command {
        return run_nucleus(profile, probe);
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
        Some(Command::Nucleus { .. }) => unreachable!("handled above"),
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

/// Run NUCLEUS profile validation.
fn run_nucleus(profile_path: &Path, probe: bool) -> Result<(), Error> {
    let profile = nucleus::parse_profile(profile_path)?;
    let result = nucleus::validate_profile(&profile, probe);

    println!("sporePrint: NUCLEUS profile validation");
    println!("  Profile: {} ({})", result.profile_name, profile_path.display());
    if let Some(desc) = &profile.profile.description {
        println!("  Description: {desc}");
    }
    if let Some(base) = profile.profile.base() {
        println!("  Extends: {base}");
    }
    println!("  Declared primals: {}", result.total_declared);
    if !profile.launch_order().is_empty() {
        println!("  Launch order: {}", profile.launch_order().join(" → "));
    }
    if profile.federation_enabled() {
        println!("  Federation: enabled");
    }
    println!();

    if !result.healthy.is_empty() {
        println!("  HEALTHY ({}/{}):", result.healthy.len(), result.total_declared);
        for p in &result.healthy {
            let probe_info = p.probe.as_ref().map_or_else(String::new, |pr| {
                format!(
                    " ({}ms{})",
                    pr.latency.as_millis(),
                    pr.version.as_deref().map_or(String::new(), |v| format!(", v{v}"))
                )
            });
            println!(
                "    ✅ {} [{}] → {}{}",
                p.name,
                p.role,
                p.socket_path.as_deref().unwrap_or("?"),
                probe_info
            );
        }
    }

    if !result.missing.is_empty() {
        println!();
        println!("  MISSING ({}/{}):", result.missing.len(), result.total_declared);
        for p in &result.missing {
            let marker = if p.required { "❌" } else { "⚠️" };
            let probe_err = p.probe.as_ref()
                .and_then(|pr| pr.error.as_deref())
                .map_or(String::new(), |e| format!(" — {e}"));
            println!("    {marker} {} [{}] (required={}){probe_err}", p.name, p.role, p.required);
        }
    }

    println!();
    println!(
        "  Critical path: {}",
        if result.critical_met { "✅ MET" } else { "❌ FAILED" }
    );
    println!(
        "  Min healthy: {}",
        if result.min_healthy_met { "✅ MET" } else { "❌ FAILED" }
    );
    println!();

    if result.passed() {
        println!("  RESULT: ✅ NUCLEUS COMPLIANT");
        Ok(())
    } else {
        println!("  RESULT: ❌ NUCLEUS NON-COMPLIANT");
        Err(Error::Config("NUCLEUS validation failed".into()))
    }
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
