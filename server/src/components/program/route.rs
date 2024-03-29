use actix_web::{web, Scope};

use crate::{components::program::controller::ProgramController, middlewares::{test::TestMiddleware, upload_file::UploadFileMiddleware, validate_jwt::ValidateJwtMiddleware}};

pub fn program_router(path_prefix: &str) -> Scope {
    // web::scope(path_prefix)
    web::scope(path_prefix).
        route("upload", 
              web::post().to(ProgramController::upload_program).
              // web::post().to(ProgramController::upload_program))
                    // wrap(ValidateJwtMiddleware).
                    // wrap(TestMiddleware))

                    // wrap(TestMiddleware).
                    // wrap(ValidateJwtMiddleware))

                    // wrap(ValidateJwtMiddleware).
                    // wrap(UploadFileMiddleware))

                    wrap(UploadFileMiddleware).
                    wrap(ValidateJwtMiddleware))

                    // wrap(ValidateJwtMiddleware))
}