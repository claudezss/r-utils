#![allow(unreachable_patterns)]

use crate::cli::schemas::S3SyncArgs;
use crate::cli::sync::sync;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    S3Sync(S3SyncArgs),
}

pub async fn cli_entry() {
    let _cli = Cli::parse();
    match _cli.command {
        Commands::S3Sync(args) => sync(&args).await,
        _ => unreachable!(),
    }
}
