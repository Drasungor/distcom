use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use uuid::Uuid;

use crate::common::app_error::AppError;

use super::account_mysql_dal::AccountMysqlDal;
use super::db_models::refresh_token::RefreshToken;
use super::model::{LoginTokens, ReceivedNewAccount};
use super::db_models::account::CompleteAccount;
use super::utils::{generate_basic_token, generate_login_tokens, generate_password_hash, is_password_valid};

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

    pub async fn login(username: String, password: String) -> Result<LoginTokens, AppError> {
        let account_data = AccountMysqlDal::get_account_data_by_username(username).await;
        if (!is_password_valid(password, account_data.password_hash)) {
            panic!("Wrong password (refactor this)")
        }
        let login_tokens = generate_login_tokens(&account_data.organization_id);
        let refresh_token_data = RefreshToken {
            token_id: login_tokens.refresh_token.token_id.clone(),
            user_id: account_data.organization_id,
        };
        AccountMysqlDal::add_refresh_token(refresh_token_data).await;

        // TODO: fix the return values, it should not always return ok
        return Ok(login_tokens);
    }

}