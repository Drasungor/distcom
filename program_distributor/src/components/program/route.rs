use actix_web::{web, Scope};

use crate::middlewares::{upload_file::UploadFileMiddleware, validate_jwt::ValidateJwtMiddleware};
use super::controller::ProgramController;

pub fn program_router(path_prefix: &str) -> Scope {
    // web::scope(path_prefix)
    web::scope(path_prefix).
        // get
        route("all", web::get().to(ProgramController::get_general_programs)).
        route("template", web::get().to(ProgramController::retrieve_program_template)).
        route("{program_id}", web::get().to(ProgramController::download_program)).
        route("inputs/{program_id}", web::get().to(ProgramController::retrieve_input_group)).
        route("program-and-inputs/{program_id}", web::get().to(ProgramController::retrieve_program_and_input_group)).
        route("organization/{organization_id}", web::get().to(ProgramController::get_organization_programs)).

        

        // post
        route("upload", web::post().to(ProgramController::upload_program).wrap(ValidateJwtMiddleware)).
        route("inputs/{program_id}", web::post().to(ProgramController::add_inputs_group).wrap(ValidateJwtMiddleware))
}