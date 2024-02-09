use actix_web::{web, Scope};

pub fn account_router(path_prefix: &str) -> Scope {
    web::scope(path_prefix).
        route("", web::get().to(crate::handlers::index::index))
}