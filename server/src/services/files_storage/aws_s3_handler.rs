use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

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

    // async fn upload(&self, file_path: &Path) -> Result<(), AppError> {
    async fn upload(&self, file_path: &Path, new_object_name: &str) -> Result<(), AppError> {
    // Path::new();
        if (!file_path.exists()) {
            println!("The path does not exist");
            return Err(AppError::new(AppErrorType::InternalServerError));
        }
        
        let key: &str;
        match file_path.to_str() {
            Some(stringified_path) => {
                key = new_object_name;
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
        let req = self.s3_client.as_ref().expect("S3 client not initialized").put_object().bucket(self.bucket_name.clone()).key(key).
                                            body(body).content_type(content_type);
        match req.send().await {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error in request send: {:?}", error);
                Err(AppError::new(AppErrorType::InternalServerError))
            }
        }
    }

    async fn download(&self, object_name: &str, storage_path: &Path) -> Result<(), AppError> {
        println!("AAAAAAAAA");
        let client_ref = self.s3_client.as_ref().expect("Client was not set");
        println!("BBBBBBBBB");
        let req = client_ref.get_object().bucket(self.bucket_name.clone()).key(object_name);
        println!("CCCCCCCCC");
        println!("object_name: {}", object_name);
        let res = req.send().await.expect("Error in sent request");
        println!("DDDDDDDDD");
        let mut data: ByteStream = res.body;
        println!("EEEEEEEEE");
        let file_path_str = storage_path.to_str().expect("Error in file download path generation");
        println!("FFFFFFFFF");
        let file = File::create(file_path_str).expect("Error in file creation");
        println!("GGGGGGGGG");
        let mut buf_writer = BufWriter::new(file);
        println!("HHHHHHHHH");
        while let Some(bytes) = data.try_next().await.expect("Error in received data stream chunk") {
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