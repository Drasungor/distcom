use std::fs::File;
use std::str::Bytes;

use actix_web::error::BlockingError;
use csv::Reader;
use diesel::connection;
use diesel::r2d2::PooledConnection;
use diesel::result::DatabaseErrorKind;
// use super::{dal::AccountDal, db_models::account::NewAccount};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;
use uuid::Uuid;
use base64::prelude::*;

use super::db_models::program::StoredProgram;
use super::db_models::program_input_group::ProgramInputGroup;
use super::db_models::specific_program_input::SpecificProgramInput;
// use super::db_models::refresh_token::RefreshToken;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::schema::program_input_group;
use crate::schema::specific_program_input;
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

    fn store_inputs(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, input_group_id: String, mut input_reader: Reader<File>) -> Result<(), diesel::result::Error> {
        // Storage of specific inputs
        let mut current_input = 0;
        for line in input_reader.records() {
            let line_ok = line.expect("Error in line reading");
            let line_iterator = line_ok.into_iter();
            let mut counter = 0;

            for value in line_iterator {
                let specific_input = SpecificProgramInput {
                    specific_input_id: Uuid::new_v4().to_string(),
                    input_group_id: input_group_id.clone(),
                    blob_data: Some(BASE64_STANDARD.decode(value).expect("Error in base 64 decoding")),
                    order: current_input
                };
                counter += 1;
                diesel::insert_into(specific_program_input::table)
                    .values(&specific_input)
                    .execute(connection)?;
            }
            assert!(counter == 1, "There is more than one element per line");
            current_input += 1;
        }
        return Ok(());
    }

    pub async fn add_input_group(organization_id: &String, program_id: &String, input_group_id: &String, mut input_reader: Reader<File>) -> Result<(), AppError> {
        let cloned_organization_id = organization_id.clone();
        let cloned_input_group_id = input_group_id.clone();
        let cloned_program_id = program_id.clone();
        
        let program_input_group = ProgramInputGroup {
            input_group_id: cloned_input_group_id.clone(),
            program_id: cloned_program_id.clone(),
            input_was_reserved: false,
        };

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            // TODO: Check why when no value is found we do not return an error, probably not returning a value is not viewed as an
            // error, but as a valid result
            program::table
                .filter(program::program_id.eq(cloned_program_id).and(program::organization_id.eq(cloned_organization_id)))
                .first::<StoredProgram>(connection)?;

            diesel::insert_into(program_input_group::table)
                    .values(&program_input_group)
                    .execute(connection)?;

            // // Storage of specific inputs
            // let mut current_input = 0;
            // for line in input_reader.records() {
            //     let line_ok = line.expect("Error in line reading");
            //     let line_iterator = line_ok.into_iter();
            //     let mut counter = 0;

            //     for value in line_iterator {
            //         let specific_input = SpecificProgramInput {
            //             specific_input_id: Uuid::new_v4().to_string(),
            //             input_group_id: cloned_input_group_id.clone(),
            //             blob_data: Some(BASE64_STANDARD.decode(value).expect("Error in base 64 decoding")),
            //             order: current_input
            //         };
            //         counter += 1;
            //         diesel::insert_into(specific_program_input::table)
            //             .values(&specific_input)
            //             .execute(connection)?;
            //     }
            //     assert!(counter == 1, "There is more than one element per line");
            //     current_input += 1;
            // }

            Self::store_inputs(connection, cloned_input_group_id, input_reader)?;

            return Ok(());
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
        };
        // return Ok(());
    }

    // pub async fn retrieve_input_group(organization_id: &String, program_id: &String, input_group_id: &String, mut input_reader: Reader<File>) -> Result<(), AppError> {
    pub async fn retrieve_input_group(program_id: &String) -> Result<(), AppError> {
        let cloned_program_id = program_id.clone();

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            // program::table
            // .filter(program::program_id.eq(cloned_program_id).and(program::organization_id.eq(cloned_organization_id)))
            // .first::<StoredProgram>(connection)?;

            let found_input_group: ProgramInputGroup = program_input_group::table
                .filter(program_input_group::program_id.eq(cloned_program_id).and(program_input_group::input_was_reserved.eq(false)))
                // TODO: return a good error indicating that no unreserved input was found
                .first::<ProgramInputGroup>(connection).expect("No input group was found");

            let input_group_id = found_input_group.input_group_id;

            // diesel::update(program_input_group::table.find(input_group_id))
            diesel::update(program_input_group::table.filter(program_input_group::input_group_id.eq(input_group_id)))
                .set(program_input_group::input_was_reserved.eq(true))
                .execute(connection).expect("Error in input group update");

            // while let Ok(current_input) =  {
                
            // }

            return Ok(());
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
        };
    }
}
