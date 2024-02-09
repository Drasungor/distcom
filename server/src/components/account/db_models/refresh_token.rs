use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::refresh_token)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RefreshToken {
    pub token_id: i64,
    pub user_id: i64,
}