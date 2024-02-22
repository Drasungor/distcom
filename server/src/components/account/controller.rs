use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};
use serde_derive::{Serialize, Deserialize};

use super::service::AccountService;

// use crate::common::server_dependencies::ServerDependencies;


// pub struct AccountController<'a> {
//     dependencies: &'a ServerDependencies<'a>,
// }

pub struct AccountController;

// impl<'a> AccountController<'a> {
impl AccountController {

    // fn new(dependencies: &'a ServerDependencies) -> AccountController<'a> {
    //     AccountController { dependencies }
    // }

    // async fn register(&self, body: web::Json<ReceivedNewAccount>) -> impl Responder {
    async fn register(body: web::Json<ReceivedNewAccount>) -> impl Responder {
        // self.dependencies.service_dependencies.account_service.unwrap("asdasd")
        // self.dependencies.account_service.unwrap().register(body.into_inner()).await;
        // self.dependencies.account_service.as_ref().unwrap().register(body.into_inner()).await;
        AccountService::register(body.into_inner()).await;
        HttpResponse::Ok()
    }
    
    async fn goodbye_two(&self) -> impl Responder {
        HttpResponse::Ok().body("Goodbye, world! two")
    }
    
}


#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct ReceivedNewAccount {
    pub username: String,
    pub password: String,
    pub name: String,
    pub description: String,
    pub account_was_verified: bool,
}


// pub fn account_controller() -> AccountController {
//     AccountController {
//         register,
//     }
// }