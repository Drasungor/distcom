use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use uuid::Uuid;

use crate::common::server_dependencies::ServerDependencies;
use super::account_mysql_dal::AccountMysqlDal;
use super::controller::ReceivedNewAccount;
// use super::controller::ReceivedNewAccount;
use super::db_models::account::CompleteAccount;
use super::utils::generate_password_hash;

// use super::{dal::AccountDal, utils::generate_password_hash};

pub struct AccountService<'a> {
    dependencies: &'a ServerDependencies,
    account_dal: AccountMysqlDal,
}

impl<'a> AccountService<'a> {

    fn new(dependencies: &ServerDependencies, database_connection_pool: Pool<ConnectionManager<MysqlConnection>>) -> AccountService {
        AccountService { 
            dependencies,
            account_dal: AccountMysqlDal::new(database_connection_pool),
        }
    }
    
    async fn register(&self, new_account_data: ReceivedNewAccount) {
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
        self.account_dal.register_account(new_account).await
    }
    
}