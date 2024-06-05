use std::path::Path;

use async_trait::async_trait;

use crate::common::app_error::AppError;

#[async_trait]
pub trait FileStorage {
    async fn set_up_connection(&mut self) -> Result<(), AppError>;
    async fn download(&self, object_name: &str, storage_path: &Path) -> Result<(), AppError>;
    async fn download_program(&self, file_path: &Path, organization_id: &str, program_id: &str) -> Result<(), AppError>;
    async fn upload(&self, file_path: &Path, new_object_name: &str) -> Result<(), AppError>;
    async fn upload_program(&self, file_path: &Path, organization_id: &str, program_id: &str) -> Result<(), AppError>;
    async fn delete(&self) -> Result<(), AppError>;
}
