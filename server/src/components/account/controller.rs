use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};
use serde_derive::{Serialize, Deserialize};

use super::service::AccountService;

pub struct AccountController;

impl AccountController {

    pub async fn register(body: web::Json<ReceivedNewAccount>) -> impl Responder {
        AccountService::register(body.into_inner()).await;
        HttpResponse::Ok()
    }

    pub async fn login(body: web::Json<Credentials>) -> impl Responder {
        AccountService::login(body.username.clone(), body.password.clone()).await;
        HttpResponse::Ok()
    }
    
    async fn goodbye_two(&self) -> impl Responder {
        HttpResponse::Ok().body("Goodbye, world! two")
    }
    
}


#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct ReceivedNewAccount {
    pub username: String,
    pub password: String,
    pub name: String,
    pub description: String,
}
