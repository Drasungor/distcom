use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

use async_trait::async_trait;
use aws_sdk_s3 as s3;
use aws_config::{self, meta::region::RegionProviderChain, Region};
use s3::primitives::ByteStream;
// use s3::primitives::ByteStream;
// use std::path::Path;

use crate::common::app_error::{AppError, AppErrorType, InternalServerErrorType};
use super::file_storage::FileStorage;

pub struct AwsS3Handler {
    s3_client: Option<s3::Client>,
    bucket_name: String,
    region: String,
    key_id: String,
    key_secret: String,
}

#[async_trait]
impl FileStorage for AwsS3Handler {


    async fn set_up_connection(&mut self) -> Result<(), AppError> {
        let region = Region::new(self.region.clone()); // TODO: GET VALUES FROM THE CONFIG VALUE
        let cred = s3::config::Credentials::new(self.key_id.clone(), self.key_secret.clone(), None, None, "Loaded-from-custom-env");
        let conf_builder = s3::config::Builder::new().region(region).credentials_provider(cred);
        let conf = conf_builder.build();
        self.s3_client = Some(s3::Client::from_conf(conf));
        Ok(())
    }

    async fn upload(&self, file_path: &Path, new_object_name: &str) -> Result<(), AppError> {
        if (!file_path.exists()) {
            return Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UploadedFileNotFound)));
        }
        let key: &str;
        match file_path.to_str() {
            Some(stringified_path) => {
                key = new_object_name;
            },
            None => {
                return Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::PathToStringConversionError)));
            }
        }
        let body: ByteStream = ByteStream::from_path(file_path).await?;
        let content_type = mime_guess::from_path(file_path).first_or_octet_stream().to_string();
        let req = self.s3_client.as_ref().expect("S3 client not initialized").put_object().bucket(self.bucket_name.clone()).key(key).
                                            body(body).content_type(content_type);
        req.send().await?;
        return Ok(());
    }

    async fn download(&self, object_name: &str, storage_path: &Path) -> Result<(), AppError> {
        let client_ref = self.s3_client.as_ref().expect("Client was not set");
        let req = client_ref.get_object().bucket(self.bucket_name.clone()).key(object_name);
        let res = req.send().await?;
        let mut data: ByteStream = res.body;
        let file_path_str;
        if let Some(path_str) = storage_path.to_str() {
            file_path_str = path_str;
        } else {
            return Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::PathToStringConversionError)));
        }
        let file = File::create(file_path_str)?;
        let mut buf_writer = BufWriter::new(file);
        while let Some(bytes) = data.try_next().await? {
            buf_writer.write(&bytes).expect("Error in chunch writing");
        }
        buf_writer.flush().expect("Error in file flushing");
        Ok(())
    }

    async fn delete(&self) -> Result<(), AppError> {
        Ok(())
    }
}

impl AwsS3Handler {

    // s3_conection_data: "region:bucket_name:key_id:key_secret", variables cannot contain the ":" character
    pub fn new(s3_conection_data: &str) -> AwsS3Handler {
        let connection_parameters: Vec<&str> = s3_conection_data.split(":").collect(); // TODO: make the separation character a config attribute
        return AwsS3Handler {
            s3_client: None,
            region: connection_parameters[0].to_string(),
            bucket_name: connection_parameters[1].to_string(),
            key_id: connection_parameters[2].to_string(),
            key_secret: connection_parameters[3].to_string(),
        };
    }

}