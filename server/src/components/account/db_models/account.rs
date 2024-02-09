use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Account {
    pub organization_id: i64,
    pub name: String,
    pub description: String,
    pub account_was_verified: bool,
    pub username: String,
    pub password: String,
}