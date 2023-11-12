use actix_web::{web, App, HttpServer};
use actix_multipart::Multipart;
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
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn upload_file(mut payload: Multipart) -> Result<String, actix_web::error::Error> {
    while let Ok(Some(field_result)) = payload.try_next().await {
        let field = field_result;

        let content_type = field.content_type();
        // let filename = match field.content_disposition() {
        //     Some(cd) => cd.get_filename().unwrap_or("unknown"),
        //     None => "unknown",
        // };

        let filename = match field.content_disposition() {
            Some(cd) => cd.get_filename().unwrap_or_else(|| "unknown".to_string()),
            None => "unknown".to_string(),
        };

        // Define the file path where you want to save the uploaded file
        let file_path = format!("./uploads/{}", filename);

        // Create a new file and write the field data to it
        let mut f = web::block(|| File::create(&file_path)).await?;
        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }

        // You can return the file path or any other response as needed
        return Ok(format!("File saved at: {}", file_path));
    }
    
    // If no file was received
    Ok("No file uploaded".to_string())
}
