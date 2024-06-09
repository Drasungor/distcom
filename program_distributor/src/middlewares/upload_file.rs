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

use crate::RequestExtension;

use super::callable_upload_file::upload_files;


pub struct UploadFileMiddleware;

impl<S, B> Transform<S, ServiceRequest> for UploadFileMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UploadFileMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UploadFileMiddlewareMiddleware { service }))
    }
}

// Important docs: https://www.shuttle.rs/blog/2023/12/15/using-actix-rust

pub struct UploadFileMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for UploadFileMiddlewareMiddleware<S>
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
        let my_payload = req.take_payload();
        let multipart = actix_multipart::Multipart::new(&headers, my_payload);
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let upload_file_result = upload_files(multipart).await;
            upload_file_result.expect("File upload failed");
            let res = fut.await?;
            Ok(res)
        })
    }
}


fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_folder(path: &str) -> () {
    if !folder_exists(path) {
        fs::create_dir(path).expect("Error in uploads folder creation")
    }
}
