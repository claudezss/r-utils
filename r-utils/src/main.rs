use r_utils::hello;
pub mod s3;

#[cfg(feature = "cli")]
pub mod cli {
    pub mod interface;
    pub mod schemas;
    pub mod sync;
}
#[cfg(feature = "cli")]
use crate::cli::interface;

#[tokio::main]
async fn main() {
    if !cfg!(feature = "cli") {
        hello();
    } else {
        interface::cli_entry().await
    }
}
