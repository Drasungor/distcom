use std::fs; // Add import for File
// use actix_web::{web, App, HttpServer};
use actix_web::{web, HttpResponse};
use std::fs::File; // Add import for File
use actix_multipart::Multipart;
use futures_util::stream::TryStreamExt;
use std::io::Write; // Add import for Write
// use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceRequest;
use actix_web::dev;
// use actix_web::scope;
use actix_web::{dev::Service, dev::ServiceResponse, Error};

fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_folder(path: &str) -> () {
    if !folder_exists(path) {
        fs::create_dir(path).expect("Error in uploads folder creation")
    }
}

// pub async fn upload_file(mut payload: Multipart, &srv: web::types::ServiceRequest) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(mut payload: Multipart, srv: &web::types::ServiceRequest) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(mut payload: Multipart, srv: &ServiceRequest) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(req: ServiceRequest, _app: &actix_web::App<AppEntry>) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(req: ServiceRequest, payload: &actix_web::scope::ScopeService) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(req: ServiceRequest, payload: &actix_web::scope::ScopeService) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(req: actix_web::dev::ServiceRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(req: actix_web::dev::ServiceRequest, _: &actix_web::web::Data<()>) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(req: actix_web::dev::ServiceRequest, srv: &dyn actix_web::dev::Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error, Future = impl std::future::Future>) -> Result<actix_web::dev::ServiceResponse, actix_web::Error> {
pub async fn upload_file(req: actix_web::dev::ServiceRequest, srv: &actix_web::scope::ScopeService) -> Result<actix_web::dev::ServiceResponse, actix_web::Error> {


    let mut file_paths: Vec<String> = Vec::new();
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);
    while let Ok(Some(field_result)) = payload.try_next().await {
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
        // Ok("No file uploaded".to_string())
    } else {
        Ok(HttpResponse::Ok().body(format!("Files saved at: {:?}", file_paths)))
        // Ok(format!("Files saved at: {:?}", file_paths))
    }
}
