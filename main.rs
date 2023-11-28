use actix_web::{web, App, HttpServer};
// use actix_multipart::Multipart;
// use std::fs; // Add import for File
// use std::fs::File; // Add import for File
// use std::io::Write; // Add import for Write
// use futures_util::stream::TryStreamExt;

mod handlers;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
                    .route("", web::post().to(middleware::upload_file::upload_file))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
