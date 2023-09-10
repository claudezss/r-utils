#[cfg(feature = "cli")]
pub mod cli {
    pub mod interface;
    pub mod schemas;
    pub mod sync;
}

#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "s3")]
pub use s3::{
    create_s3_client, get_object_content, get_objects, s3_client, upload_object,
};

#[cfg(feature = "mfa")]
pub mod mfa;
#[cfg(feature = "mfa")]
pub use mfa::get_totp;

pub fn hello() {
    println!("hello, this is r-utils 🫡")
}
