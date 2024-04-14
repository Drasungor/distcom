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
        let program_id = path.as_str().to_string();
        let files_names = upload_file(form).await.expect("Failed file upload");

        // TODO: Change expect calls to an internal server error handling
        let extension_value = req.extensions().get::<RequestExtension>().expect("Extension should be initialized").clone();
        let jwt_payload = extension_value.jwt_payload.clone().expect("The jwt payload does not exist");
        for file_name in files_names {
            let file_path = format!("./uploads/{}", file_name);
            ProgramService::add_program_input_group(&jwt_payload.organization_id, &program_id, &file_path).await;
            fs::remove_file(file_path).expect("Error in file deletion");
        }
        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

    // pub async fn download_program(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
    pub async fn download_program(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        // let (organization_id, program_id) = &path.into_inner();
        let file_name = format!("{}.tar", program_id);
        let download_file_path = format!("./downloads/{}", file_name);
        let organization_id = ProgramService::get_program_uploader_id(&program_id).await;

        if (organization_id.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }

        let object_name = format!("{}/{}", organization_id.unwrap(), file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            read_guard.download(&object_name, Path::new(&download_file_path)).await.expect("File upload error");
        }

        let file = actix_files::NamedFile::open_async(download_file_path).await.expect("Problem with async read file");
        return file.into_response(&req);
    }

    pub async fn retrieve_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let file_path = ProgramService::retrieve_input_group(&program_id).await;
        // return AppHttpResponseBuilder::get_http_response(Ok(()));
        if (file_path.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }
        let file = actix_files::NamedFile::open_async(file_path.unwrap()).await.expect("Problem with async read file");
        return file.into_response(&req);
    }

}
