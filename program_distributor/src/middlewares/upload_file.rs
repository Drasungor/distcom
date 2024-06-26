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

use super::callable_upload_file::upload_file;


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

        // let init_data = RequestExtension {
        //     jwt_payload: None,
        //     // files_names: None,
        // };

        // let mut extension = req.extensions_mut();

        // req.extensions_mut().insert(init_data);

        let fut = self.service.call(req);
        
        Box::pin(async move {

            // let extensions_clone = extension;

            let upload_file_result = upload_file(multipart).await;
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

// async fn upload_file(mut payload: Multipart) -> Result<Vec<String>, String> {
//     let mut file_paths: Vec<String> = Vec::new();
//     let uploads_folder = "./uploads";
//     create_folder(uploads_folder);

//     // let aux_result = payload.try_next().await;
//     // println!("PAyload next test: {:?}", aux_result);

//     // if let Err(e) = aux_result {
//     //     println!("Print aux_result match {}", e);
//     // }

//     while let Ok(Some(field_result)) = payload.try_next().await {
//         let mut field_is_file = true;
//         let mut field = field_result;
//         let filename = match field.content_disposition().get_filename() {
//             Some(cd) => cd.to_string(),
//             None => {
//                 field_is_file = false;
//                 "unknown".to_string()
//             }
//         };

//         if (field_is_file) {
//             // Define the file path where you want to save the uploaded file
//             let file_path = format!("{}/{}", uploads_folder, filename);
//             let file_path_clone = file_path.clone();
//             // Create a new file and write the field data to it
//             let f = web::block(|| File::create(file_path_clone)).await.
//                                 expect("Error in file creation task").expect("Error in file creation");
//             while let Some(chunk) = field.try_next().await.expect("Error in file chunk get") {
//                 let mut file_pointer_clone = f.try_clone().expect("Error in file pointer clone");
//                 web::block(move || file_pointer_clone.write_all(&chunk)).await.
//                         expect("Error chunk creation task").expect("Error in chunk creation");
//             }
//             file_paths.push(file_path);
//         }
//     }
//     // if file_paths.is_empty() {
//     //     Ok(HttpResponse::Ok().body("No file uploaded"))
//     // } else {
//     //     Ok(HttpResponse::Ok().body(format!("Files saved at: {:?}", file_paths)))
//     // }
//     return Ok(file_paths);
// }
