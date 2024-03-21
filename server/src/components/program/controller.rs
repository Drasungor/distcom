use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};

use crate::common::app_http_response_builder::AppHttpResponseBuilder;

// use super::service::AccountService;
// use super::model::{ReceivedNewAccount, Credentials};

pub struct ProgramController;

impl ProgramController {

    // pub async fn register(body: web::Json<ReceivedNewAccount>) -> impl Responder {
    //     let registration_result = AccountService::register(body.into_inner()).await;
    //     return AppHttpResponseBuilder::get_http_response(registration_result);
    // }

    // pub async fn login(body: web::Json<Credentials>) -> impl Responder {
    //     let login_result = AccountService::login(body.username.clone(), body.password.clone()).await;
    //     return AppHttpResponseBuilder::get_http_response(login_result);
    // }

    // pub async fn upload_program(body: web::Json<()>) -> impl Responder {
    // pub async fn upload_program(req: HttpRequest, form: web::Form<FormData>) -> impl Responder {
        // pub async fn upload_program(form: web::Form<actix_web::http::header::DispositionType>) -> impl Responder {
    pub async fn upload_program(form: Multipart) -> impl Responder {
        println!("HELLO HELLO HELLO I am upload_program in the controller");
        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

}
