use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::{config::Region, Client, primitives::ByteStream, Error};
use std::path::Path;
use uuid::Uuid;


// #[derive(Debug)]
// pub struct S3Service {
//     /// The AWS Region.
//     region: Option<String>,
//     /// The name of the bucket.
//     bucket: String,
//     /// The name of the file to upload.
//     filename: String,
//     /// The name of the object in the bucket.
//     key: String,
//     /// Whether to display additional information.
//     verbose: bool,
// }

pub async fn upload_object(
    // client: &Client,
    // bucket: &str,
    filename: &str,
    // key: &str,
) -> Result<String, Error> {

    let region_provider = RegionProviderChain::first_try(Region::new("ap-southeast-1"));
    let bucket = std::env::var("AWS_S3_BUCKET").unwrap_or("user-profilepic-bucket".to_owned());

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    let bucket_url = std::env::var("BUCKET_URL").unwrap();

    let body = ByteStream::from_path(Path::new(filename)).await;

    let key = format!("assets/{}", Uuid::new_v4());

    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(bucket)
                .key(key.as_str())
                .body(b)
                .send()
                .await?;

            println!("Upload success. Version: {:?}", resp.version_id);

        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{}", e);
        }
    }
    let uploaded_url = format!("{}/{}", bucket_url, &key);

    Ok(uploaded_url)
}
