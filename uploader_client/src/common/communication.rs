use std::str::FromStr;
use std::fmt;
use serde_derive::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EndpointResult<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct EndpointError {
    pub status: String,
    pub error_code: String,
    pub error_message: String,
}

#[derive(Debug, PartialEq)]
pub enum AppErrorType {
    AccountNotFound,
    WrongCredentials,
    UsernameAlreadyExists,
    RefreshTokenNotfound,
    InvalidToken,
    InternalServerError,
}

impl FromStr for AppErrorType {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACCOUNT_NOT_FOUND" => Ok(AppErrorType::AccountNotFound),
            "WRONG_CREDENTIALS" => Ok(AppErrorType::WrongCredentials),
            "USERNAME_ALREADY_EXISTS" => Ok(AppErrorType::UsernameAlreadyExists),
            "REFRESH_TOKEN_NOT_FOUND" => Ok(AppErrorType::RefreshTokenNotfound),
            "INVALID_TOKEN" => Ok(AppErrorType::InvalidToken),
            "INTERNAL_SERVER_ERROR" => Ok(AppErrorType::InternalServerError),
            _ => Err(s.to_string()),
        }
    }
}
