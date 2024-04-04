use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::program)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct StoredProgram {
    pub organization_id: String,
    pub program_id: String,
    pub input_lock_timeout: i64,
}