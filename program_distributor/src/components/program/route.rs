use actix_web::{web, Scope};

use crate::middlewares::validate_jwt::ValidateJwtMiddleware;
use super::controller::ProgramController;

pub fn program_router(path_prefix: &str) -> Scope {
    web::scope(path_prefix).
        // get
        route("all", web::get().to(ProgramController::get_general_programs)).
        route("mine", web::get().to(ProgramController::get_my_programs).wrap(ValidateJwtMiddleware)).
        route("template", web::get().to(ProgramController::retrieve_program_template)).
        route("inputs/{program_id}", web::get().to(ProgramController::retrieve_input_group)).
        route("inputs/all/{program_id}", web::get().to(ProgramController::get_program_input_groups).wrap(ValidateJwtMiddleware)).
        route("program-and-inputs/{program_id}", web::get().to(ProgramController::retrieve_program_and_input_group)).
        route("organization/{organization_id}", web::get().to(ProgramController::get_organization_programs)).
        route("proof/{program_id}/{input_group_id}", web::get().to(ProgramController::download_proof).wrap(ValidateJwtMiddleware)).
        route("proofs", web::get().to(ProgramController::get_programs_with_proven_executions).wrap(ValidateJwtMiddleware)).
        route("proofs/{program_id}", web::get().to(ProgramController::get_input_groups_with_proven_executions).wrap(ValidateJwtMiddleware)).
        route("{program_id}", web::get().to(ProgramController::download_program)).

        // patch
        route("proof/{program_id}/{input_group_id}", web::patch().to(ProgramController::mark_proof_as_invalid).wrap(ValidateJwtMiddleware)).

        // delete
        route("{program_id}", web::delete().to(ProgramController::delete_program).wrap(ValidateJwtMiddleware)).
        route("proof/{program_id}/{input_group_id}", web::delete().to(ProgramController::confirm_proof_validity).wrap(ValidateJwtMiddleware)).
        route("input/{program_id}/{input_group_id}", web::delete().to(ProgramController::delete_input_group).wrap(ValidateJwtMiddleware)).
        
        // post
        route("upload", web::post().to(ProgramController::upload_program).wrap(ValidateJwtMiddleware)).
        route("proof", web::post().to(ProgramController::upload_proof)). 
        route("inputs/{program_id}", web::post().to(ProgramController::add_inputs_group).wrap(ValidateJwtMiddleware))
}