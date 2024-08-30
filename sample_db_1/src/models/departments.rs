use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Departments{

    id :i32,
    name :String,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}