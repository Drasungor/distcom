use std::fs; // Add import for File
use actix_web::{web, HttpResponse};
use std::fs::File; // Add import for File
use actix_multipart::Multipart;
use futures_util::stream::TryStreamExt;
use std::io::Write; // Add import for Write

fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_folder(path: &str) -> () {
    if !folder_exists(path) {
        fs::create_dir(path).expect("Error in uploads folder creation")
    }
}

// pub async fn upload_file(mut payload: web::Payload) -> Result<HttpResponse, actix_web::error::Error> {
pub async fn upload_file(payload: &mut Multipart) -> Result<HttpResponse, actix_web::error::Error> {
// pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::error::Error> {
    let mut file_paths: Vec<String> = Vec::new();
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);

    let aux_result = payload.try_next().await;
    println!("PAyload next test: {:?}", aux_result);

    if let Err(e) = aux_result {
        println!("Print aux_result match {}", e);
        panic!("Epicooooooooooooooooooooo");
    }

    println!("asjhdkaslhdkasjhlajhsdjk");

    while let Ok(Some(field_result)) = payload.try_next().await {
        println!("BORRAR inside while loop");
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
        println!("Going to while loop top");
    }
    println!("After while loop");
    if file_paths.is_empty() {
        Ok(HttpResponse::Ok().body("No file uploaded"))
    } else {
        Ok(HttpResponse::Ok().body(format!("Files saved at: {:?}", file_paths)))
    }
}
