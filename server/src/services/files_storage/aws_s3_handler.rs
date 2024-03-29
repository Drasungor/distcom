use std::{env, path::Path};

use async_trait::async_trait;
use aws_sdk_s3 as s3;
use aws_config::{self, meta::region::RegionProviderChain, Region};
use s3::primitives::ByteStream;
// use std::path::Path;

use crate::common::app_error::{AppError, AppErrorType};
use super::file_storage::FileStorage;

pub struct AwsS3Handler {
    s3_client: Option<s3::Client>,
    bucket_name: String,
}

#[async_trait]
impl FileStorage for AwsS3Handler {


    async fn set_up_connection(&mut self) -> Result<(), AppError> {
        // let my_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

        // let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));


        
        

        // let key_id = env::var("AWS_ACCESS_KEY_ID").expect("No AWS_ACCESS_KEY_ID environment variable");
        // let key_secret = env::var("AWS_SECRET_ACCESS_KEY").expect("No AWS_SECRET_ACCESS_KEY environment variable");

        let key_id = ""; // TODO: GET VALUES FROM THE CONFIG VALUE
        let key_secret = ""; // TODO: GET VALUES FROM THE CONFIG VALUE


        let region = Region::new("us-east-1"); // TODO: GET VALUES FROM THE CONFIG VALUE
        let cred = s3::config::Credentials::new(key_id, key_secret, None, None, "Loaded-from-custom-env");
        let conf_builder = s3::config::Builder::new().region(region).credentials_provider(cred);
        let conf = conf_builder.build();

        // self.s3_client = Some(s3::Client::new(&my_config));
        self.s3_client = Some(s3::Client::from_conf(conf));
        Ok(())
    }

    async fn upload(&self, file_path: &Path) -> Result<(), AppError> {
    // Path::new();
        if (!file_path.exists()) {
            println!("The path does not exist");
            return Err(AppError::new(AppErrorType::InternalServerError));
        }
        
        let key: &str;
        match file_path.to_str() {
            Some(stringified_path) => {
                key = stringified_path;
            },
            None => {
                println!("Path conversion error");
                return Err(AppError::new(AppErrorType::InternalServerError));
            }
        }

        let body: ByteStream;
        match ByteStream::from_path(file_path).await {
            Ok(generated_bytestream) => {
                body = generated_bytestream;
            },
            // Err(_) => {
            Err(error) => {
                println!("Bytestream generation error: {}", error);
                return Err(AppError::new(AppErrorType::InternalServerError));
            }
        }

        let content_type = mime_guess::from_path(file_path).first_or_octet_stream().to_string();

        println!("Content type: {}", content_type);
        println!("Bucket name: {}", self.bucket_name.clone());
        println!("Key: {}", key);
        println!("Bytestream: {:?}", body);

        let req = self.s3_client.as_ref().expect("S3 client not initialized").put_object().bucket(self.bucket_name.clone()).key(key).
                                            body(body).content_type(content_type);

        // match req.send().await {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(AppError::new(AppErrorType::InternalServerError)),
        // }

        match req.send().await {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error in request send: {:?}", error);
                Err(AppError::new(AppErrorType::InternalServerError))
            }
        }
    }

    async fn delete(&self) -> Result<(), AppError> {
        Ok(())
    }



}

impl AwsS3Handler {

    // TODO: Check if this function can be async, or if the initialization of s3_client should be done in another method
    pub fn new(s3_conection_data: &str) -> AwsS3Handler {
        // let my_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        return AwsS3Handler {
            // s3_client: s3::Client::new(&my_config),
            s3_client: None,
            bucket_name: s3_conection_data.to_string(), // TODO: GET BUCKET VALUE FROM THE CONFIG VALUE
        };
    }

}