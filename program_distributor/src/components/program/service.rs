use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use uuid::Uuid;
use csv;
use std::fs::File; // Add import for File

use crate::common::app_error::{AppError, AppErrorType};
use crate::components::program::program_mysql_dal::ProgramMysqlDal;

use super::model::PagedPrograms;


pub struct ProgramService;

impl ProgramService {

    pub async fn add_organization_program(organization_id: String, program_id: String, name: String, description: String, input_lock_timeout: i64) -> Result<(), AppError> {
        ProgramMysqlDal::add_organization_program(organization_id, program_id, name, description, input_lock_timeout).await?;
        return Ok(());
    }

    pub async fn add_program_input_group(organization_id: &String, program_id: &String, input_file_path: &String) -> Result<(), AppError> {
        let file = File::open(input_file_path).expect("Error while reading file");
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
        let input_group_id = Uuid::new_v4().to_string();
        ProgramMysqlDal::add_input_group(organization_id, program_id, &input_group_id, reader).await?;
        return Ok(());
    }

    pub async fn retrieve_input_group(program_id: &String) -> Result<(String, String), AppError> {
        let (input_group_id, file_path) = ProgramMysqlDal::retrieve_input_group(program_id).await?;
        return Ok((input_group_id, file_path));
    }

    pub async fn get_program_uploader_id(program_id: &String) -> Result<String, AppError> {
        let organization_id = ProgramMysqlDal::get_program_uploader_id(program_id).await?;
        return Ok(organization_id);
    }

    pub async fn get_organization_programs(organization_id: String, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        return ProgramMysqlDal::get_organization_programs(organization_id, limit, page).await;
    }

    pub async fn get_general_programs(name_filter: Option<String>, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        return ProgramMysqlDal::get_general_programs(name_filter, limit, page).await;
    }

    pub async fn delete_input_group_reservation(input_group_id: &String) -> Result<(), AppError> {
        return ProgramMysqlDal::delete_input_group_reservation(input_group_id).await;
    }


}