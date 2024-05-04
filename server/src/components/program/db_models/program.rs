use diesel::prelude::*;
use serde::{Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = crate::schema::program)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct StoredProgram {
    pub organization_id: String,
    pub program_id: String,
    pub description: String,
    pub name: String,
    pub input_lock_timeout: i64,
}