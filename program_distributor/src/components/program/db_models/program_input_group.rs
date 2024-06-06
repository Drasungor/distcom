use diesel::prelude::*;
use chrono::{NaiveDateTime};
use diesel::sql_types::Timestamp;
use std::time::{SystemTime};

#[derive(Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::program_input_group)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ProgramInputGroup {
    pub input_group_id: String,
    pub program_id: String,
    pub last_reserved: Option<NaiveDateTime>,
    pub proven_datetime: Option<NaiveDateTime>,
}