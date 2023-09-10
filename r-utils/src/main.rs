use r_utils::hello;

#[cfg(feature = "s3")]
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
    #[cfg(feature = "cli")]
    {
        interface::cli_entry().await
    }
    if !cfg!(feature = "cli") {
        hello();
    }
}
