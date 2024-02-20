// use diesel_async::AsyncMysqlConnection;
// use diesel_async::RunQueryDsl;

// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::mysql::MysqlConnection;
use diesel::RunQueryDsl;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;
use super::{dal::AccountDal, db_models::account::{CompleteAccount}};

// Define a struct that will implement the Printable trait
struct AccountMysqlDal {
// struct AccountMysqlDal<'a> {
    // database_connection: &'a mut AsyncMysqlConnection,
    database_connection_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl AccountMysqlDal {
    fn new(database_connection_pool: Pool<ConnectionManager<MysqlConnection>>) -> AccountMysqlDal {
        return AccountMysqlDal{ database_connection_pool };
    }
}


impl AccountDal for AccountMysqlDal {
    async fn register_account(&mut self, new_account_data: CompleteAccount) {
        let mut connection = self.database_connection_pool.get().expect("get connection failure");
        web::block(move || diesel::insert_into(crate::schema::account::table)
        .values(&new_account_data)
        .execute(&mut connection)
        .expect("Error saving new post")).await.expect("Error in future await");
    }

}