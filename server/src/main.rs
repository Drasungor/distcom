use actix_web::{web, App, HttpServer};
mod handlers;
mod middlewares;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!("{:?}", common::config::configObject);
    println!("{:?}", common::config::CONFIG_OBJECT.x);
    // common::config::configObject
    // (&common::config::CONFIG_OBJECT).clone();
    HttpServer::new(|| {
        App::new()
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
