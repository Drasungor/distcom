use actix_web::{web, App, HttpServer};
// use diesel::migration::MigrationSource;
use diesel::mysql::MysqlConnection;
// use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::R2D2Connection;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use diesel_async::{RunQueryDsl, AsyncConnection, AsyncMysqlConnection};
// use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};
// use diesel_async_migrations::{EmbeddedMigrations, embed_migrations};
use mysql_diesel_async_migration::EmbeddedMigrations;
use embed_migrations_macro_function::mysql_embed_migrations;

// Copied implementation from
// https://github.com/diesel-rs/diesel/blob/master/guide_drafts/migration_guide.md
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// pub const MIGRATIONS: diesel_async_migrations::EmbeddedMigrations = diesel_async_migrations::embed_migrations!();
pub const MIGRATIONS: EmbeddedMigrations = mysql_embed_migrations!();


mod handlers;
mod middlewares;
mod common;
mod services;
mod components;
mod schema;
mod utils;

#[derive(Clone)]
struct AppState {
    // db_connection_pool: Pool<ConnectionManager<MysqlConnection>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", common::config::CONFIG_OBJECT.x);
    let database_url = common::config::CONFIG_OBJECT.database_url.as_str();
    let mut connection = AsyncMysqlConnection::establish(database_url).await.expect("text2");
    MIGRATIONS.run_pending_migrations(&mut connection).await.expect("The migration failed");
    // let state = web::Data::new(AppState { db_connection_pool: connection_pool });
    let state = web::Data::new(AppState { });

    println!("ekisdddddddddddddddddddddddddddddddddddd");

    // let query_result = connection_pool_copy.get().expect("exploto el get de conexion").ping();
    // query_result.expect("Error in database ping");

    println!("pase el ping");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
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
