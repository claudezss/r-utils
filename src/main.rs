// S3 Sync
#[cfg(feature = "s3-sync")]
pub mod s3_sync;
#[cfg(feature = "s3-sync")]
pub use s3_sync::{sync, S3SyncArgs};
// S3
#[cfg(feature = "s3")]
pub mod s3;

// MFA
#[cfg(feature = "mfa")]
pub mod mfa;

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use r_utils::hello;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "s3-sync")]
    S3Sync(S3SyncArgs),
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "cli")]
    {
        let _cli = Cli::parse();
        match _cli.command {
            #[cfg(feature = "s3-sync")]
            Commands::S3Sync(args) => sync(&args).await,
            _ => unreachable!(),
        }
    }

    if !cfg!(feature = "cli") {
        hello();
    }
}
