use actix_web::{web, App, HttpServer};
use actix_multipart::Multipart;
use std::fs; // Add import for File
use std::fs::File; // Add import for File
use std::io::Write; // Add import for Write
use futures_util::stream::TryStreamExt;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .route("", web::get().to(handlers::index::index))
            )
            .service(
                web::scope("/greet")
                    .route("", web::get().to(handlers::greet::greet))
                    .service(
                        web::scope("/two")
                            .route("", web::get().to(handlers::greet::greet_two))
                    )
            )
            .service(
                web::scope("/goodbye")
                    .route("", web::get().to(handlers::goodbye::goodbye))
                    .route("/two", web::get().to(handlers::goodbye::goodbye_two))
            ).service(
                web::scope("/upload")
                    .route("", web::post().to(upload_file))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_folder(path: &str) -> () {
    if !folder_exists(path) {
        fs::create_dir(path).expect("Error in uploads folder creation")
    }
}

async fn upload_file(mut payload: Multipart) -> Result<String, actix_web::error::Error> {
    while let Ok(Some(field_result)) = payload.try_next().await {
        // let field = field_result;
        let mut field = field_result;

        // let content_type = field.content_type();

        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => "unknown".to_string(),
        };
        
        let uploads_folder = "./uploads";
        create_folder(uploads_folder);
        // Define the file path where you want to save the uploaded file
        let file_path = format!("{}/{}", uploads_folder, filename);
        let file_path_clone = file_path.clone();
        // Create a new file and write the field data to it
        let f = web::block(|| File::create(file_path_clone)).await??;
        while let Some(chunk) = field.try_next().await? {
            let mut file_pointer_clone = f.try_clone()?;
            web::block(move || file_pointer_clone.write_all(&chunk)).await??;
        }

        // You can return the file path or any other response as needed
        return Ok(format!("File saved at: {}", file_path));
    }
    
    // If no file was received
    Ok("No file uploaded".to_string())
}
