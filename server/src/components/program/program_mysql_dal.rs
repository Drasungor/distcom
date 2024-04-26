use std::fs::File;
use std::str::Bytes;
use actix_web::error::BlockingError;
use csv::Reader;
use diesel::connection;
use diesel::r2d2::PooledConnection;
use diesel::result::DatabaseErrorKind;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::web;
use uuid::Uuid;
use base64::prelude::*;
use csv;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use super::db_models::program::StoredProgram;
use super::db_models::program_input_group::ProgramInputGroup;
use super::db_models::specific_program_input::SpecificProgramInput;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::schema::program_input_group;
use crate::schema::specific_program_input;
use crate::schema::{program};

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
            // input_was_reserved: false,
            last_reserved: None,
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
    }

    pub async fn get_program_uploader_id(program_id: &String) -> Result<String, AppError> {
        let cloned_program_id = program_id.clone();
        
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let found_program = program::table
            .filter(program::program_id.eq(cloned_program_id))
            .first::<StoredProgram>(connection)?;

            return Ok(found_program.organization_id);
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Ok(organization_id)) => Ok(organization_id),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    _ => Err(AppError::new(AppErrorType::InternalServerError))
                }
            },
            Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError)),
        };
    }


    fn get_available_input_group_id(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, 
                                      program_id: &String, current_datetime: &NaiveDateTime) -> String {

        let returned_input_group;

        let found_program: StoredProgram = program::table
            .filter(program::program_id.eq(program_id.clone()))
            .first::<StoredProgram>(connection).expect("No program was found");


        let mut found_input_group: Result<ProgramInputGroup, _> = program_input_group::table
            .filter(program_input_group::program_id.eq(program_id.clone()).and(program_input_group::last_reserved.is_null()))
            .first::<ProgramInputGroup>(connection);


        if (found_input_group.is_ok()) {
            returned_input_group = found_input_group.unwrap();
        } else {
            let found_input_groups_array: Vec<ProgramInputGroup> = program_input_group::table
            .filter(program_input_group::program_id.eq(program_id).and(program_input_group::last_reserved.is_not_null()))
            .load::<ProgramInputGroup>(connection).expect("Error finding taken input groups");

            let mut chosen_input_index: i32 = -1;

            // Try to find of the reserved inputs one that suffered a timeout
            for i in 0..found_input_groups_array.len() {
                let current_input_group = &found_input_groups_array[i];
                let current_last_reserved_date = current_input_group.last_reserved.unwrap();
                let difference = *current_datetime - current_last_reserved_date;
                let difference_in_seconds = difference.num_seconds();
                if (difference_in_seconds > found_program.input_lock_timeout) {
                    chosen_input_index = i as i32;
                    break;
                }
            }
            assert!(chosen_input_index != -1, "No input group is available");
            returned_input_group = found_input_groups_array[chosen_input_index as usize].clone();
        }

        let input_group_id = returned_input_group.input_group_id;
        diesel::update(program_input_group::table.filter(program_input_group::input_group_id.eq(input_group_id.clone())))
                .set(program_input_group::last_reserved.eq(Some(current_datetime)))
                .execute(connection).expect("Error in input group update");

        return input_group_id;
    }

    fn store_input_group_in_csv(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, 
                                file_path: &String, input_group_id: &String) {
        let mut input_line_counter = 0;
        let mut current_input = specific_program_input::table
            .filter(specific_program_input::input_group_id.eq(input_group_id.clone()).and(specific_program_input::order.eq(input_line_counter)))
            // TODO: return a good error indicating that no unreserved input was found
            .first::<SpecificProgramInput>(connection);

        {
            let file = File::create(file_path.clone()).expect("Error in file creation");
        }
        let mut writer = csv::Writer::from_path(file_path.clone()).expect("Error in writer generation");

        while let Ok(input_tuple) = current_input {
            input_line_counter += 1;
            let encoded_data = BASE64_STANDARD.encode(input_tuple.blob_data.expect("Blob data is null"));
            writer.write_record(&[encoded_data]).expect("Error in writer");

            current_input = specific_program_input::table
            .filter(specific_program_input::input_group_id.eq(input_group_id.clone()).and(specific_program_input::order.eq(input_line_counter)))
            // TODO: return a good error indicating that no unreserved input was found
            .first::<SpecificProgramInput>(connection);
        }
    }

    pub async fn retrieve_input_group(program_id: &String) -> Result<(String, String), AppError> {
        let cloned_program_id = program_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let current_system_time = SystemTime::now();
            let since_the_epoch = current_system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
            let current_datetime = DateTime::from_timestamp_millis(since_the_epoch.as_millis().try_into().unwrap()).unwrap();
            let now_naive_datetime = current_datetime.naive_utc();

            let input_group_id = Self::get_available_input_group_id(connection, &cloned_program_id, &now_naive_datetime);

            let file_path = format!("./downloads/{}.csv", input_group_id);

            Self::store_input_group_in_csv(connection, &file_path, &input_group_id);

            return Ok((input_group_id, file_path));
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError)),
            Ok(Ok(result_tuple)) => Ok(result_tuple),
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
