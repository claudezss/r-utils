use clap::{Args};

#[derive(Args, Debug)]
pub struct S3SyncArgs {
    pub token: String,
    pub path: std::path::PathBuf,
}
