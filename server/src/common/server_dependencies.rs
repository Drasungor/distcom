// use crate::components::account::dal::AccountDal;

use crate::components::account::{account_mysql_dal::AccountMysqlDal, service::AccountService};

pub struct ServiceDependencies {
    accountService: Option<AccountService>,
}

// pub struct DatabaseDependencies {
//     // Cannot use traits with async methods
//     // account_dal: Box<dyn AccountDal>,
    
//     account_dal: AccountMysqlDal,
    
// }

pub struct ServerDependencies {
    pub service_dependencies: ServiceDependencies,
    // pub database_dependencies: DatabaseDependencies,
}

// impl ServerDependencies {

// }
