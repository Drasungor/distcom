use actix_web::{HttpResponse, Responder};

pub async fn goodbye() -> impl Responder {
    HttpResponse::Ok().body("Goodbye, world!")
}

pub async fn goodbye_two() -> impl Responder {
    HttpResponse::Ok().body("Goodbye, world! two")
}
