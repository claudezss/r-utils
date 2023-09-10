use aws_sdk_s3 as s3;
use aws_sdk_s3::Error;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Debug)]
pub struct S3Object {
    pub key: String,
    pub last_modified: String,
    pub size: i64,
    pub name: String,
    pub content: Vec<u8>,
}

pub async fn s3_client() -> s3::Client {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    client
}

pub async fn create_s3_client(
    region: &str,
    key: &str,
    secret: &str,
) -> s3::Client {
    std::env::set_var("AWS_DEFAULT_REGION", region);
    std::env::set_var("AWS_ACCESS_KEY_ID", key);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", secret);
    s3_client().await
}

pub async fn get_object_content(
    client: &s3::Client,
    bucket_name: &str,
    file_path: &str,
) -> Result<Vec<u8>, Error> {
    let content = client
        .get_object()
        .bucket(bucket_name)
        .key(file_path)
        .send()
        .await
        .unwrap();
    let body: Vec<u8> = content.body.collect().await.unwrap().to_vec();
    // let body_str = std::str::from_utf8(&body).unwrap();
    // Ok(body_str.to_string())
    Ok(body)
}

pub async fn get_objects(
    client: &s3::Client,
    bucket_name: &str,
    prefix: &str,
    include_content: Option<bool>,
) -> Result<Vec<S3Object>, Error> {
    let include_content = include_content.unwrap_or(false);
    let objects = client
        .list_objects_v2()
        .bucket(bucket_name)
        .prefix(prefix)
        .send()
        .await
        .unwrap();

    let mut objects_vec: Vec<S3Object> = Vec::default();

    for obj in objects.contents().unwrap_or_default().iter() {
        let key = obj.key().unwrap();
        if key == prefix {
            continue;
        };

        let dt: DateTime<Utc> =
            SystemTime::try_from(obj.last_modified.unwrap())
                .unwrap()
                .into();
        let mut content = Vec::new();

        if include_content {
            content =
                get_object_content(client, bucket_name, &key).await.unwrap();
        }

        objects_vec.push(S3Object {
            key: key.to_string(),
            last_modified: dt.to_rfc3339().to_string(),
            size: obj.size(),
            name: key.replace(prefix, "").to_string(),
            content,
        })
    }

    Ok(objects_vec)
}

pub async fn upload_object(
    client: &s3::Client,
    bucket_name: &str,
    file_path: &str,
    key: &str,
) {
    let body =
        s3::primitives::ByteStream::from_path(std::path::Path::new(file_path))
            .await;

    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
        .unwrap();
}
