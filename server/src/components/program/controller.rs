use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};
use std::{fs::{self, File}, path::Path, thread, time::Duration};
use actix_files;
use tar::{Builder, Archive};
use fs2::FileExt;

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

        // let file = actix_files::NamedFile::open_async(download_file_path).await.expect("Problem with async read file");
        // return file.into_response(&req);

        let program_file = File::open(download_file_path.clone()).expect("Error opening program file");
        let named_file = actix_files::NamedFile::from_file(program_file, download_file_path).expect("Error in NamedFile creation");
        return named_file.into_response(&req);
    }

    pub async fn retrieve_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let input_result = ProgramService::retrieve_input_group(&program_id).await;
        // return AppHttpResponseBuilder::get_http_response(Ok(()));
        if (input_result.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }
        let input_file_name = input_result.unwrap().1;
        let input_file = File::open(input_file_name.clone()).expect("Error opening program file");

        // println!("Before sleep function");
        // thread::sleep(Duration::from_secs(100));
        // println!("After sleep function");

        let named_file = actix_files::NamedFile::from_file(input_file, input_file_name).expect("Error in NamedFile creation");
        return named_file.into_response(&req);

        // return file.into_response(&req);
        // file.into_response(&req).await;
        // return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

    pub async fn retrieve_program_and_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {

        let program_id = path.as_str().to_string();
        // let (organization_id, program_id) = &path.into_inner();
        let program_file_name = format!("{}.tar", program_id);
        let downloaded_program_file_path = format!("./downloads/{}", program_file_name);
        let organization_id = ProgramService::get_program_uploader_id(&program_id).await;

        if (organization_id.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }

        let object_name = format!("{}/{}", organization_id.unwrap(), program_file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            read_guard.download(&object_name, Path::new(&downloaded_program_file_path)).await.expect("File upload error");
        }
        let (input_group_id, input_file_path) = ProgramService::retrieve_input_group(&program_id).await.expect("Error in input group retrieval");

        let tar_file_path = format!("./downloads/{}_{}.tar", program_id, input_group_id);
        let tar_file = File::create(tar_file_path.clone()).unwrap();
        let mut tar_file_builder = Builder::new(tar_file);

        tar_file_builder.append_path(downloaded_program_file_path).expect("Error in adding program to tar builder");
        tar_file_builder.append_path(input_file_path).expect("Error in adding input to tar builder");
        tar_file_builder.finish().expect("Error in builder finish");

        let tar_file = File::open(tar_file_path.clone()).expect("Error opening program file");
        let named_file = actix_files::NamedFile::from_file(tar_file, tar_file_path).expect("Error in NamedFile creation");
        return named_file.into_response(&req);


    }

    


}
