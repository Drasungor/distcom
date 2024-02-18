// use super::db_models::account::NewAccount;

use super::db_models::account::CompleteAccount;

pub trait AccountDal {
    // async fn register_account(&self, new_account_data: &NewAccount);
    async fn register_account(&mut self, new_account_data: &CompleteAccount);

}