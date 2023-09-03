#[cfg(feature = "s3-sync")]
pub mod s3_sync;
#[cfg(feature = "s3-sync")]
pub use s3_sync::{sync, S3SyncArgs};

#[cfg(feature = "s3")]
pub mod s3;

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
    #[cfg(feature = "s3-sync")]
    S3Sync(S3SyncArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        #[cfg(feature = "s3-sync")]
        Commands::S3Sync(args) => sync(&args).await,
    }
}
