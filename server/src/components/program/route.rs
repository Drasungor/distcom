use actix_web::{web, Scope};

use crate::components::program::controller::ProgramController;

pub fn program_router(path_prefix: &str) -> Scope {
    web::scope(path_prefix)
    // web::scope(path_prefix).
    //     route("register", web::post().to(AccountController::register)).
    //     route("login", web::post().to(AccountController::login))
}