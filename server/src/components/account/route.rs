use actix_web::{web, Scope};

use crate::components::account::controller::AccountController;

pub fn account_router(path_prefix: &str) -> Scope {
    web::scope(path_prefix).
        route("", web::post().to(AccountController::register)).
        route("", web::post().to(AccountController::login))
}