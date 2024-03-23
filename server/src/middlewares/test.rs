use std::fs; // Add import for File
use actix_web::{web, HttpResponse, HttpMessage};
use std::fs::File; // Add import for File
use actix_multipart::Multipart;
use futures_util::stream::TryStreamExt;
use std::io::Write; // Add import for Write
use actix_web::dev::{ServiceRequest, Transform, forward_ready};
use actix_web::{dev::Service, dev::ServiceResponse, Error};
use std::future::{ready, Ready};
use std::pin::Pin;


pub struct TestMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TestMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TestMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TestMiddlewareMiddleware { service }))
    }
}

// Important docs: https://www.shuttle.rs/blog/2023/12/15/using-actix-rust

pub struct TestMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TestMiddlewareMiddleware<S>
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
        println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa Test Middleware executed before handling the request");
        let headers = req.headers().clone();
        let my_payload = req.take_payload();
        let fut = self.service.call(req);
        Box::pin(async move {
            println!("Hi I am test middleware");
            let res = fut.await?;
            Ok(res)
        })
    }
}
