// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![doc = "sporePrint validation CLI — entity registry, content integrity, and metric sync."]

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod content;
mod error;
mod fetch;
mod links;
mod model;
mod notebook;
mod provenance;
mod refresh;
mod registry;
mod report;
mod time;
mod totals;

use error::{Diagnostic, Error};

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
    let config_path = root.join("config.toml");
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
            run_validate(&root, &config, check, strict, verbose)
        }
        Some(Command::Refresh {
            repos_root,
            write,
            source,
        }) => run_refresh(&config_path, &config, &repos_root, write, source.as_deref()),
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
            run_render_notebooks(&root, &dirs, effective_springs.as_deref());
            Ok(())
        }
        Some(Command::FetchRefresh { write, source }) => {
            run_fetch_refresh(&root, &config_path, &config, write, source.as_deref())
        }
        Some(Command::CheckLinks) => run_check_links(&root),
        Some(Command::Provenance {
            verify,
            diff,
            write,
        }) => run_provenance(&root, verify, diff, write),
    }
}

fn run_validate(
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
    totals::validate(
        &config.extra.entity_registry,
        &config.extra.totals,
        &mut diagnostics,
    );

    let content_dir = root.join("content");
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
            if let Diagnostic::Warning(msg) = diag {
                *diag = Diagnostic::Error(format!("(strict) {msg}"));
            }
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

fn run_refresh(
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
        let pct = if d.registered > 0 {
            #[allow(clippy::cast_precision_loss)]
            let diff = d.actual as f64 - d.registered as f64;
            #[allow(clippy::cast_precision_loss)]
            let base = d.registered as f64;
            format!("{:+.1}%", diff / base * 100.0)
        } else {
            "new".to_string()
        };
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

/// Walk up from `start` looking for a `.gate` file, then derive the springs root.
fn discover_springs_root(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        if dir.join(".gate").is_file() {
            let springs = dir.join("springs");
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

fn run_render_notebooks(root: &Path, dirs: &[PathBuf], springs: Option<&Path>) {
    println!("spore-validate: rendering notebooks to Zola markdown...");

    let (count, messages) = notebook::render_notebooks(root, dirs, springs);

    for msg in &messages {
        println!("  {msg}");
    }

    println!("\n  Rendered {count} notebook(s)");
}

fn run_check_links(root: &Path) -> Result<(), Error> {
    let content_root = root.join("content");
    if !content_root.is_dir() {
        return Err(Error::Config("content/ directory not found".into()));
    }

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

fn run_provenance(root: &Path, verify: bool, diff: bool, write: bool) -> Result<(), Error> {
    let content_dir = root.join("content");
    if !content_dir.is_dir() {
        return Err(Error::Config("content/ directory not found".into()));
    }

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

fn run_fetch_refresh(
    root: &Path,
    config_path: &Path,
    config: &model::Config,
    write: bool,
    source: Option<&str>,
) -> Result<(), Error> {
    println!("spore-validate: fetching upstream repos...");

    let messages = fetch::fetch_and_refresh(root, source);
    for msg in &messages {
        println!("{msg}");
    }

    let clone_root = fetch::clone_dir();
    println!("\nspore-validate: scanning for metric drift...");
    run_refresh(config_path, config, &clone_root, write, source)
}
