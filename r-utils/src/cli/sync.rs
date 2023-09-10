use crate::cli::schemas::{AWSConfig, S3SyncArgs};
use crate::s3::{create_s3_client, get_objects, s3_client, upload_object};
use aws_sdk_s3::Client;
use curl::easy::Easy;
use serde_json::{from_slice, from_str, Value};
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;

async fn get_secrets(token: &str) -> Value {
    let url = format!("https://api.claudezss.com/secret?token={}", token);
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
    content
}

pub async fn sync(args: &S3SyncArgs) {
    let upload: bool = args.upload.unwrap_or(false);

    let client: Client;

    if std::env::var("AWS_ACCESS_KEY_ID").is_ok()
        && std::env::var("AWS_DEFAULT_REGION").is_ok()
        && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok()
    {
        println!("Loading S3 secrets from env vars");
        client = s3_client().await;
    } else {
        println!("Fetching S3 secrets from host");
        let secrets = get_secrets(&args.token).await;

        if secrets.get("aws").is_none() {
            println!("Invalid Token");
            return;
        }

        let aws_content = secrets["aws"].to_string();
        let aws_config: AWSConfig = from_str(&aws_content).unwrap();
        client = create_s3_client(
            &aws_config.region,
            &aws_config.key,
            &aws_config.secret,
        )
        .await;
    }

    println!("Fetching objects from prefix: {}", &args.target_s3_path);
    let files =
        get_objects(&client, "claudezss", &args.target_s3_path, Some(!upload))
            .await
            .unwrap();

    if !upload {
        // pull files from s3
        if !args.path.exists() {
            println!("Creating folder: {}", args.path.to_str().unwrap());
            create_dir_all(&args.path).unwrap();
        }

        for f in files {
            let path_str =
                format!("{}/{}", args.path.to_str().unwrap(), f.name);
            let path = std::path::Path::new(&path_str).to_str().unwrap();

            println!("Writing file: {}", path);
            let mut output = File::create(path).unwrap();
            output.write_all(&f.content).unwrap();
            // write!(output, "{}", f.content).unwrap()
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
            let target_key_path =
                std::path::Path::new(&target_key_str).to_str().unwrap();

            println!("Uploading object {} to {}", &file_path, target_key_path);
            upload_object(&client, "claudezss", &file_path, target_key_path)
                .await;
        }
    }
}
