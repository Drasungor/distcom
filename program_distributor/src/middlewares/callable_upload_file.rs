use actix_web::web;
use serde;
use serde::de::DeserializeOwned;
use std::fs::File;
use actix_multipart::Multipart;
use futures_util::stream::TryStreamExt;
use std::io::Write; // Add import for Write
use uuid::Uuid;
use serde_json;
use bytes;

use crate::utils::{file_helpers::get_file_suffix, local_storage_helpers::create_folder};


async fn process_file_field(mut field: actix_multipart::Field, uploads_folder: &str, filename: &String) -> String {
    let file_suffix = get_file_suffix(filename);
    let new_filename = format!("{}.{}", Uuid::new_v4(), file_suffix);

    // Define the file path where you want to save the uploaded file
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
    new_filename
}

async fn process_text_object_field<T>(mut field: actix_multipart::Field) -> T 
where
T: DeserializeOwned,
{
    let mut json_str_bytes = bytes::BytesMut::from("");

    while let Some(chunk) = field.try_next().await.expect("Error in file chunk get") {
        json_str_bytes.extend_from_slice(&chunk);
    }
    let bytes_array = json_str_bytes.to_vec();
    let u8_vector = bytes_array.as_slice();
    let json_str = std::str::from_utf8(u8_vector).expect("Error in conversion to json string");
    serde_json::from_str::<T>(json_str).expect("Error in conversion from string to struct")
}

pub async fn upload_files(mut payload: Multipart) -> Result<Vec<String>, String> {
    let mut files_names: Vec<String> = Vec::new();
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);

    while let Ok(Some(field_result)) = payload.try_next().await {
        let mut field_is_file = true;
        let field = field_result;
        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => {
                field_is_file = false;
                "unknown".to_string()
            }
        };
        
        if field_is_file {
            let new_filename = process_file_field(field, uploads_folder, &filename).await;
            files_names.push(new_filename);
        }
    }
    Ok(files_names)
}

// pub async fn upload_files_with_body<T>(mut payload: Multipart) -> Result<(Vec<String>, T), String>
// where
// T: DeserializeOwned,
// {
//     let mut files_names: Vec<String> = Vec::new();
//     let mut received_object: Option<T> = None;
//     let uploads_folder = "./uploads";
//     create_folder(uploads_folder);

//     while let Ok(Some(field_result)) = payload.try_next().await {
//         let mut field_is_file = true;
//         let field = field_result;
//         let filename = match field.content_disposition().get_filename() {
//             Some(cd) => cd.to_string(),
//             None => {
//                 field_is_file = false;
//                 "unknown".to_string()
//             }
//         };

//         if field_is_file {
//             let new_filename = process_file_field(field, uploads_folder, &filename).await;
//             files_names.push(new_filename);
//         } else {
//             // TODO: remove assert and manage the error properly
//             assert!(received_object.is_none(), "More than one data attribute was received");
//             received_object = Some(process_text_object_field::<T>(field).await);
//         }
//     }
//     Ok((files_names, received_object.expect("No body was received")))
// }

pub async fn upload_exact_amount_files_with_body<T>(mut payload: Multipart, files_amount: u64) -> Result<(Vec<String>, T), String>
where
T: DeserializeOwned,
{
    let mut files_names: Vec<String> = Vec::new();
    let mut received_object: Option<T> = None;
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);

    let mut counter = 0;

    while let Ok(Some(field_result)) = payload.try_next().await {
        let mut field_is_file = true;
        let field = field_result;
        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => {
                field_is_file = false;
                "unknown".to_string()
            }
        };

        println!("filename: {filename}");

        if field_is_file {

            // TODO: manage this with an error instead of an assert
            assert!(counter < files_amount, "This endpoint expects {files_amount} files but more ({counter}) were sent");


            let new_filename = process_file_field(field, uploads_folder, &filename).await;
            files_names.push(new_filename);

            counter += 1;

        } else {
            // TODO: remove assert and manage the error properly
            assert!(received_object.is_none(), "More than one data attribute was received");
            received_object = Some(process_text_object_field::<T>(field).await);
        }
    }

    // TODO: manage this with an error instead of an assert
    assert!(counter == files_amount, "This endpoint expects {files_amount} files but less ({counter}) were sent");

    Ok((files_names, received_object.expect("No body was received")))
}

pub async fn upload_one_file_with_body<T: DeserializeOwned>(payload: Multipart) -> Result<(String, T), String> {
    let (files_array, returned_object) = upload_exact_amount_files_with_body::<T>(payload, 1).await?;
    let files_amount = files_array.len();

    // TODO: manage this with an error instead of an assert
    assert!(files_amount == 1, "This endpoint expects 1 file but {files_amount} were sent");

    Ok((files_array[0].clone(), returned_object))
}

// #[allow(dead_code)]
pub async fn upload_exact_amount_files(mut payload: Multipart, files_amount: u64) -> Result<Vec<String>, String> {
    let mut files_names: Vec<String> = Vec::new();
    let uploads_folder = "./uploads";
    create_folder(uploads_folder);
    let mut counter = 0;
    while let Ok(Some(field_result)) = payload.try_next().await {

        // TODO: manage this with an error instead of an assert
        assert!(counter < files_amount, "This endpoint expects {files_amount} files but more were sent");

        let mut field_is_file = true;
        let field = field_result;
        let filename = match field.content_disposition().get_filename() {
            Some(cd) => cd.to_string(),
            None => {
                field_is_file = false;
                "unknown".to_string()
            }
        };
        
        if field_is_file {
            let new_filename = process_file_field(field, uploads_folder, &filename).await;
            files_names.push(new_filename);
            counter += 1;
        }
    }

    // TODO: manage this with an error instead of an assert
    assert!(counter == files_amount, "This endpoint expects {files_amount} files but less were sent");

    Ok(files_names)
}

// #[allow(dead_code)]
pub async fn upload_one_file(payload: Multipart) -> Result<String, String> {
    let files_array = upload_exact_amount_files(payload, 1).await?;
    let files_amount = files_array.len();

    // TODO: manage this with an error instead of an assert
    assert!(files_amount == 1, "This endpoint expects 1 files but {files_amount} were sent");

    Ok(files_array[0].clone())
}