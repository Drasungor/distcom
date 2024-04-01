use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};
use std::{fs, path::Path, thread, time::Duration};
use crate::{common, RequestExtension};

use crate::{common::app_http_response_builder::AppHttpResponseBuilder, middlewares::callable_upload_file::upload_file};
// use crate::services::files_storage::aws_s3_handler::AwsS3Handler;
use crate::services::files_storage::file_storage::FileStorage;

pub struct ProgramController;

impl ProgramController {

    // pub async fn upload_program(mut form: Multipart) -> impl Responder {
    pub async fn upload_program(req: HttpRequest, mut form: Multipart) -> impl Responder {
        let files_names = upload_file(form).await.expect("Failed file upload");

        println!("Extensions asdasdasdasdasd: {:?}", req.extensions().get::<RequestExtension>());
        
        for file_name in files_names {
            // let file_path = format!("./upload/{}", file_name);
            // fs::remove_file(file_path).expect("Error in file deletion");
            let file_path = format!("./uploads/{}", file_name);


            {
                let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
                read_guard.upload(Path::new(&file_path), &file_name).await.expect("File upload error");
            }

            fs::remove_file(file_path).expect("Error in file deletion");
        }

        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

}
