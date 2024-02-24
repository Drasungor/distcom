// use diesel_async::AsyncMysqlConnection;
// use diesel_async::RunQueryDsl;

use diesel::connection;
// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;

use super::db_models::account::CompleteAccount;
use crate::schema::account;
// use crate::schema::account::dsl::*;

pub struct AccountMysqlDal;

impl AccountMysqlDal {

    pub async fn register_account(new_account_data: CompleteAccount) {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            diesel::insert_into(account::table)
                    .values(&new_account_data)
                //    .execute(&mut connection)
                    .execute(connection)
                    .expect("Error saving new post");
            return Ok(());

        }).expect("asdasdasd");
        }).await.expect("Error in future await")
    }

    pub async fn get_account_data_by_username(username: String) -> CompleteAccount {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account = web::block(move || account::table
            .filter(account::username.eq(username))
            .first::<CompleteAccount>(&mut connection)
            .expect("Error loading posts")).await.expect("Failed wait for get_account_data");
        return found_account;
    }
}
