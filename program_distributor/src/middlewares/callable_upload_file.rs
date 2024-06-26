use std::{fs, io::Read}; // Add import for File
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use serde::de::{DeserializeOwned};
use std::fs::File; // Add import for File
use actix_multipart::Multipart;
use futures_util::stream::TryStreamExt;
use std::io::Write; // Add import for Write
use uuid::Uuid;
// use serde_derive::Deserialize;
use serde_json;
use bytes;

use crate::utils::file_helpers::get_file_suffix;

fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_folder(path: &str) -> () {
    if !folder_exists(path) {
        fs::create_dir(path).expect("Error in uploads folder creation")
    }
}


pub async fn upload_file(mut payload: Multipart) -> Result<Vec<String>, String> {
    let mut files_names: Vec<String> = Vec::new();
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);

    // let aux_result = payload.try_next().await;
    // println!("PAyload next test: {:?}", aux_result);

    // if let Err(e) = aux_result {
    //     println!("Print aux_result match {}", e);
    // }

    while let Ok(Some(field_result)) = payload.try_next().await {
        let mut field_is_file = true;
        let mut field = field_result;
        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => {
                field_is_file = false;
                "unknown".to_string()
            }
        };

        
        if (field_is_file) {
            let file_suffix = get_file_suffix(&filename);
            let new_filename = format!("{}.{}", Uuid::new_v4(), file_suffix);

            // Define the file path where you want to save the uploaded file
            // let file_path = format!("{}/{}.{}", uploads_folder, new_filename, file_suffix);
            let file_path = format!("{}/{}", uploads_folder, new_filename);
            let file_path_clone = file_path.clone();
            // Create a new file and write the field data to it
            let f = web::block(|| File::create(file_path_clone)).await.
                                expect("Error in file creation task").expect("Error in file creation");
            while let Some(chunk) = field.try_next().await.expect("Error in file chunk get") {
                let mut file_pointer_clone = f.try_clone().expect("Error in file pointer clone");
                web::block(move || file_pointer_clone.write_all(&chunk)).await.
                        expect("Error chunk creation task").expect("Error in chunk creation");
            }
            files_names.push(new_filename);
        }
    }
    return Ok(files_names);
}

pub async fn upload_file_with_body<T>(mut payload: Multipart) -> Result<(Vec<String>, T), String>
where
T: DeserializeOwned,
{
    let mut files_names: Vec<String> = Vec::new();
    let mut received_object: Option<T> = None;
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);

    // let aux_result = payload.try_next().await;
    // println!("PAyload next test: {:?}", aux_result);

    // if let Err(e) = aux_result {
    //     println!("Print aux_result match {}", e);
    // }

    while let Ok(Some(field_result)) = payload.try_next().await {
        let mut field_is_file = true;
        let mut field = field_result;
        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => {
                field_is_file = false;
                "unknown".to_string()
            }
        };


        if (field_is_file) {
            let file_suffix = get_file_suffix(&filename);
            let new_filename = format!("{}.{}", Uuid::new_v4(), file_suffix);

            // Define the file path where you want to save the uploaded file
            // let file_path = format!("{}/{}.{}", uploads_folder, new_filename, file_suffix);
            let file_path = format!("{}/{}", uploads_folder, new_filename);
            let file_path_clone = file_path.clone();
            // Create a new file and write the field data to it
            let f = web::block(|| File::create(file_path_clone)).await.
                                expect("Error in file creation task").expect("Error in file creation");
            while let Some(chunk) = field.try_next().await.expect("Error in file chunk get") {
                let mut file_pointer_clone = f.try_clone().expect("Error in file pointer clone");
                web::block(move || file_pointer_clone.write_all(&chunk)).await.
                        expect("Error chunk creation task").expect("Error in chunk creation");
            }
            files_names.push(new_filename);
        } else {
            // TODO: remove assert and manage the error properly
            assert!(received_object.is_none(), "More than one data attribute was received");

            let mut json_str_bytes = bytes::BytesMut::from("");

            while let Some(chunk) = field.try_next().await.expect("Error in file chunk get") {
                json_str_bytes.extend_from_slice(&chunk);
            }
            let bytes_array = json_str_bytes.to_vec();
            let u8_vector = bytes_array.as_slice();
            let json_str = std::str::from_utf8(u8_vector).expect("Error in conversion to json string");
            received_object = Some(serde_json::from_str::<T>(&json_str).expect("Error in conversion from string to struct"));
        }
    }
    return Ok((files_names, received_object.expect("No body was received")));
}