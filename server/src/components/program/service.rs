use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use uuid::Uuid;
use csv;
use std::fs::File; // Add import for File

use crate::common::app_error::{AppError, AppErrorType};
use crate::components::program::program_mysql_dal::ProgramMysqlDal;


pub struct ProgramService;

impl ProgramService {

    pub async fn add_organization_program(organization_id: String, program_id: String, input_lock_timeout: i64) -> Result<(), AppError> {
        ProgramMysqlDal::add_organization_program(organization_id, program_id, input_lock_timeout).await?;
        return Ok(());
    }

    pub async fn add_program_input_group(organization_id: &String, program_id: &String, input_file_path: &String) -> Result<(), AppError> {
        let file = File::open(input_file_path).expect("Error while reading file");
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
        let input_group_id = Uuid::new_v4().to_string();
        ProgramMysqlDal::add_input_group(organization_id, program_id, &input_group_id, reader).await?;
        return Ok(());
    }

}