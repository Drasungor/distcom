use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};
use serde_derive::{Serialize, Deserialize};

use crate::common::app_http_response_builder::AppHttpResponseBuilder;

use super::service::AccountService;
use super::model::{ReceivedNewAccount, Credentials};

pub struct AccountController;

impl AccountController {

    pub async fn register(body: web::Json<ReceivedNewAccount>) -> impl Responder {
        let registration_result = AccountService::register(body.into_inner()).await;
        return AppHttpResponseBuilder::get_http_response(registration_result);
    }

    pub async fn login(body: web::Json<Credentials>) -> impl Responder {
        let login_result = AccountService::login(body.username.clone(), body.password.clone()).await;
        return AppHttpResponseBuilder::get_http_response(login_result);
    }
    
}
