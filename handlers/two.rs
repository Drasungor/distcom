use actix_web::{HttpResponse, Responder};

pub async fn greet_two() -> impl Responder {
    HttpResponse::Ok().body("Hello, again, Actix!")
}
