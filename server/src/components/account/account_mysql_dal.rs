use actix_web::error::BlockingError;
use diesel::connection;
use diesel::result::DatabaseErrorKind;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;

use super::db_models::account::CompleteAccount;
use super::db_models::refresh_token::RefreshToken;
use super::model::PagedOrganizations;
use super::model::ReturnedOrganization;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::schema::{account, refresh_token};

pub struct AccountMysqlDal;

impl AccountMysqlDal {

    pub async fn register_account(new_account_data: CompleteAccount) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let insertion_result = diesel::insert_into(account::table)
                    .values(&new_account_data)
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

    pub async fn get_account_data_by_username(username: String) -> Result<CompleteAccount, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let found_account = account::table
                .filter(account::username.eq(username))
                .first::<CompleteAccount>(connection);
            return found_account;

        })
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

    pub async fn delete_refresh_token(refresh_token_id: String) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            diesel::delete(refresh_token::table.filter(refresh_token::token_id.eq(refresh_token_id)))
                    .execute(connection).expect("Error in refresh token deletion");
            return Ok(());
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

    // TODO: use this function in a "change password" endpoint, it still has not been used
    pub async fn delete_user_refresh_tokens(user_id: String) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            diesel::delete(refresh_token::table.filter(refresh_token::user_id.eq(user_id)))
                    .execute(connection).expect("Error in refresh token deletion");
            return Ok(());
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



    pub async fn get_organizations(name_filter: Option<String>, limit: i64, page: i64) -> Result<PagedOrganizations, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let mut find_accounts_query = account::table
                .offset((page - 1) * limit).limit(limit).into_boxed().filter(account::account_was_verified.eq(true));

            if let Some(name_string) = name_filter {
                // We only use the % at the end of the "like" filter because otherwise the column index will not be used
                find_accounts_query = find_accounts_query.filter(account::name.like(format!("{}%", name_string)));
            }

            let found_accounts: Vec<CompleteAccount> = find_accounts_query.load::<CompleteAccount>(connection).expect("Error finding taken input groups");


            let count_of_matched_elements: i64 = account::table
                .filter(account::account_was_verified.eq(true))
                .count()
                .get_result(connection)
                .expect("Error finding count of matched elements");
            
            let returned_organizations = found_accounts.iter().map(|organization| ReturnedOrganization {
                organization_id: organization.organization_id.clone(),
                name: organization.name.clone(),
                description: organization.description.clone(),
            }).collect();
 
            return Ok(PagedOrganizations {
                organizations: returned_organizations,
                total_elements_amount: count_of_matched_elements,
            });
        })
        }).await;
        return match found_account_result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Ok(paged_organizations)) => Ok(paged_organizations),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                // TODO: handle correctly
                Err(AppError::new(AppErrorType::InternalServerError))
            },
            Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError)),
        };
    }
    
}
