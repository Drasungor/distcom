use actix_web::{HttpResponse, Responder};

pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
