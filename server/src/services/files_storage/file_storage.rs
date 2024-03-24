use async_trait::async_trait;

use crate::common::app_error::AppError;

#[async_trait]
pub trait FileStorage {
    async fn upload(&self, file_path: &str) -> Result<(), AppError>;
    async fn delete(&self) -> Result<(), AppError>;
}
