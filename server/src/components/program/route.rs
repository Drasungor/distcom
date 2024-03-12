use actix_web::{web, Scope};

use crate::{components::program::controller::ProgramController, middlewares::{test::TestMiddleware, upload_file::UploadFileMiddleware}};

pub fn program_router(path_prefix: &str) -> Scope {
    // web::scope(path_prefix)
    web::scope(path_prefix).
        route("upload", 
              web::post().to(ProgramController::upload_program).
                    wrap(UploadFileMiddleware).
                    wrap(TestMiddleware))

    
}