use std::fs; // Add import for File
// use actix_web::{web, App, HttpServer};
// use actix_web::{web, HttpResponse, middleware, HttpMessage};
use actix_web::{web, HttpResponse, HttpMessage};
// use futures_util::future::LocalBoxFuture;
use std::fs::File; // Add import for File
use actix_multipart::Multipart;
use futures_util::stream::TryStreamExt;
use std::io::Write; // Add import for Write
// use actix_web::dev::ServiceRequest;
use actix_web::dev::{ServiceRequest, Transform, forward_ready};
// use actix_web::dev;
// use actix_web::scope;
use actix_web::{dev::Service, dev::ServiceResponse, Error};
use std::future::{ready, Ready};
use std::pin::Pin;


pub struct CustomMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CustomMiddleware
where
    // S: 'static,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    // type Transform = CustomMiddlewareMiddleware<S, B>;
    type Transform = CustomMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // fn new_transform(&self, service: S) -> Result<Self::Transform, Self::InitError> {
    //     Ok(CustomMiddlewareMiddleware { service })
    // }

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CustomMiddlewareMiddleware { service }))
    }
}


// struct CustomMiddlewareMiddleware<S, B> {
pub struct CustomMiddlewareMiddleware<S> {
    service: S,
}

// impl<S, B> Service<ServiceRequest> for CustomMiddlewareMiddleware<S, B>
impl<S, B> Service<ServiceRequest> for CustomMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    // type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'static>>;
    // type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;


    // fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
    //     self.service.poll_ready(cx)
    // }
    forward_ready!(service);

    // fn call(&self, req: ServiceRequest) -> Self::Future {
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Do something before handling the request
        println!("Middleware executed before handling the request");

        // let aaa = req.parts().1.clone();
        // let ccc = aaa.take();
        // let bbb = req.take_payload();
        // let bbb = ccc.into_stream();

        let headers = req.headers().clone();
        let my_payload = req.take_payload();
        // let my_payload_2 = req.take_payload();

        let fut = self.service.call(req);
        

        Box::pin(async move {
            let res = fut.await?;
            let multipart = actix_multipart::Multipart::new(&headers, my_payload);
            upload_file(multipart).await?;
            println!("Hi from response");
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

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::error::Error> {
    let mut file_paths: Vec<String> = Vec::new();
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);
    println!("bbbbbbbbbbbbbbbbbbbbb");
    while let Ok(Some(field_result)) = payload.try_next().await {
        println!("aaaaaaaaaaaaaaaaaaa");
        let mut field = field_result;
        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => "unknown".to_string(),
        };
        
        // Define the file path where you want to save the uploaded file
        let file_path = format!("{}/{}", uploads_folder, filename);
        let file_path_clone = file_path.clone();
        // Create a new file and write the field data to it
        let f = web::block(|| File::create(file_path_clone)).await??;
        while let Some(chunk) = field.try_next().await? {
            let mut file_pointer_clone = f.try_clone()?;
            web::block(move || file_pointer_clone.write_all(&chunk)).await??;
        }
        file_paths.push(file_path);
    }
    if file_paths.is_empty() {
        Ok(HttpResponse::Ok().body("No file uploaded"))
    } else {
        Ok(HttpResponse::Ok().body(format!("Files saved at: {:?}", file_paths)))
    }
}
