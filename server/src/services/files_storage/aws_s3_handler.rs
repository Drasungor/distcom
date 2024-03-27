use std::path::Path;

use async_trait::async_trait;
use aws_sdk_s3 as s3;
use aws_config;
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
        let my_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        self.s3_client = Some(s3::Client::new(&my_config));
        Ok(())
    }

    async fn upload(&self, file_path: &Path) -> Result<(), AppError> {
    // Path::new();
        if (!file_path.exists()) {
            return Err(AppError::new(AppErrorType::InternalServerError));
        }
        
        let key: &str;
        match file_path.to_str() {
            Some(stringified_path) => {
                key = stringified_path;
            },
            None => {
                return Err(AppError::new(AppErrorType::InternalServerError));
            }
        }

        let body: ByteStream;
        match ByteStream::from_path(file_path).await {
            Ok(generated_bytestream) => {
                body = generated_bytestream;
            },
            Err(_) => {
                return Err(AppError::new(AppErrorType::InternalServerError));
            }
        }

        let content_type = mime_guess::from_path(file_path).first_or_octet_stream().to_string();
        let req = self.s3_client.as_ref().expect("S3 client not initialized").put_object().bucket(self.bucket_name.clone()).key(key).
                                            body(body).content_type(content_type);

        match req.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::new(AppErrorType::InternalServerError)),
        }
    }

    async fn delete(&self) -> Result<(), AppError> {
        Ok(())
    }



}

impl AwsS3Handler {

    // TODO: Check if this function can be async, or if the initialization of s3_client should be done in another method
    pub fn new(bucket_name: &str) -> AwsS3Handler {
        // let my_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        return AwsS3Handler {
            // s3_client: s3::Client::new(&my_config),
            s3_client: None,
            bucket_name: bucket_name.to_string(),
        };
    }

}