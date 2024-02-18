use diesel_async::AsyncMysqlConnection;
use diesel_async::RunQueryDsl;

// use super::{dal::AccountDal, db_models::account::NewAccount};
use super::{dal::AccountDal, db_models::account::{CompleteAccount}};

// Define a struct that will implement the Printable trait
struct AccountMysqlDal<'a> {
    database_connection: &'a mut AsyncMysqlConnection,
}

impl<'a> AccountMysqlDal<'a> {
    fn new(database_connection: &'a mut AsyncMysqlConnection) -> AccountMysqlDal<'a> {
        return AccountMysqlDal{ database_connection };
    }
}


impl<'a> AccountDal for AccountMysqlDal<'a> {
    // async fn register_account(&self, new_account_data: &NewAccount) {
    async fn register_account(&mut self, new_account_data: &CompleteAccount) {
        diesel::insert_into(crate::schema::account::table)
            .values(new_account_data)
            .execute(self.database_connection)
            .await
            .expect("Error saving new post");
    }

}