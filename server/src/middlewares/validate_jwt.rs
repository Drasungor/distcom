use actix_web::{web, HttpResponse, HttpMessage};
use actix_web::dev::{ServiceRequest, Transform, forward_ready};
use actix_web::{dev::Service, dev::ServiceResponse, Error};
use std::future::{ready, Ready};
use std::pin::Pin;

use crate::common;
use crate::utils::jwt_helpers::validate_jwt;


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
        // Do something before handling the request
        println!("Validate jwt Middleware executed before handling the request");
        let headers = req.headers().clone();
        let my_payload = req.take_payload();
        let fut = self.service.call(req);

        // let asdd = headers.get("token").unwrap().to_str();

        // validate_jwt(common::config::CONFIG_OBJECT.token.basic_token_secret.as_str(), token);

        Box::pin(async move {
            let res = fut.await?;
            // let multipart = actix_multipart::Multipart::new(&headers, my_payload);
            // upload_file(multipart).await?;
            println!("Hi from jwt");
            Ok(res)
        })
    }
}
