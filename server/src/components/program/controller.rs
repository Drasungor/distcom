use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};
use std::{fs, path::Path, thread, time::Duration};
use actix_files;

use crate::{common, utils::file_helpers::{get_file_suffix, get_filename_without_suffix}, RequestExtension};
use crate::{common::app_http_response_builder::AppHttpResponseBuilder, middlewares::callable_upload_file::upload_file};
use crate::services::files_storage::file_storage::FileStorage;

use super::service::ProgramService;

pub struct ProgramController;

impl ProgramController {

    pub async fn upload_program(req: HttpRequest, form: Multipart) -> impl Responder {
        let files_names = upload_file(form).await.expect("Failed file upload");

        // TODO: Change expect calls to an internal server error handling
        let extension_value = req.extensions().get::<RequestExtension>().expect("Extension should be initialized").clone();
        let jwt_payload = extension_value.jwt_payload.clone().expect("The jwt payload does not exist");
        
        // TODO: check that only onefile is uploaded
        let file_id = get_filename_without_suffix(&files_names[0]);

        for file_name in files_names {
            let file_path = format!("./uploads/{}", file_name);
            let new_file_name = format!("{}/{}", jwt_payload.organization_id, file_name);
            {
                let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
                read_guard.upload(Path::new(&file_path), &new_file_name).await.expect("File upload error");
            }
            fs::remove_file(file_path).expect("Error in file deletion");
        }


        // TODO: stop hardcoding and receive this from the multipart form data
        let input_lock_timeout = 3600;

        let program_storage_result = ProgramService::add_organization_program(jwt_payload.organization_id, file_id, input_lock_timeout).await;
        return AppHttpResponseBuilder::get_http_response(program_storage_result);
    }

    // TODO: implement the storage of inputs group in the database

    // pub async fn add_inputs_group(req: HttpRequest) -> impl Responder {
    pub async fn add_inputs_group(req: HttpRequest, path: web::Path<String>, form: Multipart) -> impl Responder {

        println!("Path variable: {}", path.as_str());

        let program_id = path.as_str();

        let files_names = upload_file(form).await.expect("Failed file upload");

        // TODO: Change expect calls to an internal server error handling
        let extension_value = req.extensions().get::<RequestExtension>().expect("Extension should be initialized").clone();
        let jwt_payload = extension_value.jwt_payload.clone().expect("The jwt payload does not exist");
        
        // println!("files_names: {:?}", files_names);

        for file_name in files_names {
            let file_path = format!("./uploads/{}", file_name);
            // let new_file_name = format!("{}/{}", jwt_payload.organization_id, file_name);
            // {
            //     let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            //     read_guard.upload(Path::new(&file_path), &new_file_name).await.expect("File upload error");
            // }
            fs::remove_file(file_path).expect("Error in file deletion");
        }

        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

    pub async fn download_program(req: HttpRequest) -> impl Responder {
        let file = actix_files::NamedFile::open_async("./uploads/test.png").await.expect("Problem with async read file");
        return file.into_response(&req);
    }

}
