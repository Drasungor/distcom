use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    InternalServerError,
}

impl AppErrorType {
    pub fn to_string(&self) -> String {
        match self {
            AppErrorType::DbError => String::from("DB_ERROR"),
            AppErrorType::NotFoundError => String::from("NOT_FOUND_ERROR"),
            AppErrorType::InternalServerError => String::from("INTERNAL_SERVER_ERROR"),
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    error_type: AppErrorType,
    message_text: String,
    status_code: StatusCode,
}

impl AppError {

    pub fn new(error_type: AppErrorType) -> AppError {

        let message_text: &str;
        let status_code: StatusCode;

        match error_type {
            AppErrorType::DbError => {
                message_text = "asdasdsa";
                status_code = StatusCode::IM_A_TEAPOT;
            },
            AppErrorType::NotFoundError => {
                message_text = "asdasdsa";
                status_code = StatusCode::IM_A_TEAPOT;
            },
            AppErrorType::InternalServerError => {
                message_text = "Internal server error";
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
            },
        };

        return AppError {
            error_type,
            message_text: message_text.to_string(),
            status_code,
        };

    }

    pub fn message(&self) -> &String {
        return &self.message_text;
    }

    pub fn error_type(&self) -> String {
        return self.error_type.to_string();
    }

    pub fn status_code(&self) -> StatusCode {
        return self.status_code;
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}