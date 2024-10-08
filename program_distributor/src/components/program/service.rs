use uuid::Uuid;
use csv;
use std::fs::File; // Add import for File

use crate::common::app_error::AppError;
use crate::components::program::program_mysql_dal::ProgramMysqlDal;
use super::model::{PagedProgramInputGroups, PagedPrograms};


pub struct ProgramService;

impl ProgramService {

    pub async fn add_organization_program(organization_id: String, program_id: String, name: String, description: String, input_lock_timeout: i64) -> Result<(), AppError> {
        ProgramMysqlDal::add_organization_program(organization_id, program_id, name, description, input_lock_timeout).await?;
        Ok(())
    }

    pub async fn add_program_input_group(organization_id: &String, program_id: &String, name: &String, input_file_path: &String) -> Result<String, AppError> {
        let file = File::open(input_file_path)?;
        let reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
        let input_group_id = Uuid::new_v4().to_string();
        ProgramMysqlDal::add_input_group(organization_id, program_id, &input_group_id, name, reader).await?;
        Ok(input_group_id)
    }

    pub async fn retrieve_input_group(program_id: &String) -> Result<(String, String), AppError> {
        let (input_group_id, file_path) = ProgramMysqlDal::retrieve_input_group(program_id).await?;
        Ok((input_group_id, file_path))
    }

    pub async fn set_input_group_as_proven(program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        ProgramMysqlDal::set_input_group_as_proven(program_id, input_group_id).await
    }

    pub async fn delete_input_group_proven_mark(organization_id: &String, program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        ProgramMysqlDal::delete_input_group_proven_mark(organization_id, program_id, input_group_id).await
    }
    
    pub async fn confirm_proof_validity(organization_id: &String, program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        ProgramMysqlDal::delete_input_group_entry(organization_id, program_id, input_group_id).await
    }

    pub async fn delete_program(organization_id: &String, program_id: &String) -> Result<(), AppError> {
        ProgramMysqlDal::delete_program(organization_id, program_id).await
    }

    pub async fn get_programs_with_proven_executions(organization_id: &String, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        ProgramMysqlDal::get_programs_with_proven_executions(organization_id, limit, page).await
    }

    pub async fn get_input_groups_with_proven_executions(organization_id: &String, program_id: &String, limit: i64, page: i64) -> Result<PagedProgramInputGroups, AppError> {
        ProgramMysqlDal::get_input_groups_with_proven_executions(organization_id, program_id, limit, page).await
    }

    pub async fn get_input_groups(organization_id: &String, program_id: &String, limit: i64, page: i64) -> Result<PagedProgramInputGroups, AppError> {
        ProgramMysqlDal::get_input_groups(organization_id, program_id, limit, page).await
    }

    pub async fn delete_input_group(organization_id: &String, program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        ProgramMysqlDal::delete_input_group(organization_id, program_id, input_group_id).await
    }

    pub async fn get_program_uploader_id(program_id: &String) -> Result<String, AppError> {
        let organization_id = ProgramMysqlDal::get_program_uploader_id(program_id).await?;
        Ok(organization_id)
    }

    pub async fn get_organization_programs(organization_id: String, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        ProgramMysqlDal::get_organization_programs(organization_id, limit, page).await
    }

    pub async fn get_general_programs(name_filter: Option<String>, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        ProgramMysqlDal::get_general_programs(name_filter, limit, page).await
    }

    pub async fn delete_input_group_reservation(input_group_id: &String) -> Result<(), AppError> {
        ProgramMysqlDal::delete_input_group_reservation(input_group_id).await
    }


}