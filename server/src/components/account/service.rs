use crate::common::server_dependencies::ServerDependencies;

pub struct AccountService<'a> {
    dependencies: &'a ServerDependencies,
}

impl<'a> AccountService<'a> {

    fn new(dependencies: &ServerDependencies) -> AccountService {
        AccountService { dependencies }
    }
    
    
}