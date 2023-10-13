// use aws_sdk_s3 as s3;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::{
    create_multipart_upload::CreateMultipartUploadOutput, get_object::GetObjectOutput,
};
use aws_sdk_s3::{config::Region, Client as S3Client};
use aws_smithy_http::byte_stream::{ByteStream, Length};
use s3_service::error::Error;
use std::process;
use uuid::Uuid;

//In bytes, minimum chunk size of 5MB. Increase CHUNK_SIZE to send larger chunks.
const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

pub struct S3BucketService {
    s3_client:Client,
    bucket_name:String,
}

impl S3BucketService {
    pub fn init() -> Self {
        let aws_configuration = aws_config::load_from_env().await;

        //create aws s3 client
        let s3_client = S3Client::new(&aws_configuration);
    }

    async fn upload_image(&self, object_key: &str,
        mut files: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
        // get the name of aws bucket from env variable
        let bucket = std::env::var("AWS_S3_BUCKET").unwrap_or("my-bucket-name".to_owned());
        // if you have a public url for your bucket, place it as ENV variable BUCKET_URL    
        //get the public url for aws bucket
        let bucket_url = std::env::var("BUCKET_URL").unwrap_or(bucket.to_owned());
        // we are going to store the respose in HashMap as filename: url => key: value
        let mut res = HashMap::new();
        
        // send the urls in response
        Ok(Json(serde_json::json!(res)))
    }
}