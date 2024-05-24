use actix_web::{web, HttpResponse, HttpMessage};
use actix_web::dev::{ServiceRequest, Transform, forward_ready};
use actix_web::{dev::Service, dev::ServiceResponse, Error};
use diesel::insert_into;
use std::future::{ready, Ready};
use std::pin::Pin;

use crate::{common, RequestExtension};
use crate::utils::jwt_helpers::{validate_jwt, Claims};


pub struct ValidateJwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ValidateJwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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

impl<S, B> Service<ServiceRequest> for ValidateJwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();
        
        // TODO: manage this errors correctly instead of using expect
        let token = headers.get("token").expect("No token was received").to_str().expect("Error in token parsing");
        let jwt_payload: Claims = validate_jwt(common::config::CONFIG_OBJECT.token.basic_token_secret.as_str(), token).expect("Error in token decoding");
        
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
