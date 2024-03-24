use async_trait::async_trait;

use crate::common::app_error::AppError;

use super::file_storage::FileStorage;

pub struct AwsS3Handler;

#[async_trait]
impl FileStorage for AwsS3Handler {

    async fn upload(&self, file_path: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn delete(&self) -> Result<(), AppError> {
        Ok(())
    }

}