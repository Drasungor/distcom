use actix_web::error::BlockingError;
use diesel::connection;
use diesel::result::DatabaseErrorKind;
// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;

use super::db_models::program::StoredProgram;
// use super::db_models::refresh_token::RefreshToken;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::schema::{program};
// use crate::schema::account::dsl::*;

pub struct ProgramMysqlDal;

impl ProgramMysqlDal {

    pub async fn add_organization_program(organization_id: String, program_id: String, input_lock_timeout: i64) -> Result<(), AppError> {

        let stored_program = StoredProgram {
            organization_id,
            program_id,
            input_lock_timeout,
        };

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let insertion_result = diesel::insert_into(program::table)
                    .values(&stored_program)
                    .execute(connection);
            return insertion_result;

        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Ok(_)) => Ok(()),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    _ => Err(AppError::new(AppErrorType::InternalServerError))
                }
            },
            Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError)),
        }
    }

}
