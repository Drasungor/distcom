use std::fs::File;

use actix_web::{HttpRequest, HttpResponse};
use tar::Builder;

use crate::{common::{app_error::AppError, app_http_response_builder::AppHttpResponseBuilder}, utils::actix_helpers::generate_named_file_response};

// Controller 

fn compress_program_with_input(program_id: &str, input_group_id: &str, downloaded_program_file_path: &str, 
                                   program_file_name: &str, input_file_path: &str) -> Result<String, AppError> {
    let tar_file_path = format!("./aux_files/{}/{}_{}.tar", input_group_id, program_id, input_group_id);
    let tar_file = File::create(tar_file_path.clone())?;
    let mut tar_file_builder = Builder::new(tar_file);
    tar_file_builder.append_path_with_name(downloaded_program_file_path, program_file_name)?;
    tar_file_builder.append_path_with_name(input_file_path, format!("{}.csv", input_group_id))?;
    tar_file_builder.finish()?;
    Ok(tar_file_path)
}



pub fn manage_program_with_input_compression(req: &HttpRequest, program_id: &str, input_group_id: &str, downloaded_program_file_path: &str, 
                                             program_file_name: &str, input_file_path: &str) -> HttpResponse {
    let tar_file_path;
    let tar_file_path_result = compress_program_with_input(program_id, input_group_id, downloaded_program_file_path, program_file_name, input_file_path);
    match tar_file_path_result {
        Ok(ok_tar_file_path) => {
            tar_file_path = ok_tar_file_path;
        }
        Err(app_error) => {
            return AppHttpResponseBuilder::get_http_response::<()>(Err(app_error));
        },
    }
    generate_named_file_response(req, &tar_file_path)
}

