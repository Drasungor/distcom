use actix_web::{web, App, HttpServer, middleware};
// use actix_multipart::Multipart;
// use std::fs; // Add import for File
// use std::fs::File; // Add import for File
// use std::io::Write; // Add import for Write
// use futures_util::stream::TryStreamExt;
use actix_web::dev::ServiceRequest;

mod handlers;
mod middlewares;

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
                // .wrap_fn(|req, srv| {
                //     println!("Hi from start. You requested: {}", req.path());
                //     srv.call(req).map(|res| {
                //         println!("Hi from response");
                //         res
                //     })
                // })
                // wrap(middleware::Logger::default())
                // wrap_fn(middleware::upload_file::upload_file)
                    .route("", web::post().to(handlers::greet::greet_two))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
