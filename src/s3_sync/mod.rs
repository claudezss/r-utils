pub mod schemas;
pub use schemas::{AWSConfig, S3SyncArgs};

use curl::easy::Easy;
use serde_json::{from_slice, from_str, Value};
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;

use crate::s3::{create_s3_client, get_objects, upload_object};

pub async fn sync(args: &S3SyncArgs) {
    let url = format!("https://api.claudezss.com/secret?token={}", args.token);

    let upload: bool = args.upload.unwrap_or(false);

    let mut content: Value = Value::default();

    {
        let mut easy = Easy::new();
        easy.url(&url).unwrap();
        _ = easy.ssl_verify_host(false);

        let mut transfer = easy.transfer();

        transfer
            .write_function(|data| {
                content = from_slice(data).unwrap_or(Value::from(""));
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    if content.get("aws").is_none() {
        println!("Invalid Token");
        return;
    }

    let aws_content = content["aws"].to_string();
    let aws_config: AWSConfig = from_str(&aws_content).unwrap();
    let client = create_s3_client(&aws_config.region, &aws_config.key, &aws_config.secret).await;
    let files = get_objects(&client, "claudezss", &args.target_s3_path, Some(true))
        .await
        .unwrap();

    if !upload {
        // pull files from s3
        if !args.path.exists() {
            println!("Creating folder: {}", args.path.to_str().unwrap());
            create_dir_all(&args.path).unwrap();
        }

        for f in files {
            println!("Writing file: {}", f.name);
            let path = format!("{}/{}", args.path.to_str().unwrap(), f.name);
            let mut output = File::create(path).unwrap();
            write!(output, "{}", f.content).unwrap()
        }
    } else {
        if !args.path.exists() {
            println!("Path doesn't exist");
            return;
        };

        let local_files = read_dir(&args.path).unwrap();

        for f in local_files {
            let file_path = f.unwrap().path().to_str().unwrap().to_string();
            let path = std::path::Path::new(&file_path);
            let target_key_str = format!(
                "{}{}",
                args.target_s3_path,
                path.file_name().unwrap().to_str().unwrap()
            );
            let target_key_path = std::path::Path::new(&target_key_str);
            upload_object(
                &client,
                "claudezss",
                &file_path,
                target_key_path.to_str().unwrap(),
            )
            .await;
        }
    }
}
