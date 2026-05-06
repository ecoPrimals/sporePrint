use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process;

mod content;
mod model;
mod refresh;
mod registry;
mod totals;

#[derive(Parser)]
#[command(
    name = "spore-validate",
    about = "Typed validation tooling for the sporePrint entity registry and content"
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
    },

    /// Compare registry metrics against actual repo contents
    Refresh {
        /// Root directory containing ecoPrimals checkout (with primals/, springs/, infra/)
        repos_root: PathBuf,

        /// Write updated metrics back to config.toml (otherwise print-only)
        #[arg(long)]
        write: bool,

        /// Refresh only this entity ID (default: all entities with repos)
        #[arg(long)]
        source: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let root = cli.root.canonicalize().unwrap_or_else(|_| cli.root.clone());
    let config_path = root.join("config.toml");

    let config = match model::parse_config(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("  ERROR: {e}");
            process::exit(1);
        }
    };

    match cli.command {
        None | Some(Command::Validate { .. }) => {
            let (check, strict) = match &cli.command {
                Some(Command::Validate { check, strict }) => (*check, *strict),
                _ => (false, false),
            };
            run_validate(&root, &config, check, strict);
        }
        Some(Command::Refresh {
            repos_root,
            write,
            source,
        }) => {
            run_refresh(&config_path, &config, &repos_root, write, source.as_deref());
        }
    }
}

fn run_validate(root: &PathBuf, config: &model::Config, check: bool, strict: bool) {
    println!("spore-validate: checking sporePrint entity registry...");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    registry::validate(&config.extra.entity_registry, &mut errors);
    totals::validate(
        &config.extra.entity_registry,
        &config.extra.totals,
        &mut errors,
    );

    let content_dir = root.join("content");
    if content_dir.is_dir() {
        content::validate_taxonomies(
            root,
            &content_dir,
            &config.extra.entity_registry,
            &mut errors,
            &mut warnings,
        );

        if check {
            content::check_integrity(
                root,
                &content_dir,
                &config.extra.entity_registry,
                &mut errors,
                &mut warnings,
            );
        }
    }

    if strict {
        errors.extend(
            warnings
                .drain(..)
                .map(|w| w.replace("WARN", "ERROR (strict)")),
        );
    }

    for w in &warnings {
        println!("  WARN:  {w}");
    }
    for e in &errors {
        println!("  ERROR: {e}");
    }

    let entity_count = config.extra.entity_registry.len();

    if errors.is_empty() {
        println!(
            "  OK: {entity_count} entities, {} warning(s), 0 errors",
            warnings.len()
        );
    } else {
        println!(
            "\n  {} error(s), {} warning(s)",
            errors.len(),
            warnings.len()
        );
        process::exit(1);
    }
}

fn run_refresh(
    config_path: &Path,
    config: &model::Config,
    repos_root: &PathBuf,
    write: bool,
    source: Option<&str>,
) {
    let repos_root = repos_root
        .canonicalize()
        .unwrap_or_else(|_| repos_root.clone());

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
        println!("  SKIP: {repo} — repo not found");
    }

    if result.drifts.is_empty() {
        println!(
            "  OK: {} repos scanned, all metrics match registry",
            result.scanned
        );
    } else {
        println!();
        for d in &result.drifts {
            let pct = if d.registered > 0 {
                let diff = d.actual as f64 - d.registered as f64;
                format!("{:+.1}%", diff / d.registered as f64 * 100.0)
            } else {
                "new".to_string()
            };
            println!(
                "  DRIFT: [{}] {} — registered: {}, actual: {} ({pct})",
                d.key, d.field, d.registered, d.actual
            );
        }
        println!(
            "\n  {} repos scanned, {} metric(s) drifted",
            result.scanned,
            result.drifts.len()
        );

        if write {
            match refresh::write_updates(config_path, &result.drifts) {
                Ok(()) => println!("  WRITE: config.toml updated with {} metric(s)", result.drifts.len()),
                Err(e) => {
                    eprintln!("  ERROR: failed to write config.toml: {e}");
                    process::exit(1);
                }
            }
        }
    }
}
