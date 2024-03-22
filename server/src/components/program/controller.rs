use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};

use crate::{common::app_http_response_builder::AppHttpResponseBuilder, middlewares::callable_upload_file::upload_file};

// use super::service::AccountService;
// use super::model::{ReceivedNewAccount, Credentials};

pub struct ProgramController;

impl ProgramController {
    // pub async fn upload_program(body: web::Json<()>) -> impl Responder {
    // pub async fn upload_program(req: HttpRequest, form: web::Form<FormData>) -> impl Responder {
    // pub async fn upload_program(form: web::Form<actix_web::http::header::DispositionType>) -> impl Responder {
            
    // pub async fn upload_program(req: HttpRequest, mut form: Multipart) -> impl Responder {
    pub async fn upload_program(mut form: Multipart) -> impl Responder {
        println!("HELLO HELLO HELLO I am upload_program in the controller");
        upload_file(&mut form).await.expect("Failed file upload");
        // upload_file(form).await.expect("Failed file upload");
        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

    // pub async fn upload_program(req: HttpRequest) -> impl Responder {
    // // pub async fn upload_program(mut req: ServiceRequest) -> impl Responder {
    //     println!("HELLO HELLO HELLO I am upload_program in the controller");

    //     let headers = req.headers().clone();
    //     // let my_payload = req.take_payload() as Payload<HttpRequest::Stream>;
    //     let my_payload = req.take_payload();
    //     let mut multipart = actix_multipart::Multipart::new(&headers, my_payload);

    //     upload_file(&mut multipart).await.expect("Failed file upload");
    //     // upload_file(form).await.expect("Failed file upload");
    //     return AppHttpResponseBuilder::get_http_response(Ok(()));
    // }


}
