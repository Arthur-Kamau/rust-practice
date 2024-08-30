mod schema;
mod models;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models::users::{Users,NewUsers};
use schema::users::dsl::*;
use diesel::{Connection, RunQueryDsl};


fn main() {
    println!("Hello, world!");

    dotenvy::dotenv();

    let db_url = env::var("DATABASE_URL").expect("Database url cannot be found");


   let mut connection = PgConnection::establish(&db_url).unwrap_or_else(|e| panic!("Error connecting to db {}", e));

    // let now = Utc::now().naive_utc();
    let new_user = NewUsers {
        name: " Cathrine".to_string(),
        bio: "Hr Officer".to_string(),
        blocked_reason: "".to_string(),
        email: "cathy@proton.com".to_string(),

    };

   diesel::insert_into(
        schema::users::table
    ).values(
        &new_user
    )  .returning(Users::as_returning())
       .get_result(&mut connection).expect("Error saving user");

}
