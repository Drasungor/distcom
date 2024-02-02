use actix_web::{web, App, HttpServer};
mod handlers;
mod middlewares;
mod common;
mod services;
// use diesel::pg::PgConnection;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
// use r2d2_diesel::ConnectionManager;


// #[derive(Clone)]
struct AppState {
    // db_connection_pool: Pool<ConnectionManager<PgConnection>>
    db_connection_pool: Pool<ConnectionManager<MysqlConnection>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", common::config::CONFIG_OBJECT.x);
    // println!("{:?}", common::lazy_initialize::CONFIG_OBJECT.x);
    // println!("{:?}", common::config::CONFIG_OBJECT.database_url);
    let database_url = common::config::CONFIG_OBJECT.database_url.as_str();
    // let database_connection: PgConnection = PgConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    // let manager = ConnectionManager::<PgConnection>::new(database_url);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    let connection_pool = Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

    let state = web::Data::new(AppState { db_connection_pool: connection_pool });

    println!("ekisdddddddddddddddddddddddddddddddddddd");

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
