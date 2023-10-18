// use aws_sdk_s3 as s3;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::{get_object::GetObjectOutput, put_object::{PutObjectError, PutObjectOutput}};
use aws_sdk_s3::{config::Region, Client as S3Client, primitives::ByteStream, error::SdkError};
// use aws_smithy_http::byte_stream::{ByteStream, Length};
// use error::Error;
use std::path::Path;
// use std::process;
// use uuid::Uuid;

//In bytes, minimum chunk size of 5MB. Increase CHUNK_SIZE to send larger chunks.
const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

pub struct S3BucketService {
    s3_client:S3Client,
    bucket_name: String,
}

impl S3BucketService {
    pub fn init() -> Self {
        let region_provider = RegionProviderChain::first_try(Region::new("ap-southeast-1"));
        // let region = region_provider.region().unwrap();
    
        let shared_config = aws_config::from_env().region(region_provider).load().await;
        
        // let aws_configuration = aws_config::load_from_env();

        //create aws s3 client
        let s3_client = S3Client::new(&shared_config);
        let bucket_name = "user-profilepic-bucket".to_string();
        S3BucketService { 
            s3_client: s3_client, 
            bucket_name: bucket_name 
        }
    }

    async fn upload_image(&self, object_key: &str,
        file_name: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        // get the name of aws bucket from env variable
        // let bucket = std::env::var("AWS_S3_BUCKET").unwrap_or("my-bucket-name".to_owned());
        // if you have a public url for your bucket, place it as ENV variable BUCKET_URL    
        //get the public url for aws bucket
        // let bucket_url = std::env::var("BUCKET_URL").unwrap_or(bucket.to_owned());
        // we are going to store the respose in HashMap as filename: url => key: value
        // let mut res = HashMap::new();
        
        // send the urls in response
        // Ok(Json(serde_json::json!(res)))

        let body = ByteStream::from_path(Path::new(file_name)).await;
        self.s3_client
            .put_object()
            .bucket(self.bucket_name.to_owned())
            .key(object_key)
            .body(body.unwrap())
            .send()
            .await
    }

    pub fn generate_public_url(&self, object_key: &str) -> String {
        //get the public url for aws bucket
        let bucket_url = std::env::var("BUCKET_URL").unwrap();
        format!("{}/{}", bucket_url, object_key)
    }
}