use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employees{

    id :i32,
    dept_id :Option<i32>,
    user_id :Option<i32>,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}