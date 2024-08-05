use diesel::RunQueryDsl;
use diesel::prelude::*;
use actix_web::web;

use super::db_models::account::CompleteAccount;
use super::db_models::refresh_token::RefreshToken;
use super::model::PagedOrganizations;
use super::model::ReturnedOrganization;
use crate::common::app_error::AppError;
use crate::schema::{account, refresh_token};
use crate::utils::diesel_helpers::manage_converted_dal_result;

pub struct AccountMysqlDal;

impl AccountMysqlDal {

    pub async fn register_account(new_account_data: CompleteAccount) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        // connection.transaction::<_, diesel::result::Error, _>(|connection| {
        connection.transaction::<_, AppError, _>(|connection| {

            // let insertion_result = diesel::insert_into(account::table)
            //         .values(&new_account_data)
            //         .execute(connection);
            // return insertion_result;
            diesel::insert_into(account::table)
                    .values(&new_account_data)
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Ok(_)) => Ok(()),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         match db_err_kind {
        //             DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
        //             unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
        //         }
        //     },
        //     Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        // }
        return manage_converted_dal_result(result);
    }

    pub async fn get_account_data_by_username(username: String) -> Result<CompleteAccount, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, AppError, _>(|connection| {

            let found_account = account::table
                .filter(account::username.eq(username))
                .first::<CompleteAccount>(connection)?;
            return Ok(found_account);

        })
        }).await;
        // return match found_account_result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Err(diesel_error)) => match diesel_error {
        //         diesel::result::Error::NotFound => Err(AppError::new(AppErrorType::AccountNotFound)),
        //         unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
        //     },
        //     Ok(Ok(returned_account)) => Ok(returned_account),
        // }
        return manage_converted_dal_result(found_account_result);
    }

    pub async fn add_refresh_token(refresh_token_data: RefreshToken) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, AppError, _>(|connection| {

            diesel::insert_into(refresh_token::table)
                    .values(&refresh_token_data)
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Ok(_)) => Ok(()),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", db_err_kind)))))
        //     },
        //     Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        // };
        return manage_converted_dal_result(result);
    }

    pub async fn delete_refresh_token(refresh_token_id: String) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, AppError, _>(|connection| {

            diesel::delete(refresh_token::table.filter(refresh_token::token_id.eq(refresh_token_id)))
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Ok(_)) => Ok(()),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", db_err_kind)))))
        //     },
        //     Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        // };
        return manage_converted_dal_result(result);
    }

    pub async fn user_refresh_token_exists(refresh_token_id: String, user_id: String) -> Result<bool, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, AppError, _>(|connection| {
            let found_account = refresh_token::table
                    .filter(refresh_token::token_id.eq(refresh_token_id).and(refresh_token::user_id.eq(user_id)))
                    .first::<RefreshToken>(connection);
            return Ok(found_account.is_ok());
        })
        }).await;
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Ok(user_refresh_token_exists)) => Ok(user_refresh_token_exists),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", db_err_kind)))))
        //     },
        //     Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        // };
        return manage_converted_dal_result(result);
    }

    // TODO: use this function in a "change password" endpoint, it still has not been used
    pub async fn delete_user_refresh_tokens(user_id: String) -> Result<(), AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, AppError, _>(|connection| {

            diesel::delete(refresh_token::table.filter(refresh_token::user_id.eq(user_id)))
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Ok(_)) => Ok(()),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", db_err_kind)))))
        //     },
        //     Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        // };
        return manage_converted_dal_result(result);
    }

    pub async fn get_organizations(name_filter: Option<String>, limit: i64, page: i64) -> Result<PagedOrganizations, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, AppError, _>(|connection| {

            let mut find_accounts_query = account::table
                .offset((page - 1) * limit).limit(limit).into_boxed().filter(account::account_was_verified.eq(true));

            if let Some(name_string) = name_filter {
                // We only use the % at the end of the "like" filter because otherwise the column index will not be used
                find_accounts_query = find_accounts_query.filter(account::name.like(format!("{}%", name_string)));
            }

            let found_accounts: Vec<CompleteAccount> = find_accounts_query.load::<CompleteAccount>(connection)?;
            let count_of_matched_elements: i64 = account::table
                .filter(account::account_was_verified.eq(true))
                .count()
                .get_result(connection)?;
            
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
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
        //     Ok(Ok(paged_organizations)) => Ok(paged_organizations),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", db_err_kind)))))
        //     },
        //     Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        // };
        return manage_converted_dal_result(result);
    }
    
}
