use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};
use serde_derive::{Serialize, Deserialize};

use super::service::AccountService;
use super::model::{ReceivedNewAccount, Credentials};

pub struct AccountController;

impl AccountController {

    pub async fn register(body: web::Json<ReceivedNewAccount>) -> impl Responder {
        AccountService::register(body.into_inner()).await;
        HttpResponse::Ok()
    }

    pub async fn login(body: web::Json<Credentials>) -> impl Responder {
        let login_result = AccountService::login(body.username.clone(), body.password.clone()).await;
        HttpResponse::Ok().json(login_result)
    }
    
}
