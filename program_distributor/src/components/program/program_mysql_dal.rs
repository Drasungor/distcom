use std::fs::File;
use csv::Reader;
use diesel::r2d2::PooledConnection;
use diesel::result::DatabaseErrorKind;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use actix_web::web;
use uuid::Uuid;
use base64::prelude::*;
use csv;
use chrono::NaiveDateTime;

use super::db_models::program::StoredProgram;
use super::db_models::program_input_group::ProgramInputGroup;
use super::db_models::specific_program_input::SpecificProgramInput;
use super::model::PagedProgramInputGroups;
use super::model::PagedPrograms;
use crate::common::app_error::AppError;
use crate::common::app_error::AppErrorType;
use crate::common::app_error::InternalServerErrorType;
use crate::components::account::db_models::account::CompleteAccount;
use crate::schema::program_input_group;
use crate::schema::specific_program_input;
use crate::schema::{program, account};
use crate::utils::datetime_helpers::get_current_naive_datetime;
use crate::utils::diesel_helpers::general_manage_diesel_task_result;

pub struct ProgramMysqlDal;

impl ProgramMysqlDal {

    pub async fn add_organization_program(organization_id: String, program_id: String, name: String, description: String, input_lock_timeout: i64) -> Result<(), AppError> {
        let stored_program = StoredProgram {
            organization_id,
            program_id,
            name,
            description,
            input_lock_timeout,
        };

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {
            diesel::insert_into(program::table)
                        .values(&stored_program)
                        .execute(connection)?;
            return Ok(());

        })
        }).await;
        return general_manage_diesel_task_result(result);
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
            last_reserved: None,
            proven_datetime: None,
        };

        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        // let mut connection: Result<PooledConnection<ConnectionManager<MysqlConnection>>, diesel::r2d2::Error> = crate::common::config::CONNECTION_POOL.get();
        // // : r2d2::Error
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
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
            Ok(Ok(_)) => Ok(()),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
                }
            },
            Ok(Err(_)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError("Unknown error: ".to_string())))),
        };
        // return general_manage_diesel_task_result(result);
    }

    pub async fn get_program_uploader_id(program_id: &String) -> Result<String, AppError> {
        let cloned_program_id = program_id.clone();
        
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        // connection.transaction::<_, diesel::result::Error, _>(|connection| {
        connection.transaction::<_, AppError, _>(|connection| {

            let found_program = program::table
            .filter(program::program_id.eq(cloned_program_id))
            .first::<StoredProgram>(connection)?;

            return Ok(found_program.organization_id);
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
            Ok(Ok(organization_id)) => Ok(organization_id),
            // Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
            //     match db_err_kind {
            //         DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
            //         unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
            //     }
            // },
            // // Ok(Err(AppError)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
            // Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
            Ok(Err(err)) => Err(err),
        };
        // return general_manage_diesel_task_result(result);
    }


    // fn get_available_input_group_id(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, 
    //                                   program_id: &String, current_datetime: &NaiveDateTime) -> String {
    fn get_available_input_group_id(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, 
        program_id: &String, current_datetime: &NaiveDateTime) -> Result<String, AppError> {
    // fn get_available_input_group_id(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, 
    //     program_id: &String, current_datetime: &NaiveDateTime) -> Result<String, AppErrorType> {
        
        let returned_input_group;

        // let found_program: StoredProgram = program::table
        //     .filter(program::program_id.eq(program_id.clone()))
        //     .first::<StoredProgram>(connection).expect("No program was found");

        let found_program_option: Option<StoredProgram> = program::table
            .filter(program::program_id.eq(program_id.clone()))
            .first::<StoredProgram>(connection).optional()?;

        let found_program: StoredProgram;
        if let Some(found_program_value) = found_program_option {
            found_program = found_program_value;
        } else {
            return Err(AppError::new(AppErrorType::ProgramNotFound))
        }

        // let found_input_group: Result<ProgramInputGroup, _> = program_input_group::table
        //     .filter(program_input_group::program_id.eq(program_id.clone()).and(program_input_group::last_reserved.is_null()))
        //     .first::<ProgramInputGroup>(connection);
        let found_input_group_option: Option<ProgramInputGroup> = program_input_group::table
            .filter(program_input_group::program_id.eq(program_id.clone()).and(program_input_group::last_reserved.is_null()))
            .first::<ProgramInputGroup>(connection).optional()?;
        // let found_input_group: ProgramInputGroup;
        // if let Some(found_input_group_value) = found_input_group_option {
        //     found_input_group = found_input_group_value;
        // } else {
        //     return Err(AppError::new(AppErrorType::ProgramNotFound))
        // }

        if let Some(found_input_group) = found_input_group_option {
            returned_input_group = found_input_group;
        } else {
            let found_input_groups_array: Vec<ProgramInputGroup> = program_input_group::table
            .filter(program_input_group::program_id.eq(program_id).and(program_input_group::last_reserved.is_not_null()))
            .load::<ProgramInputGroup>(connection)?;

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
            if chosen_input_index == -1 {
                return Err(AppError::new(AppErrorType::InputGroupNotFound));
            }
            returned_input_group = found_input_groups_array[chosen_input_index as usize].clone();
        }

        // if found_input_group.is_ok() {
        //     returned_input_group = found_input_group.unwrap();
        // } else {
        //     let found_input_groups_array: Vec<ProgramInputGroup> = program_input_group::table
        //     .filter(program_input_group::program_id.eq(program_id).and(program_input_group::last_reserved.is_not_null()))
        //     .load::<ProgramInputGroup>(connection)?;

        //     let mut chosen_input_index: i32 = -1;

        //     // Try to find of the reserved inputs one that suffered a timeout
        //     for i in 0..found_input_groups_array.len() {
        //         let current_input_group = &found_input_groups_array[i];
        //         let current_last_reserved_date = current_input_group.last_reserved.unwrap();
        //         let difference = *current_datetime - current_last_reserved_date;
        //         let difference_in_seconds = difference.num_seconds();
        //         if (difference_in_seconds > found_program.input_lock_timeout) {
        //             chosen_input_index = i as i32;
        //             break;
        //         }
        //     }
        //     assert!(chosen_input_index != -1, "No input group is available");
        //     returned_input_group = found_input_groups_array[chosen_input_index as usize].clone();
        // }

        let input_group_id = returned_input_group.input_group_id;
        diesel::update(program_input_group::table.filter(program_input_group::input_group_id.eq(input_group_id.clone())))
                .set(program_input_group::last_reserved.eq(Some(current_datetime)))
                // .execute(connection).expect("Error in input group update");
                .execute(connection)?;

        return Ok(input_group_id);
    }

    fn store_input_group_in_csv(connection: &mut PooledConnection<ConnectionManager<MysqlConnection>>, 
                                file_path: &String, input_group_id: &String) {
        let mut input_line_counter = 0;
        let mut current_input = specific_program_input::table
            .filter(specific_program_input::input_group_id.eq(input_group_id.clone()).and(specific_program_input::order.eq(input_line_counter)))
            // TODO: return a good error indicating that no unreserved input was found
            .first::<SpecificProgramInput>(connection);

        {
            // println!("file_path: {}", file_path);

            // TODO: change this so that the created folder comes from the parent directory of the file path
            // stored in the variable "file_path"
            std::fs::create_dir_all(format!("./aux_files/{}", input_group_id)).expect("Error while creating parent dir path");


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
        // connection.transaction::<_, diesel::result::Error, _>(|connection| {
        connection.transaction::<_, AppError, _>(|connection| {
            let now_naive_datetime = get_current_naive_datetime();
            let input_group_id = Self::get_available_input_group_id(connection, &cloned_program_id, &now_naive_datetime)?;
            let file_path = format!("./aux_files/{}/{}.csv", input_group_id, input_group_id);
            Self::store_input_group_in_csv(connection, &file_path, &input_group_id);
            return Ok((input_group_id, file_path));
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
            Ok(Ok(result_tuple)) => Ok(result_tuple),
            // Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
            //     match db_err_kind {
            //         DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
            //         unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
            //     }
            // },
            // Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
            Ok(Err(err)) => Err(err),
        };
    }

    pub async fn set_input_group_as_proven(program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        let cloned_program_id = program_id.clone();
        let cloned_input_group_id = input_group_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {
            let now_naive_datetime = get_current_naive_datetime();
            diesel::update(program_input_group::table.filter(
                                program_input_group::input_group_id.eq(cloned_input_group_id.clone()).
                                and(program_input_group::program_id.eq(cloned_program_id.clone()))))
                    .set(program_input_group::proven_datetime.eq(Some(now_naive_datetime)))
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        return general_manage_diesel_task_result(result);
    }

    pub async fn delete_input_group_proven_mark(organization_id: &String, program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        let cloned_organization_id = organization_id.clone();
        let cloned_program_id = program_id.clone();
        let cloned_input_group_id = input_group_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let found_program = program::table
                .filter(program::program_id.eq(&cloned_program_id).and(program::organization_id.eq(cloned_organization_id)))
                .first::<StoredProgram>(connection)?;


            diesel::update(program_input_group::table.filter(
                                program_input_group::input_group_id.eq(&cloned_input_group_id).
                                and(program_input_group::program_id.eq(&cloned_program_id))))
                    .set((
                        program_input_group::proven_datetime.eq(None::<NaiveDateTime>), 
                        program_input_group::last_reserved.eq(None::<NaiveDateTime>)
                    ))
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        return general_manage_diesel_task_result(result);
    }

    pub async fn delete_input_group_entry(organization_id: &String, program_id: &String, input_group_id: &String) -> Result<(), AppError> {
        let cloned_organization_id = organization_id.clone();
        let cloned_program_id = program_id.clone();
        let cloned_input_group_id = input_group_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let found_program = program::table
                .filter(program::program_id.eq(&cloned_program_id).and(program::organization_id.eq(cloned_organization_id)))
                .first::<StoredProgram>(connection)?;


            diesel::delete(program_input_group::table.filter(
                                program_input_group::input_group_id.eq(cloned_input_group_id.clone()).
                                and(program_input_group::program_id.eq(cloned_program_id.clone()))))
                    .execute(connection)?;
            return Ok(());
        })
        }).await;
        return general_manage_diesel_task_result(result);
    }


    pub async fn delete_input_group_reservation(input_group_id: &String) -> Result<(), AppError> {
        let cloned_input_group_id = input_group_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            diesel::update(program_input_group::table.filter(program_input_group::input_group_id.eq(cloned_input_group_id)))
                .set(program_input_group::last_reserved.eq(None::<NaiveDateTime>))
                .execute(connection)?;
            return Ok(());
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
            Ok(Ok(result_tuple)) => Ok(result_tuple),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
                }
            },
            Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        };
    }

    pub async fn get_programs_with_proven_executions(organization_id: &String, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        let cloned_organization_id = organization_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            // TODO: check if we should ask first for the organization programs and then get the proven programs, or maybe 
            // we could join the two queries into one
            let proven_programs: Vec<String> = program_input_group::table
                .filter(program_input_group::proven_datetime.is_not_null())
                .select(program_input_group::program_id)
                .distinct()
                .load::<String>(connection)?;

            let programs: Vec<StoredProgram> = program::table
                .filter(program::program_id.eq_any(&proven_programs).and(program::organization_id.eq(&cloned_organization_id)))
                .offset((page - 1) * limit).limit(limit)
                .load::<StoredProgram>(connection)?;

            let count_of_matched_elements: i64 = program::table
                .filter(program::program_id.eq_any(proven_programs).and(program::organization_id.eq(cloned_organization_id)))
                .count().get_result(connection)?;
            
            return Ok(PagedPrograms {
                programs,
                total_elements_amount: count_of_matched_elements,
            });
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
            Ok(Ok(result)) => Ok(result),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
                }
            },
            Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        };
    }

    pub async fn get_input_groups_with_proven_executions(organization_id: &String, program_id: &String, limit: i64, page: i64) -> Result<PagedProgramInputGroups, AppError> {
        let cloned_organization_id = organization_id.clone();
        let cloned_program_id = program_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {
            program::table
                .filter(program::program_id.eq(&cloned_program_id).and(program::organization_id.eq(cloned_organization_id)))
                .first::<StoredProgram>(connection)?;

            let proven_input_groups: Vec<ProgramInputGroup> = program_input_group::table
                .filter(program_input_group::proven_datetime.is_not_null().and(program_input_group::program_id.eq(&cloned_program_id)))
                .offset((page - 1) * limit).limit(limit)
                .load::<ProgramInputGroup>(connection)?;

            let count_of_matched_elements = program_input_group::table
                .filter(program_input_group::proven_datetime.is_not_null().and(program_input_group::program_id.eq(cloned_program_id)))
                .count().get_result(connection)?;
                
            return Ok(PagedProgramInputGroups {
                program_input_groups: proven_input_groups,
                total_elements_amount: count_of_matched_elements,
            });
        })
        }).await;
        return match result {
            Err(BlockingError) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::TaskSchedulingError))),
            Ok(Ok(result_array)) => Ok(result_array),
            Ok(Err(diesel::result::Error::DatabaseError(db_err_kind, info))) => {
                match db_err_kind {
                    DatabaseErrorKind::UniqueViolation => Err(AppError::new(AppErrorType::UsernameAlreadyExists)),
                    unknown_database_error => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown database error: {:?}", unknown_database_error)))))
                }
            },
            Ok(Err(err)) => Err(AppError::new(AppErrorType::InternalServerError(InternalServerErrorType::UnknownError(format!("Unknown error: {:?}", err))))),
        };
    }


    pub async fn get_organization_programs(organization_id: String, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {
            account::table
                .filter(account::account_was_verified.eq(true))
                .first::<CompleteAccount>(connection)?;

            let programs: Vec<StoredProgram> = program::table
                .filter(program::organization_id.eq(&organization_id))
                .offset((page - 1) * limit).limit(limit)
                .load::<StoredProgram>(connection)?;

            let count_of_matched_elements: i64 = program::table
                .filter(program::organization_id.eq(&organization_id))
                .count().get_result(connection)?;
 
            return Ok(PagedPrograms {
                programs,
                total_elements_amount: count_of_matched_elements,
            });
        })
        }).await;
        return general_manage_diesel_task_result(found_account_result);
    }

    

    pub async fn get_general_programs(name_filter: Option<String>, limit: i64, page: i64) -> Result<PagedPrograms, AppError> {
        // let cloned_organization_id = organization_id.clone();
        let mut connection = crate::common::config::CONNECTION_POOL.get().expect("get connection failure");
        let found_account_result = web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {

            let mut programs_query = program::table.offset((page - 1) * limit).limit(limit).into_boxed();

            let mut count_of_matched_elements_query = program::table.into_boxed();

            if let Some(name_string) = name_filter {
                programs_query = programs_query.filter(program::name.like(format!("{}%", name_string)));
                count_of_matched_elements_query = count_of_matched_elements_query.filter(program::name.like(format!("{}%", name_string)))
            }

            let programs: Vec<StoredProgram> = programs_query.load::<StoredProgram>(connection)?;
            let count_of_matched_elements = count_of_matched_elements_query.count().get_result(connection)?;
 
            return Ok(PagedPrograms {
                programs,
                total_elements_amount: count_of_matched_elements,
            });
        })
        }).await;
        return general_manage_diesel_task_result(found_account_result);
    }

}
