use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub published: bool,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPost {

    pub title: String,
    pub body: String,
    pub published: bool,

}
