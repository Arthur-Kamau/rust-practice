use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug,Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users{
    pub   id :i32,
    pub user_types:i32,
    pub name: String,
    pub  email: String,
    pub  bio: String,
    pub  blocked_reason: String,
    pub  is_blocked: bool,
    pub  is_deleted: bool,
    pub updated_at: chrono::NaiveDateTime,
    pub  created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUsers{
   pub name: String,
    pub email: String,
    pub bio: String,
    pub blocked_reason: String
}