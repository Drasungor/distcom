use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use uuid::Uuid;

use crate::common::app_error::{AppError, AppErrorType};
use crate::components::program::program_mysql_dal::ProgramMysqlDal;


pub struct ProgramService;

impl ProgramService {

    pub async fn add_organization_program(organization_id: String, program_id: String, input_lock_timeout: i64) -> Result<(), AppError> {
        ProgramMysqlDal::add_organization_program(organization_id, program_id, input_lock_timeout).await?;
        return Ok(());
    }

}