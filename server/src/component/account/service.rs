use actix_web::{web, Scope};

pub fn account_service(path_prefix: &str) -> Scope {
    web::scope(path_prefix).
        route("", web::get().to(handlers::index::index))
}