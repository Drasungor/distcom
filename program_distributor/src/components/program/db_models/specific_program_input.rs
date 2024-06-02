use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::specific_program_input)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SpecificProgramInput {
    pub specific_input_id: String,
    pub input_group_id: String,

    // TODO: Check why this is an option, maybe it could be just a u8 vec
    pub blob_data: Option<Vec<u8>>,
    pub order: i32,
}