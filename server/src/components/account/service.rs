use crate::common::server_dependencies::ServerDependencies;

use super::{dal::AccountDal, utils::generate_password_hash};

pub struct AccountService<'a> {
    dependencies: &'a ServerDependencies,
    accountDal: Box<dyn AccountDal>,
}

impl<'a> AccountService<'a> {

    fn new(dependencies: &ServerDependencies) -> AccountService {
        AccountService { dependencies }
    }
    
    async fn register(&self, username: String, password: String) {
        let password_hash = generate_password_hash(password);

        


    }
    
}