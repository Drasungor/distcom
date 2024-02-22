use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use uuid::Uuid;

use super::account_mysql_dal::AccountMysqlDal;
use super::controller::ReceivedNewAccount;
use super::db_models::account::CompleteAccount;
use super::utils::generate_password_hash;

pub struct AccountService;

impl AccountService {

    pub async fn register(new_account_data: ReceivedNewAccount) {
        let id = Uuid::new_v4();
        let password_hash = generate_password_hash(new_account_data.password);

        let new_account = CompleteAccount {
            organization_id: id.to_string(),
            username: new_account_data.username,
            password_hash,
            name: new_account_data.name,
            description: new_account_data.description,
            account_was_verified: false,
        };

        AccountMysqlDal::register_account(new_account).await;

    }
    
}