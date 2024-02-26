use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}

impl AppErrorType {
    pub fn to_string(&self) -> String {
        match self {
            AppErrorType::DbError => String::from("DB_ERROR"),
            AppErrorType::NotFoundError => String::from("NOT_FOUND_ERROR"),
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    message_text: Option<String>,
    error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        // TODO: refactor this implementation
        match &*self {
            AppError {
                message_text: Some(message),
                ..
            } => message.clone(),
            AppError {
                message_text: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }

    pub fn error_code(&self) -> String {
        return self.error_type.to_string();
    }
}

// impl From<PoolError> for AppError {
//     fn from(error: PoolError) -> AppError {
//         AppError {
//             message: None, 
//             cause: Some(error.to_string()),
//             error_type: AppErrorType::DbError
//         }
//     }
// }

// impl From<Error> for AppError {
//     fn from(error: Error) -> AppError {
//         AppError {
//             message: None, 
//             cause: Some(error.to_string()),
//             error_type: AppErrorType::DbError
//         }
//     }
// }

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}