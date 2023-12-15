use actix_web::{HttpResponse, Responder};
// use serde_json::Result;
use serde_derive::Serialize;
// use serde::Serialize;

#[derive(Serialize)]
struct Goodbye {
    message: String,
}

pub async fn goodbye() -> impl Responder {
    let goodbye = Goodbye {
        message: "Goodbye, world!".to_string(),
    };
    HttpResponse::Ok().json(goodbye)
}

pub async fn goodbye_two() -> impl Responder {
    HttpResponse::Ok().body("Goodbye, world! two")
}
