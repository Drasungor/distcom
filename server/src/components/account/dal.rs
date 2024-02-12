use super::db_models::account::NewAccount;

pub trait AccountDal {
    async fn register_account(new_account_data: NewAccount);

}