use actix_web::error::BlockingError;

use crate::common::app_error::{AppError, AppErrorType, InternalServerErrorType};

pub fn general_manage_diesel_task_result<T>(result: Result<Result<T, diesel::result::Error>, BlockingError>) -> Result<T, AppError> {
    match result {
        Err(_blocking_error) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        Ok(Ok(result)) => Ok(result),
        Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, _info))) => {
            match db_err_kind {
                unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
            }
        },
        Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
    }
}

pub fn manage_converted_dal_result<T>(result: Result<Result<T, AppError>, BlockingError>) -> Result<T, AppError> {
    match result {
        Err(_blocking_error) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        Ok(Ok(organization_id)) => Ok(organization_id),
        Ok(Err(err)) => Err(err),
    }
}