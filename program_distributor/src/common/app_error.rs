use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use aws_sdk_s3::{error::SdkError, primitives::ByteStreamError, types::Error};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum InternalServerErrorType {
    TaskSchedulingError,
    UploadedFileNotFound,
    PathToStringConversionError,
    IOError(std::io::Error),
    ByteStreamGenerationError(ByteStreamError),
    S3Error(String),
    UnknownError(String),
}

impl InternalServerErrorType {
    pub fn to_string(&self) -> String {
        match self {
            InternalServerErrorType::TaskSchedulingError => String::from("Error in task/thread scheduling"),
            InternalServerErrorType::UploadedFileNotFound => String::from("The uploaded file was not found"),
            InternalServerErrorType::PathToStringConversionError => String::from("Error in path to string conversion"),
            InternalServerErrorType::ByteStreamGenerationError(byte_stream_error) => format!("Bytestream error: {:?}", byte_stream_error),
            InternalServerErrorType::IOError(io_error) => format!("IO error: {:?}", io_error.to_string()),
            InternalServerErrorType::S3Error(s3_error) => format!("S3 error: {:?}", s3_error),
            InternalServerErrorType::UnknownError(message) => message.clone(),
        }
    }
}

#[derive(Debug)]
pub enum AppErrorType {
    AccountNotFound,
    WrongCredentials,
    UsernameAlreadyExists,
    RefreshTokenNotfound,
    InvalidToken,
    InternalServerError(InternalServerErrorType),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::IOError(error)))
    }
}

impl<U, T> From<SdkError<U, T>> for AppError {
    fn from(error: SdkError<U, T>) -> Self {
        AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::S3Error(error.to_string())))
    }
}

impl From<ByteStreamError> for AppError {
    fn from(error: ByteStreamError) -> Self {
        AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::ByteStreamGenerationError(error)))
    }
}

impl AppErrorType {
    pub fn to_string(&self) -> String {
        match self {
            AppErrorType::AccountNotFound => String::from("ACCOUNT_NOT_FOUND"),
            AppErrorType::WrongCredentials => String::from("WRONG_CREDENTIALS"),
            AppErrorType::UsernameAlreadyExists => String::from("USERNAME_ALREADY_EXISTS"),
            AppErrorType::RefreshTokenNotfound => String::from("REFRESH_TOKEN_NOT_FOUND"),
            AppErrorType::InvalidToken => String::from("INVALID_TOKEN"),
            AppErrorType::InternalServerError(_) => String::from("INTERNAL_SERVER_ERROR"),
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
            AppErrorType::AccountNotFound => {
                message_text = "Account not found";
                status_code = StatusCode::NOT_FOUND;
            },
            AppErrorType::WrongCredentials => {
                message_text = "Incorrect credentials";
                status_code = StatusCode::FORBIDDEN;
            },
            AppErrorType::UsernameAlreadyExists => {
                message_text = "Username already exists";
                status_code = StatusCode::CONFLICT;
            },
            AppErrorType::RefreshTokenNotfound => {
                message_text = "That user's refresh token does not exist";
                status_code = StatusCode::NOT_FOUND;
            },
            AppErrorType::InvalidToken => {
                message_text = "That user's token is not valid";
                status_code = StatusCode::FORBIDDEN;
            },
            AppErrorType::InternalServerError(_) => {
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

    pub fn unexpected_error_message(&self) -> Option<String> {
        if let AppErrorType::InternalServerError(internal_server_error_type) = &self.error_type {
            return Some(internal_server_error_type.to_string());
        }
        return None;
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