use std::path::Path;

use async_trait::async_trait;

use crate::common::app_error::AppError;

#[async_trait]
pub trait FileStorage {
    async fn set_up_connection(&mut self) -> Result<(), AppError>;
    // async fn upload(&self, file_path: &str) -> Result<(), AppError>;
    
    // async fn upload(&self, file_path: &Path) -> Result<(), AppError>;
    async fn upload(&self, file_path: &Path, new_object_name: &str) -> Result<(), AppError>;
    async fn delete(&self) -> Result<(), AppError>;
}
