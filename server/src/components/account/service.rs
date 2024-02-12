use crate::common::server_dependencies::ServerDependencies;

use super::utils::generate_password_hashes;

pub struct AccountService<'a> {
    dependencies: &'a ServerDependencies,
}

impl<'a> AccountService<'a> {

    fn new(dependencies: &ServerDependencies) -> AccountService {
        AccountService { dependencies }
    }
    
    async fn register(username: String, password: String) {
        let password_hash = generate_password_hashes(password);




    }
    
}