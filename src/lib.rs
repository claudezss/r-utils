#[cfg(feature = "s3")]
pub mod s3;

#[cfg(feature = "mfa")]
pub mod mfa;

pub fn hello() {
    println!("hello, this is r-utils 🫡")
}
