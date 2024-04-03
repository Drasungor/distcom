use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};
use std::{fs, path::Path, thread, time::Duration};
use actix_files;

use crate::{common, RequestExtension};
use crate::{common::app_http_response_builder::AppHttpResponseBuilder, middlewares::callable_upload_file::upload_file};
use crate::services::files_storage::file_storage::FileStorage;

pub struct ProgramController;

impl ProgramController {

    pub async fn upload_program(req: HttpRequest, mut form: Multipart) -> impl Responder {
        let files_names = upload_file(form).await.expect("Failed file upload");

        // TODO: Change expect calls to an internal server error handling
        let extension_value = req.extensions().get::<RequestExtension>().expect("Extension should be initialized").clone();
        let jwt_payload = extension_value.jwt_payload.clone().expect("The jwt payload does not exist");
        
        // println!("files_names: {:?}", files_names);

        for file_name in files_names {
            let file_path = format!("./uploads/{}", file_name);
            let new_file_name = format!("{}/{}", jwt_payload.organization_id, file_name);
            {
                let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
                read_guard.upload(Path::new(&file_path), &new_file_name).await.expect("File upload error");
            }
            fs::remove_file(file_path).expect("Error in file deletion");
        }

        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

    pub async fn download_program(req: HttpRequest) -> impl Responder {
        let file = actix_files::NamedFile::open_async("./uploads/test.png").await.expect("Problem with async read file");
        return file.into_response(&req);
    }

}
