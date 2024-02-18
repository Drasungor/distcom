use crate::components::account::dal::AccountDal;

pub struct ServiceDependencies {
    account_dal: Box<dyn AccountDal>,
}

pub struct DatabaseDependencies {

}

pub struct ServerDependencies {
    service_dependencies: ServiceDependencies,
    database_dependencies: DatabaseDependencies,
}

// impl ServerDependencies {

// }
