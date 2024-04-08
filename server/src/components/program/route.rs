use actix_web::{web, Scope};

use crate::middlewares::{upload_file::UploadFileMiddleware, validate_jwt::ValidateJwtMiddleware};
use super::controller::ProgramController;

pub fn program_router(path_prefix: &str) -> Scope {
    // web::scope(path_prefix)
    web::scope(path_prefix).
        route("upload", web::post().to(ProgramController::upload_program).wrap(ValidateJwtMiddleware)).
        route("download", web::get().to(ProgramController::download_program)).
        route("inputs/{program_id}", web::post().to(ProgramController::add_inputs_group).wrap(ValidateJwtMiddleware))
}