use std::{fs::{self, File}, path::Path, thread, time::Duration};
use tar::{Builder, Archive};

use crate::common::app_error::AppError;

// Controller 

fn compress_program_with_input(program_id: &str, input_group_id: &str, downloaded_program_file_path: &str, 
                                   program_file_name: &str, input_file_path: &str) -> Result<String, AppError> {
    let tar_file_path = format!("./aux_files/{}/{}_{}.tar", input_group_id, program_id, input_group_id);
    let tar_file = File::create(tar_file_path.clone())?;
    let mut tar_file_builder = Builder::new(tar_file);
    tar_file_builder.append_path_with_name(downloaded_program_file_path, program_file_name)?;
    tar_file_builder.append_path_with_name(input_file_path, format!("{}.csv", input_group_id))?;
    tar_file_builder.finish().expect("Error in builder finish");
    return Ok(tar_file_path);
}

pub fn open_named_file(file_path: &str) ->  Result<actix_files::NamedFile, AppError> {
    let tar_file = File::open(file_path)?;
    let named_file = actix_files::NamedFile::from_file(tar_file, file_path)?;
    return Ok(named_file);
}

pub fn manage_program_with_input_compression(program_id: &str, input_group_id: &str, downloaded_program_file_path: &str, 
                                             program_file_name: &str, input_file_path: &str) -> Result<actix_files::NamedFile, AppError> {
    let tar_file_path = compress_program_with_input(program_id, input_group_id, downloaded_program_file_path, program_file_name, input_file_path)?;
    return open_named_file(&tar_file_path);
}
