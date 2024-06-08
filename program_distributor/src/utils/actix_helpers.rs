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
        return Ok(ok_jwt_payload);
    } else {
        return Err(AppHttpResponseBuilder::get_http_response::<()>(Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::JwtValuesNotSet)))));
    }
}
