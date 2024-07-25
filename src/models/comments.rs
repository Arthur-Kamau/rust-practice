use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::comments;

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_published: bool,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}


#[derive( Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewComment {

    pub title: String,
    pub body: String,
    pub is_published: bool,

}
