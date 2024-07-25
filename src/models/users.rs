use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub user_types: i32,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub password: String,
    pub avatar_url: String,
    pub is_blocked: bool,
    pub blocked_reason: Option<String>,
    pub is_deleted: bool,
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
    pub bio: String,
    pub password: String,
    pub avatar_url: String,
    pub blocked_reason:Option<String>
}

// Provide a default implementation for avatar_url
fn default_avatar_url() -> String {
    "".to_string()
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  email: String,
    pub  password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub bio: String,
    pub password: String
}