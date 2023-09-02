pub mod args;
pub use args::S3SyncArgs;

pub fn sync(args: &S3SyncArgs) {
    println!("{}", args.token);
    println!("{}", args.path.is_dir());
}
