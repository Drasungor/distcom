use async_trait::async_trait;
use aws_sdk_s3 as s3;
use aws_config;

use crate::common::app_error::AppError;
use super::file_storage::FileStorage;

pub struct AwsS3Handler {
    s3_client: s3::Client,
}

#[async_trait]
impl FileStorage for AwsS3Handler {



    async fn upload(&self, file_path: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn delete(&self) -> Result<(), AppError> {
        Ok(())
    }



}

impl AwsS3Handler {

    // TODO: Check if this function can be async, or if the initialization of s3_client should be done in another method
    async fn new(uploaded_files_url: &str) -> Result<AwsS3Handler, AppError> {
        let my_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        return Ok(AwsS3Handler {
            s3_client: s3::Client::new(&my_config),
        });
    }

}