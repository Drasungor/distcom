use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};
use serde_derive::{Serialize, Deserialize};


pub struct AccountController {
    register: fn(body: web::Json<Credentials>) -> HttpResponseBuilder,
}


#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

async fn register(body: web::Json<Credentials>) -> impl Responder {
    HttpResponse::Ok()
}

async fn goodbye_two() -> impl Responder {
    HttpResponse::Ok().body("Goodbye, world! two")
}


pub fn account_controller() -> AccountController {
    AccountController {
        register,
    }
}