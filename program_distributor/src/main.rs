
use actix_web::dev::Service;
use actix_web::{App, HttpMessage, HttpServer};
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };
use futures_util::FutureExt;
use utils::jwt_helpers::Claims;
use utils::local_storage_helpers::clear_aux_directories;

use crate::components::account::route::account_router;
use crate::components::program::route::program_router;
use crate::services::files_storage::file_storage::FileStorage;
use crate::utils::local_storage_helpers::compress_folder_contents;

// Copied implementation from
// https://github.com/diesel-rs/diesel/blob/master/guide_drafts/migration_guide.md
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


mod handlers;
mod middlewares;
mod common;
mod services;
mod components;
mod schema;
mod utils;

#[derive(Clone, Debug)]
pub struct RequestExtension {
    pub jwt_payload: Option<Claims>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    compress_folder_contents("./proven_code_template/template", "./proven_code_template/compressed_template.tar").expect("Compression failed");
    let connection_pool = &common::config::CONNECTION_POOL;
    let mut pooled_connection = connection_pool.get().expect("asdasdas");
    pooled_connection.run_pending_migrations(MIGRATIONS).expect("The migration failed");
    
    {
        // We establish the connection to s3
        let mut write_guard = common::config::FILES_STORAGE.write().expect("Error in rw lock");
        write_guard.set_up_connection().await.expect("Error in file storage connection setup");
    }

    println!("Successfully connected to database");

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let init_data = RequestExtension {
                    jwt_payload: None,
                    // files_names: None,
                };
                req.extensions_mut().insert(init_data);
                
                srv.call(req).map(|res| {
                    res
                })
            })
            .service(
                account_router("/account")
            )
            .service(
                program_router("/program")
            )
    })
    // .bind("127.0.0.1:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
