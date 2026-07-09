// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "spore-validate",
    about = "Typed validation tooling for the sporePrint entity registry and content",
    version
)]
pub struct Cli {
    /// Path to sporePrint root
    #[arg(short, long, default_value = ".", global = true)]
    pub root: PathBuf,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
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

    /// Generate static SVG visualizations via petalTongue IPC
    BuildViz {
        /// Override petalTongue socket path (default: auto-discover)
        #[arg(long, env = "PETALTONGUE_SOCKET")]
        socket: Option<String>,
    },
}
