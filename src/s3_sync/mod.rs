pub mod schemas;
pub use schemas::{AWSConfig, S3SyncArgs};

use curl::easy::Easy;
use serde_json::{from_slice, from_str, Value};
use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::s3::{create_s3_client, get_objects};

pub async fn sync(args: &S3SyncArgs) {
    let url = format!("https://api.claudezss.com/secret?token={}", args.token);

    let mut download: bool = args.download.unwrap_or(true);
    let mut upload: bool = args.upload.unwrap_or(false);

    if download == upload && download == true {
        download = true;
        upload = false;
    } else if download == upload && download == false {
        download = true;
        upload = false;
    }

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
        println!("Invalid Token")
    } else {
        let aws_content = content["aws"].to_string();
        let aws_config: AWSConfig = from_str(&aws_content).unwrap();
        let client =
            create_s3_client(&aws_config.region, &aws_config.key, &aws_config.secret).await;
        let files = get_objects(&client, "claudezss", &args.target_s3_path, Some(true))
            .await
            .unwrap();

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
    }
}
