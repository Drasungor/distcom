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
pub struct FailureResponse {
    pub status: String,
    pub error_code: String,
    pub error_message: String,
}



impl AppHttpResponseBuilder {
    // pub fn get_http_response<T: Serialize>(app_result: Result<T, AppError>) -> impl Responder {
    pub fn get_http_response<T: Serialize>(app_result: Result<T, AppError>) -> HttpResponse {
        return match app_result {
            Ok(successful_response) => HttpResponse::Ok().
                json(SuccessfulResponse { 
                    status: "success".to_string(), 
                    data: successful_response,
                }),
            Err(error) => {
                Self::generate_app_error_body(error)
                // HttpResponse::build(error.status_code()).
                //     json(FailureResponse { 
                //         status: "error".to_string(), 
                //         error_code: error.error_type(), 
                //         error_message: error.message().clone(),
                // })
            },
        };
    }

    pub fn generate_app_error_body(app_error: AppError) -> HttpResponse {
        HttpResponse::build(app_error.status_code()).
            json(FailureResponse { 
                status: "error".to_string(), 
                error_code: app_error.error_type(), 
                error_message: app_error.message().clone(),
        })
    }
}