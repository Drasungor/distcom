use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::program_input_group)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ProgramInputGroup {
    pub input_group_id: String,
    pub program_id: String,
    pub input_was_reserved: bool,
}