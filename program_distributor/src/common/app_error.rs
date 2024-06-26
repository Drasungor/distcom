use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    AccountNotFound,
    WrongCredentials,
    UsernameAlreadyExists,
    RefreshTokenNotfound,
    InvalidToken,
    InternalServerError,
}

impl AppErrorType {
    pub fn to_string(&self) -> String {
        match self {
            AppErrorType::AccountNotFound => String::from("ACCOUNT_NOT_FOUND"),
            AppErrorType::WrongCredentials => String::from("WRONG_CREDENTIALS"),
            AppErrorType::UsernameAlreadyExists => String::from("USERNAME_ALREADY_EXISTS"),
            AppErrorType::RefreshTokenNotfound => String::from("REFRESH_TOKEN_NOT_FOUND"),
            AppErrorType::InvalidToken => String::from("INVALID_TOKEN"),
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