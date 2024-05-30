use actix_web::body::{BoxBody, MessageBody};
use actix_web::http::{self, StatusCode};
use actix_web::{web, HttpMessage, HttpResponse, HttpResponseBuilder};
use actix_web::dev::{ServiceRequest, Transform, forward_ready};
use actix_web::{dev::Service, dev::ServiceResponse, Error};
use std::future::{ready, Ready};
use std::pin::Pin;

use crate::common::app_error::{AppError, AppErrorType};
use crate::common::app_http_response_builder::{AppHttpResponseBuilder, FailureResponse};
use crate::{common, RequestExtension};
use crate::utils::jwt_helpers::{validate_jwt, Claims};


pub struct ValidateJwtMiddleware;

impl<S> Transform<S, ServiceRequest> for ValidateJwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidateJwtMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateJwtMiddlewareMiddleware { service }))
    }
}

// Important docs: https://www.shuttle.rs/blog/2023/12/15/using-actix-rust

pub struct ValidateJwtMiddlewareMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for ValidateJwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();
        
        // TODO: manage this errors correctly instead of using expect
        let token = headers.get("token").expect("No token was received").to_str().expect("Error in token parsing");
        
        let jwt_payload_result = validate_jwt(common::config::CONFIG_OBJECT.token.basic_token_secret.as_str(), token);
        let jwt_payload;

        if let Err(jwt_error) = jwt_payload_result {
            println!("Error in jwt validation: {}", jwt_error);
            let (request, _pl) = req.into_parts();

            // let response = AppHttpResponseBuilder::get_http_response(Err(AppError::new(AppErrorType::InternalServerError)));

            // let response = HttpResponse::build(StatusCode::NOT_FOUND).
            //     json(FailureResponse { 
            //         status: "error".to_string(), 
            //         error_code: "error".to_string(), 
            //         error_message: "error".to_string(),
            // });

            let response = AppHttpResponseBuilder::generate_app_error_body(AppError::new(AppErrorType::InternalServerError));

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        } else {
            jwt_payload = jwt_payload_result.unwrap()
        }

        {
            let mut extensions = req.extensions_mut();
            let extensions_value = extensions.get::<RequestExtension>().expect("The extension was never initialized");
            let mut cloned_extension = extensions_value.clone();
            cloned_extension.jwt_payload = Some(jwt_payload);
            extensions.insert(cloned_extension);
            // println!("Extensions asdasdasdasdasd: {:?}", req.extensions().get::<RequestExtension>());
        }
        
        let fut = self.service.call(req);

        Box::pin(async move {
            println!("Hi from jwt");
            let res = fut.await?;
            println!("RIGHT BEFORE JWT OK(RES): {:?}", res.status());
            Ok(res)
        })
    }
}
