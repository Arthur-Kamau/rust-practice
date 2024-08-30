use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: String,
    pub is_blocked: bool,
    pub user_type: i32,
    pub reset_code: Option<i32>,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}





#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {

    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: String,

}