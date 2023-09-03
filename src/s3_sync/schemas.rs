use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Debug)]
pub struct S3SyncArgs {
    pub token: String,
    pub path: std::path::PathBuf,
    pub target_s3_path: String,
    pub download: Option<bool>,
    pub upload: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AWSConfig {
    #[serde(rename = "AWS_ACCESS_KEY_ID")]
    pub key: String,
    #[serde(rename = "AWS_DEFAULT_REGION")]
    pub region: String,
    #[serde(rename = "AWS_SECRET_ACCESS_KEY")]
    pub secret: String,
}
