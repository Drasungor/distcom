// use crate::components::account::dal::AccountDal;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };


use crate::components::account::{account_mysql_dal::AccountMysqlDal, service::AccountService};

// pub struct ServiceDependencies<'a> {
//     pub account_service: Option<AccountService<'a>>,
// }

// pub struct DatabaseDependencies {
//     // Cannot use traits with async methods
//     // account_dal: Box<dyn AccountDal>,
    
//     account_dal: AccountMysqlDal,
    
// }

// pub struct ServerDependencies<'a> {
//     // pub service_dependencies: ServiceDependencies<'a>,
//     // pub database_dependencies: DatabaseDependencies,

//     // pub account_service: Option<AccountService<'a>>,
// }

// impl<'a> ServerDependencies<'a> {
//     // pub fn new(database_connection_pool: Pool<ConnectionManager<MysqlConnection>>) -> ServerDependencies<'a> {
//     //     let mut server_dependencies = ServerDependencies {
//     //         account_service: None,
//     //     };

//     //     // let account_service: AccountService = AccountService::new(&server_dependencies, database_connection_pool.clone());
//     //     let mut account_service: AccountService = AccountService::new(database_connection_pool.clone());


//     //     server_dependencies.account_service = Some(account_service);

//     //     server_dependencies.account_service.as_mut().unwrap().assign_server_dependencies(&server_dependencies);

//     //     return server_dependencies;
//     // }
// }
