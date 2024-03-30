use actix_multipart::Multipart;
use actix_web::{dev::{Payload, ServiceRequest}, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde_derive::{Serialize, Deserialize};

use crate::{common::app_http_response_builder::AppHttpResponseBuilder, middlewares::callable_upload_file::upload_file};

pub struct ProgramController;

impl ProgramController {

    pub async fn upload_program(mut form: Multipart) -> impl Responder {
        println!("HELLO HELLO HELLO I am upload_program in the controller");

        // upload_file(&mut form).await.expect("Failed file upload");
        let files_names = upload_file(form).await.expect("Failed file upload");

        println!("files_names: {:?}", files_names);

        return AppHttpResponseBuilder::get_http_response(Ok(()));
    }

}
