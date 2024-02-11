use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};
use serde_derive::{Serialize, Deserialize};

use crate::common::server_dependencies::ServerDependencies;


pub struct AccountController<'a> {
    dependencies: &'a ServerDependencies,
}

impl<'a> AccountController<'a> {

    fn new(dependencies: &ServerDependencies) -> AccountController {
        AccountController { dependencies }
    }

    async fn register(&self, body: web::Json<Credentials>) -> impl Responder {
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


// pub fn account_controller() -> AccountController {
//     AccountController {
//         register,
//     }
// }