// use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;
use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};


use super::app_error::AppError;

#[derive(Debug)]
pub struct AppHttpResponseBuilder;


#[derive(Serialize)]
struct SuccessfulResponse<T: Serialize> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize)]
struct FailureResponse {
    status: String,
    error_code: String,
    error_message: String,
}


impl AppHttpResponseBuilder {
    pub fn get_http_response<T: Serialize>(app_result: Result<T, AppError>) -> impl Responder {
        return match app_result {
            Ok(successful_response) => HttpResponse::Ok().
                json(SuccessfulResponse { 
                    status: "success".to_string(), 
                    data: successful_response 
                }),
            Err(error_type) => HttpResponse::Ok().
                json(FailureResponse { 
                    status: "error".to_string(), 
                    error_code: error_type.error_code(), 
                    error_message: error_type.message(), 
                }),
        };
    }
}