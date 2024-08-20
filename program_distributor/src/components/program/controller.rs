use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, Responder};
use std::{fs::{self}, path::Path};

use crate::{common::{self, app_error::AppError}, middlewares::callable_upload_file::upload_one_file_with_body, utils::{actix_helpers::{extract_jwt_data, generate_named_file_response}, file_helpers::{get_filename_without_suffix}, general_controller_helpers::{process_paging_inputs, PagingParameters}}};
use crate::common::app_http_response_builder::AppHttpResponseBuilder;
use crate::services::files_storage::file_storage::FileStorage;

use super::{model::{GetPagedPrograms, PagedProgramInputGroups, UploadInputGroup, UploadProgram, UploadProof, UploadedInputGroup, UploadedProgram}, service::ProgramService, utils::manage_program_with_input_compression};

pub struct ProgramController;

impl ProgramController {

    pub async fn upload_program(req: HttpRequest, form: Multipart) -> impl Responder {
        let (file_name, uploaded_program) = upload_one_file_with_body::<UploadProgram>(form).await.expect("Failed file upload");


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

        let file_id = get_filename_without_suffix(&file_name);
        let file_path = format!("./uploads/{}", file_name);
        let program_id = get_filename_without_suffix(&file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            read_guard.upload_program(Path::new(&file_path), &jwt_payload.organization_id, &program_id).await.expect("program upload error");
        }
        
        if let Err(file_deletion_error) = fs::remove_file(file_path) {
            return AppHttpResponseBuilder::get_http_response::<()>(Err(AppError::from(file_deletion_error)));
        }

        let input_lock_timeout = uploaded_program.execution_timeout;
        let program_storage_result = ProgramService::add_organization_program(jwt_payload.organization_id.clone(), file_id.clone(), 
                                                                                                    uploaded_program.name, uploaded_program.description, 
                                                                                                    input_lock_timeout).await;
        let returned_body_result: Result<UploadedProgram, AppError>;
        if let Err(returned_error) = program_storage_result {
            returned_body_result = Err(returned_error);
            {
                let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
                read_guard.delete_program(&jwt_payload.organization_id, &program_id).await.expect("Program deletion error");
            }
        } else {
            let program_data = UploadedProgram {
                program_id: file_id,
            };
            returned_body_result = Ok(program_data);
        }
        AppHttpResponseBuilder::get_http_response(returned_body_result)
    }

    pub async fn upload_proof(form: Multipart) -> impl Responder {
        let (file_name, uploaded_program) = upload_one_file_with_body::<UploadProof>(form).await.expect("Failed file upload");
        let file_path = format!("./uploads/{}", file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            read_guard.upload_proof(Path::new(&file_path), &uploaded_program.organization_id, &uploaded_program.program_id, &uploaded_program.input_group_id).await.expect("Proof upload error");
        }
        
        if let Err(file_deletion_error) = fs::remove_file(file_path) {
            return AppHttpResponseBuilder::get_http_response::<()>(Err(AppError::from(file_deletion_error)));
        }

        let set_as_proven_result = ProgramService::set_input_group_as_proven(&uploaded_program.program_id, &uploaded_program.input_group_id).await;
        AppHttpResponseBuilder::get_http_response(set_as_proven_result)
    }

    pub async fn add_inputs_group(req: HttpRequest, path: web::Path<String>, form: Multipart) -> impl Responder {
        let program_id = path.as_str().to_string();
        // let file_name = upload_one_file(form).await.expect("Failed file upload");
        let (file_name, uploaded_inpu_group) = upload_one_file_with_body::<UploadInputGroup>(form).await.expect("Failed file upload");

        // let (file_name, uploaded_program) = upload_one_file_with_body::<UploadProgram>(form).await.expect("Failed file upload");
        // UploadInputGroup

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

        let file_path = format!("./uploads/{}", file_name);
        let add_program_input_group_result = ProgramService::add_program_input_group(&jwt_payload.organization_id, &program_id, &uploaded_inpu_group.name, &file_path).await;
        if add_program_input_group_result.is_err() {
            return AppHttpResponseBuilder::get_http_response(add_program_input_group_result);
        }
        if let Err(file_deletion_error) = fs::remove_file(file_path) {
            AppHttpResponseBuilder::get_http_response::<()>(Err(AppError::from(file_deletion_error)))
        } else {
            let uploaded_input_group_data = UploadedInputGroup {
                input_group_id: add_program_input_group_result.unwrap(),
            };
            AppHttpResponseBuilder::get_http_response(Ok(uploaded_input_group_data))
        }
    }

    pub async fn download_program(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let file_name = format!("{}.tar", program_id);
        let download_file_path = format!("./downloads/{}", file_name);
        let organization_id = ProgramService::get_program_uploader_id(&program_id).await;

        if organization_id.is_err() {
            return AppHttpResponseBuilder::get_http_response(organization_id);
        }

        let program_id = get_filename_without_suffix(&file_name);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            read_guard.download_program(Path::new(&download_file_path), organization_id.as_ref().unwrap(), &program_id).await.expect("Program download error");
        }

        generate_named_file_response(&req, &download_file_path)
    }

    pub async fn download_proof(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
        let (program_id, input_group_id) = path.into_inner();
        let file_name = format!("proof_{program_id}_{input_group_id}.bin");

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

        let download_file_path = format!("./aux_files/{}", file_name);
        let organization_id = &jwt_payload.organization_id;

        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            read_guard.download_proof(Path::new(&download_file_path), organization_id, &program_id, &input_group_id).await.expect("Proof download error");
        }

        generate_named_file_response(&req, &download_file_path)
    }

    pub async fn confirm_proof_validity(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
        let (program_id, input_group_id) = path.into_inner();

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

        let organization_id = &jwt_payload.organization_id;

        let confirm_proof_validity_response = ProgramService::confirm_proof_validity(organization_id, &program_id, &input_group_id).await;

        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");

            // Even if the aws deletion fails we dont return failure since for the user it will seem like everything worked fine,
            // we can manage the deletion in the error case manually
            let proof_deletion_result = read_guard.delete_proof(organization_id, &program_id, &input_group_id).await;
            if let Err(err) = proof_deletion_result {
                println!("Error in proof deletion: {}", err);
            }
        }

        AppHttpResponseBuilder::get_http_response(confirm_proof_validity_response)
    }


    pub async fn delete_program(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.into_inner();

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

        let organization_id = &jwt_payload.organization_id;

        let confirm_proof_validity_response = ProgramService::delete_program(organization_id, &program_id).await;

        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");

            // Even if the aws deletion fails we dont return failure since for the user it will seem like everything worked fine,
            // we can manage the deletion in the error case manually
            let proof_deletion_result = read_guard.delete_program(organization_id, &program_id).await;
            if let Err(err) = proof_deletion_result {
                println!("Error in proof deletion: {}", err);
            }
        }

        AppHttpResponseBuilder::get_http_response(confirm_proof_validity_response)
    }
    

    pub async fn mark_proof_as_invalid(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
        let (program_id, input_group_id) = path.into_inner();

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

        let organization_id = &jwt_payload.organization_id;

        let confirm_proof_validity_response = ProgramService::delete_input_group_proven_mark(organization_id, &program_id, &input_group_id).await;

        AppHttpResponseBuilder::get_http_response(confirm_proof_validity_response)
    }

    pub async fn get_programs_with_proven_executions(req: HttpRequest, query_params: web::Query<PagingParameters>) -> impl Responder {
        let paging_params = process_paging_inputs(query_params.into_inner());
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
        let organization_id = &jwt_payload.organization_id;
        let found_programs_result = ProgramService::get_programs_with_proven_executions(organization_id, paging_params.limit, paging_params.page).await;
        AppHttpResponseBuilder::get_http_response(found_programs_result)
    }

    pub async fn get_input_groups_with_proven_executions(req: HttpRequest, path: web::Path<String>, query_params: web::Query<PagingParameters>) -> impl Responder {
        let paging_params = process_paging_inputs(query_params.into_inner());
        let program_id = path.as_str().to_string();
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
        let organization_id = &jwt_payload.organization_id;
        let found_input_groups_result = ProgramService::get_input_groups_with_proven_executions(organization_id, &program_id, paging_params.limit, paging_params.page).await;
        AppHttpResponseBuilder::get_http_response(found_input_groups_result)
    }

    pub async fn get_program_input_groups(req: HttpRequest, path: web::Path<String>, query_params: web::Query<PagingParameters>) -> impl Responder {
        let paging_params = process_paging_inputs(query_params.into_inner());
        let program_id = path.as_str().to_string();
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
        let organization_id = &jwt_payload.organization_id;
        let found_input_groups_result = ProgramService::get_input_groups(organization_id, &program_id, paging_params.limit, paging_params.page).await;
        AppHttpResponseBuilder::get_http_response(found_input_groups_result)
    }

    pub async fn delete_input_group(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
        let (program_id, input_group_id) = path.into_inner();
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
        let organization_id = &jwt_payload.organization_id;
        let found_input_groups_result = ProgramService::delete_input_group(organization_id, &program_id, &input_group_id).await;
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");

            // Even if the aws deletion fails we dont return failure since for the user it will seem like everything worked fine,
            // we can manage the deletion in the error case manually
            let proof_deletion_result = read_guard.delete_proof(organization_id, &program_id, &input_group_id).await;
            if let Err(err) = proof_deletion_result {
                println!("Error in proof deletion: {}", err);
            }
        }
        AppHttpResponseBuilder::get_http_response(found_input_groups_result)
    }

    pub async fn retrieve_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let input_result = ProgramService::retrieve_input_group(&program_id).await;
        let input_file_name;
        match input_result {
            Ok(ok_input) => {
                input_file_name = ok_input.1;
            },
            Err(error) => {
                return AppHttpResponseBuilder::get_http_response::<()>(Err(error));
            }
        }
        generate_named_file_response(&req, &input_file_name)
    }

    pub async fn retrieve_program_template(req: HttpRequest) -> impl Responder {
        let input_file_name = "./proven_code_template/compressed_template.tar";
        generate_named_file_response(&req, input_file_name)
    }

    pub async fn retrieve_program_and_input_group(req: HttpRequest, path: web::Path<String>) -> impl Responder {
        let program_id = path.as_str().to_string();
        let program_file_name = format!("{}.tar", program_id);
        let organization_id = ProgramService::get_program_uploader_id(&program_id).await;
        if organization_id.is_err() {
            return AppHttpResponseBuilder::get_http_response(organization_id);
        }
        let input_group_id: String;
        let input_file_path: String;
        let retrieve_input_group_result = ProgramService::retrieve_input_group(&program_id).await;
        match retrieve_input_group_result {
            Ok(retrieve_input_group_value) => { 
                input_group_id = retrieve_input_group_value.0;
                input_file_path = retrieve_input_group_value.1;
            },
            Err(error) => {
                return AppHttpResponseBuilder::get_http_response::<()>(Err(error));
            }
        }
        let aux_files_folder = format!("./aux_files/{}", input_group_id);
        let downloaded_program_file_path = format!("{}/{}", aux_files_folder, program_file_name);
        let object_name = format!("{}/{}/program.tar", organization_id.unwrap(), program_id);
        {
            let read_guard = common::config::FILES_STORAGE.read().expect("Error in rw lock");
            let download_result = read_guard.download(&object_name, Path::new(&downloaded_program_file_path)).await;

            if download_result.is_err() {
                let input_group_reservation_deletion_result = ProgramService::delete_input_group_reservation(&input_group_id).await;
                if input_group_reservation_deletion_result.is_err()  {
                    return AppHttpResponseBuilder::get_http_response(input_group_reservation_deletion_result);
                }
            }
        }
        let response = manage_program_with_input_compression(&req, &program_id, &input_group_id, &downloaded_program_file_path, 
                                                     &program_file_name, &input_file_path);
        fs::remove_dir_all(aux_files_folder).expect("Error in folder deletion");
        response
    }

    pub async fn get_organization_programs(path: web::Path<String>, query_params: web::Query<PagingParameters>) -> impl Responder {
        let paging_params = process_paging_inputs(query_params.into_inner());
        let organization_id = path.as_str().to_string();
        let organization_programs = ProgramService::get_organization_programs(organization_id, paging_params.limit, paging_params.page).await;
        AppHttpResponseBuilder::get_http_response(organization_programs)
    }

    pub async fn get_my_programs(req: HttpRequest, query_params: web::Query<PagingParameters>) -> impl Responder {
        let paging_params = process_paging_inputs(query_params.into_inner());
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
        let organization_id = &jwt_payload.organization_id;
        let organization_programs = ProgramService::get_organization_programs(organization_id.clone(), paging_params.limit, paging_params.page).await;
        AppHttpResponseBuilder::get_http_response(organization_programs)
    }

    pub async fn get_general_programs(query_params: web::Query<GetPagedPrograms>) -> impl Responder {
        let query_params = query_params.into_inner();
        let paging = PagingParameters {
            limit: query_params.limit,
            page: query_params.page,
        };
        let paging_params = process_paging_inputs(paging);
        let organization_programs = ProgramService::get_general_programs(query_params.name_filter, paging_params.limit, paging_params.page).await;
        AppHttpResponseBuilder::get_http_response(organization_programs)
    }


}
