use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::account)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(check_for_backend(diesel::mysql::MysqlConnection))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}