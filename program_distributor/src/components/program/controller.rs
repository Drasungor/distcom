use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};
use std::{fs::{self, File}, path::Path, thread, time::Duration};
use actix_files;
use tar::{Builder, Archive};
use fs2::FileExt;

use crate::{common::{self, app_error::{AppError, AppErrorType, InternalServerErrorType}}, middlewares::callable_upload_file::upload_file_with_body, utils::{actix_helpers::extract_jwt_data, file_helpers::{get_file_suffix, get_filename_without_suffix}, general_controller_helpers::{process_paging_inputs, PagingParameters}}, RequestExtension};
use crate::{common::app_http_response_builder::AppHttpResponseBuilder, middlewares::callable_upload_file::upload_file};
use crate::services::files_storage::file_storage::FileStorage;

use super::{model::{GetPagedPrograms, PagedPrograms, UploadProgram}, service::ProgramService};

pub struct ProgramController;

impl ProgramController {

    pub async fn upload_program(req: HttpRequest, form: Multipart) -> impl Responder {

        // TODO: use version that receives only one file
        let (files_names, uploaded_program) = upload_file_with_body::<UploadProgram>(form).await.expect("Failed file upload");

        let jwt_payload;
        let extract_jwt_data_result = extract_jwt_data(&req);
        match extract_jwt_data_result {
            Ok(ok_jwt_payload) => {
                jwt_payload = ok_jwt_payload;
            },
            Err(error_response) => {
                return error_response;
            }
        }

        // TODO: check that only onefile is uploaded
        let file_id = get_filename_without_suffix(&files_names[0]);
        for file_name in files_names {
            let file_path = format!("./uploads/{}", file_name);
            let program_id = get_filename_without_suffix(&file_name);
            {
                let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
                read_guard.upload_program(Path::new(&file_path), &jwt_payload.organization_id, &program_id).await.expect("File upload error");
            }
            fs::remove_file(file_path).expect("Error in file deletion");
        }
        let input_lock_timeout = uploaded_program.execution_timeout;
        let program_storage_result = ProgramService::add_organization_program(jwt_payload.organization_id, file_id, 
                                                                                                    uploaded_program.name, uploaded_program.description, 
                                                                                                    input_lock_timeout).await;
        return AppHttpResponseBuilder::get_http_response(program_storage_result);
    }

    pub async fn add_inputs_group(req: HttpRequest, path: web::Path<String>, form: Multipart) -> impl Responder {
        let program_id = path.as_str().to_string();
        let files_names = upload_file(form).await.expect("Failed file upload");

        let jwt_payload;
        let extract_jwt_data_result = extract_jwt_data(&req);
        match extract_jwt_data_result {
            Ok(ok_jwt_payload) => {
                jwt_payload = ok_jwt_payload;
            },
            Err(error_response) => {
                return error_response;
            }
        }
        
        for file_name in files_names {
            let file_path = format!("./uploads/{}", file_name);
            ProgramService::add_program_input_group(&jwt_payload.organization_id, &program_id, &file_path).await;
            fs::remove_file(file_path).expect("Error in file deletion");
        }
        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

    pub async fn download_program(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let file_name = format!("{}.tar", program_id);
        let download_file_path = format!("./downloads/{}", file_name);
        let organization_id = ProgramService::get_program_uploader_id(&program_id).await;

        if (organization_id.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }

        let object_name = format!("{}/{}", organization_id.as_ref().unwrap(), file_name);
        let program_id = get_filename_without_suffix(&file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            // read_guard.download(&object_name, Path::new(&download_file_path)).await.expect("File upload error");
            read_guard.download_program(Path::new(&download_file_path), &organization_id.as_ref().unwrap(), &program_id).await.expect("File upload error");
        }
        let program_file = File::open(download_file_path.clone()).expect("Error opening program file");
        let named_file = actix_files::NamedFile::from_file(program_file, download_file_path).expect("Error in NamedFile creation");
        return named_file.into_response(&req);
    }

    pub async fn retrieve_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let input_result = ProgramService::retrieve_input_group(&program_id).await;
        if (input_result.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }
        let input_file_name = input_result.unwrap().1;
        let input_file = File::open(input_file_name.clone()).expect("Error opening program file");
        let named_file = actix_files::NamedFile::from_file(input_file, input_file_name).expect("Error in NamedFile creation");
        return named_file.into_response(&req);
    }

    pub async fn retrieve_program_template(req: HttpRequest) -> impl Responder {
        let input_file_name = "./proven_code_template/compressed_template.tar";
        let input_file = File::open(input_file_name.clone()).expect("Error opening program file");
        let named_file = actix_files::NamedFile::from_file(input_file, input_file_name).expect("Error in NamedFile creation");
        return named_file.into_response(&req);
    }

    pub async fn retrieve_program_and_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {

        let program_id = path.as_str().to_string();
        let program_file_name = format!("{}.tar", program_id);
        // let downloaded_program_file_path = format!("./aux_files/{}", program_file_name);
        let organization_id = ProgramService::get_program_uploader_id(&program_id).await;
        
        if (organization_id.is_err()) {
            // TODO: check how to return an error, the inferred return type fails when whe uncomment the line below this 
            // return AppHttpResponseBuilder::get_http_response(file_path);
        }

        let (input_group_id, input_file_path) = ProgramService::retrieve_input_group(&program_id).await.expect("Error in input group retrieval");
        
        let downloaded_program_file_path = format!("./aux_files/{}/{}", input_group_id, program_file_name);

        let object_name = format!("{}/{}", organization_id.unwrap(), program_file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            let download_result = read_guard.download(&object_name, Path::new(&downloaded_program_file_path)).await;

            if (download_result.is_err()) {
                ProgramService::delete_input_group_reservation(&input_group_id).await.expect("Error in input group reservation cancellation");
                panic!("Error in program download");
            }

        }

        let tar_file_path = format!("./aux_files/{}/{}_{}.tar", input_group_id, program_id, input_group_id);
        let tar_file = File::create(tar_file_path.clone()).unwrap();
        let mut tar_file_builder = Builder::new(tar_file);
        tar_file_builder.append_path_with_name(downloaded_program_file_path, program_file_name).expect("Error in adding program to tar builder");
        tar_file_builder.append_path_with_name(input_file_path, format!("{}.csv", input_group_id)).expect("Error in adding input to tar builder");
        tar_file_builder.finish().expect("Error in builder finish");
        let tar_file = File::open(tar_file_path.clone()).expect("Error opening program file");
        let named_file = actix_files::NamedFile::from_file(tar_file, tar_file_path).expect("Error in NamedFile creation");
        return named_file.into_response(&req);
    }

    pub async fn get_organization_programs(path: web::Path<String>, query_params: web::Query<PagingParameters>) -> impl Responder {
        let paging_params = process_paging_inputs(query_params.into_inner());
        let organization_id = path.as_str().to_string();
        let organization_programs = ProgramService::get_organization_programs(organization_id, paging_params.limit, paging_params.page).await;
        return AppHttpResponseBuilder::get_http_response(organization_programs)
    }

    pub async fn get_general_programs(query_params: web::Query<GetPagedPrograms>) -> impl Responder {
        let query_params = query_params.into_inner();
        let paging = PagingParameters {
            limit: query_params.limit,
            page: query_params.page,
        };
        let paging_params = process_paging_inputs(paging);
        let organization_programs = ProgramService::get_general_programs(query_params.name_filter, paging_params.limit, paging_params.page).await;
        return AppHttpResponseBuilder::get_http_response(organization_programs)
    }


}
