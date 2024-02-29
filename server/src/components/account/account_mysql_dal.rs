// use diesel_async::AsyncMysqlConnection;
// use diesel_async::RunQueryDsl;

use actix_web::error::BlockingError;
use diesel::connection;
use diesel::result::DatabaseErrorKind;
// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;

use super::db_models::account::CompleteAccount;
use super::db_models::refresh_token::RefreshToken;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::schema::{account, refresh_token};
// use crate::schema::account::dsl::*;

pub struct AccountMysqlDal;

impl AccountMysqlDal {

    pub async fn register_account(new_account_data: CompleteAccount) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let insertion_result = diesel::insert_into(account::table)
            // return diesel::insert_into(account::table)
                    .values(&new_account_data)
                    .execute(connection);
            // println!("{:?}", insertion_result);
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

    pub async fn get_account_data_by_username(username: String) -> Result<CompleteAccount, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let found_account = account::table
                .filter(account::username.eq(username))
                .first::<CompleteAccount>(connection);
            return found_account;

        })
        // }).await.expect("Failed wait for get_account_data");
        }).await;
        return match found_account_result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Err(diesel_error)) => match diesel_error {
                diesel::result::Error::NotFound => Err(AppError::new(AppErrorType::AccountNotFound)),
                _ => Err(AppError::new(AppErrorType::InternalServerError)),
            },
            Ok(Ok(returned_account)) => Ok(returned_account),
        }
    }

    pub async fn add_refresh_token(refresh_token_data: RefreshToken) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let insertion_result = diesel::insert_into(refresh_token::table)
                    .values(&refresh_token_data)
                    .execute(connection);
            return insertion_result;
        })
        }).await;
        return match found_account_result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Ok(_)) => Ok(()),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                // TODO: handle correctly
                Err(AppError::new(AppErrorType::InternalServerError))
            },
            Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError)),
        };
    }

    
}
