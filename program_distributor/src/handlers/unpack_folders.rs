use actix_web::{HttpResponse, Responder};
use std::fs::File;
use std::io::prelude::*;
use tar::Archive;

fn extract_tar_archive(archive_path: &str, dest_folder: &str) -> std::io::Result<()> {
    let archive_file = File::open(archive_path)?;
    let mut archive = Archive::new(archive_file);

    // Ensure the destination folder exists
    std::fs::create_dir_all(dest_folder)?;

    // Extract all files and directories from the archive to the destination folder
    archive.unpack(dest_folder)?;

    Ok(())
}

pub async fn process_file_upload() -> impl Responder {
    HttpResponse::Ok().body("Goodbye, world! two")
}
