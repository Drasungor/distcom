use std::fs::File;
use std::str::Bytes;

use actix_web::error::BlockingError;
use csv::Reader;
use diesel::connection;
use diesel::result::DatabaseErrorKind;
// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;
use uuid::Uuid;

use super::db_models::program::StoredProgram;
use super::db_models::program_input_group::ProgramInputGroup;
use super::db_models::specific_program_input::SpecificProgramInput;
// use super::db_models::refresh_token::RefreshToken;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::schema::program_input_group;
use crate::schema::{program};
// use crate::schema::account::dsl::*;

pub struct ProgramMysqlDal;

impl ProgramMysqlDal {

    pub async fn add_organization_program(organization_id: String, program_id: String, input_lock_timeout: i64) -> Result<(), AppError> {

        let stored_program = StoredProgram {
            organization_id,
            program_id,
            input_lock_timeout,
        };

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let insertion_result = diesel::insert_into(program::table)
                    .values(&stored_program)
                    .execute(connection);
            return insertion_result;

        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Ok(_)) => Ok(()),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    _ => Err(AppError::new(AppErrorType::InternalServerError))
                }
            },
            Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError)),
        }
    }


    
    // pub async fn add_input_group(organization_id: String, program_id: String, inputs: &[&[u8]]) -> Result<(), AppError> {
    // pub async fn add_input_group(organization_id: String, program_id: String, inputs: &Vec<Vec<u8>>) -> Result<(), AppError> {
    pub async fn add_input_group(organization_id: &String, program_id: &String, input_group_id: &String, mut input_reader: Reader<File>) -> Result<(), AppError> {

        // // let stored_program = StoredProgram {
        // //     organization_id,
        // //     program_id,
        // //     input_lock_timeout,
        // // };

        // let program_input_group = ProgramInputGroup {
        //     input_group_id,
        //     program_id: program_id.clone(),
        //     input_was_reserved: false,
        // };

        // let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        // let result = web::block(move || {
        // connection.transaction::<_, diesel::result::Error, _>(|connection| {

        //     program::table
        //         .filter(program::program_id.eq(program_id))
        //         .first::<StoredProgram>(connection)?;
        // // return found_account;

        //     diesel::insert_into(program_input_group::table)
        //             .values(&program_input_group)
        //             .execute(connection)?;

        //     let mut current_input = 0;
        //     for line in input_reader.records() {
        //         let specific_input = SpecificProgramInput {
        //             specific_input_id: Uuid::new_v4().to_string(),
        //             input_group_id,
        //             // blob_data: Option<Vec<u8>>,
        //             blob_data: Some(line.expect("Error in line reading").into_iter()),
        //             order: current_input
        //         };

        //         current_input += 1;
        //     }

        //     return Ok(());
        // })
        // }).await;
        // return match result {
        //     Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
        //     Ok(Ok(_)) => Ok(()),
        //     Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
        //         match db_err_kind {
        //             DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
        //             _ => Err(AppError::new(AppErrorType::InternalServerError))
        //         }
        //     },
        //     Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError)),
        // };

        return Ok(());
    }

}
