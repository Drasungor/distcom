use actix_web::{web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::R2D2Connection;

mod handlers;
mod middlewares;
mod common;
mod services;
mod component;

#[derive(Clone)]
struct AppState {
    db_connection_pool: Pool<ConnectionManager<MysqlConnection>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", common::config::CONFIG_OBJECT.x);
    let database_url = common::config::CONFIG_OBJECT.database_url.as_str();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    let connection_pool = Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

    let connection_pool_copy = connection_pool.clone();

    let state = web::Data::new(AppState { db_connection_pool: connection_pool });

    println!("ekisdddddddddddddddddddddddddddddddddddd");

    let query_result = connection_pool_copy.get().expect("exploto el get de conexion").ping();
    query_result.expect("Error in database ping");

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
