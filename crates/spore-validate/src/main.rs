// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = "sporePrint validation CLI — entity registry, content integrity, and metric sync."]

use clap::Parser;
use std::process::ExitCode;

mod cas;
mod cas_push;
mod certify;
mod cli;
mod commands;
mod commands_depot;
mod commands_discover;
mod commands_provenance;
mod commands_validate;
mod content;
mod depot;
mod discovery;
mod dispatch;
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
mod nucleus_probe;
mod paths;
mod petaltongue;
mod provenance;
mod refresh;
mod registry;
mod report;
mod time;
mod totals;
mod tower;

use cli::Cli;
use dispatch::{dispatch_standalone, dispatch_with_config};
use error::Error;

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

    dispatch_with_config(&cli, &root)
}
