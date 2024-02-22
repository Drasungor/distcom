use actix_web::{web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ ConnectionManager, Pool };
use diesel::r2d2::R2D2Connection;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", common::config::CONFIG_OBJECT.x);
    let connection_pool = &common::config::CONNECTION_POOL;
    let mut pooled_connection = connection_pool.get().expect("asdasdas");
    pooled_connection.run_pending_migrations(MIGRATIONS).expect("The migration failed");
    println!("ekisdddddddddddddddddddddddddddddddddddd");
    println!("pase el ping");

    HttpServer::new(move || {
        App::new()
            // .app_data(state.clone())
            .service(
                web::scope("/")
                    .route("", web::get().to(handlers::index::index))
            )
            .service(
                web::scope("/greet")
                    .route("", web::get().to(handlers::greet::greet))
                    .service(
                        web::scope("/two")
                            .route("", web::get().to(handlers::greet::greet_two))
                    )
            )
            .service(
                web::scope("/goodbye")
                    .route("", web::get().to(handlers::goodbye::goodbye))
                    .route("/two", web::get().to(handlers::goodbye::goodbye_two))
            ).service(
                web::scope("/upload")
                .wrap(middlewares::upload_file::CustomMiddleware)
                .route("", web::post().to(handlers::greet::greet_two))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
