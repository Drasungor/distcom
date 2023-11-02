use actix_web::{HttpResponse, Responder};

pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world! greet")
}

pub async fn greet_two() -> impl Responder {
    HttpResponse::Ok().body("Hello, world! greet two")
}
