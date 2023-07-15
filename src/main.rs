/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod config;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Syncronize a mirror
    Sync {
        /// Name of the mirror to synchronize
        #[arg(short, long, required=true)]
        mirror: String,
    },
    /// Dump status of Mirrorshine
    Status {

    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config) = cli.config.as_deref() {
        println!("Config file path: {}", config.display());
    }
    match &cli.command {
        Some(Commands::Sync{mirror}) => {
            println!("TOOD: syncing mirror {}", mirror);
        }
        Some(Commands::Status {}) => {
            println!("TODO: status display");
        }
        None => {
        }
    }
}