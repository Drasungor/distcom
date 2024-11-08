use std::fs::{self, File};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};

use crate::{common::{app_error::{AppError, AppErrorType, InternalServerErrorType}, app_http_response_builder::AppHttpResponseBuilder}, RequestExtension};
use super::jwt_helpers::Claims;

pub fn extract_jwt_data(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let extensions = req.extensions();
    let extension_value: &RequestExtension;
    let extension_value_result = extensions.get::<RequestExtension>();
    if let Some(ok_extension_value) = extension_value_result {
        extension_value = ok_extension_value;
    } else {
        return Err(AppHttpResponseBuilder::get_http_response::<()>(Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::QueryExtensionNotSet)))));
    }
    let jwt_payload_result = extension_value.jwt_payload.clone();
    if let Some(ok_jwt_payload) = jwt_payload_result {
        Ok(ok_jwt_payload)
    } else {
        Err(AppHttpResponseBuilder::get_http_response::<()>(Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::JwtValuesNotSet)))))
    }
}

fn open_named_file(file_path: &str) ->  Result<actix_files::NamedFile, AppError> {
    let file = File::open(file_path)?;
    let named_file = actix_files::NamedFile::from_file(file, file_path)?;
    Ok(named_file)
}

pub fn generate_named_file_response(req: &HttpRequest, file_path: &str, delete_file: bool) -> HttpResponse {
    let named_file_result = open_named_file(file_path);
    if delete_file {
        let _ = fs::remove_file(file_path);
    }
    let named_file;
    let open_named_file_result = named_file_result;
    match open_named_file_result {
        Ok(ok_named_file) => {
            named_file = ok_named_file;
        }
        Err(app_error) => {
            return AppHttpResponseBuilder::get_http_response::<()>(Err(app_error));
        },
    }
    named_file.into_response(req)
}
