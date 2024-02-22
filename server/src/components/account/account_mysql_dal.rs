// use diesel_async::AsyncMysqlConnection;
// use diesel_async::RunQueryDsl;

// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;

use super::db_models::account::CompleteAccount;

pub struct AccountMysqlDal;

impl AccountMysqlDal {

    pub async fn register_account(new_account_data: CompleteAccount) {
        // let mut connection = self.database_connection_pool.get().expect("get connection failure");

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");

        web::block(move || diesel::insert_into(crate::schema::account::table)
        .values(&new_account_data)
        .execute(&mut connection)
        .expect("Error saving new post")).await.expect("Error in future await");
    }
}

