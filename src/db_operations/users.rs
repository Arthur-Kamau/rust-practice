
use crate::models::users::{NewUser, Users};
use diesel::prelude::*;

pub fn get_all_users(connection: &mut PgConnection) -> Vec<Users> {
    use crate::schema::users::dsl::*;

    let mut all_posts: Vec<Users> = Vec::new();
    let results = users
        .select(Users::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for post in data.into_iter() {
                all_posts.push(post)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_posts;
}

pub fn get_a_user_by_mail(connection: &mut PgConnection, user_email: String) -> Option<Users> {
    use crate::schema::users::dsl::*;


    users
        .filter(email.eq(user_email))
        .first::<Users>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn get_a_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<Users> {
    use crate::schema::users::dsl::*;


    users
        .filter(id.eq(user_id))
        .first::<Users>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}


pub fn add_user(new_user: NewUser, connection: &mut PgConnection) -> Result<Users, diesel::result::Error>{
     diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        // .returning(Post::as_returning())
        .get_result::<Users>(connection)
}