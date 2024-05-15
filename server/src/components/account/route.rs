use actix_web::{web, Scope};

use crate::components::account::controller::AccountController;

pub fn account_router(path_prefix: &str) -> Scope {
    web::scope(path_prefix).
        route("register", web::post().to(AccountController::register)).
        route("login", web::post().to(AccountController::login)).
        route("organizations", web::get().to(AccountController::get_paged_organizations))
}