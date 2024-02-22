// use diesel_async::AsyncMysqlConnection;
// use diesel_async::RunQueryDsl;

// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;

use super::db_models::account::CompleteAccount;
// use super::{dal::AccountDal, db_models::account::{CompleteAccount}};

// Define a struct that will implement the Printable trait
// pub struct AccountMysqlDal {
// // struct AccountMysqlDal<'a> {
//     // database_connection: &'a mut AsyncMysqlConnection,
//     database_connection_pool: Pool<ConnectionManager<MysqlConnection>>,
// }

pub struct AccountMysqlDal;

impl AccountMysqlDal {
    // pub fn new(database_connection_pool: Pool<ConnectionManager<MysqlConnection>>) -> AccountMysqlDal {
    //     return AccountMysqlDal{ database_connection_pool };
    // }
    
    // pub async fn register_account(&self, new_account_data: CompleteAccount) {
    pub async fn register_account(new_account_data: CompleteAccount) {
        // let mut connection = self.database_connection_pool.get().expect("get connection failure");

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");

        web::block(move || diesel::insert_into(crate::schema::account::table)
        .values(&new_account_data)
        .execute(&mut connection)
        .expect("Error saving new post")).await.expect("Error in future await");
    }
}

