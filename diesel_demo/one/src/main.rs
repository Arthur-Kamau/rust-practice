// https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html
mod models;
mod schema;
use models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use chrono::Utc;
use crate::models::posts::{NewPost, Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}





fn main() {
    use schema::posts::dsl::*;

    let connection = &mut establish_connection();

    //let now = Utc::now().naive_utc();

    let new_example = NewPost {
        // id: 0,
        title: "a sample read 222".to_string(),
        body: "hahahha... the train is rolloing down 11".to_string(),
        // created_at: now,
        // updated_at: now,
        published: false,
    };

    let database_record = diesel::insert_into(schema::posts::table)
        .values(&new_example)
        .get_result::<Post>(connection)
        .expect("Error saving new example");

    println!("Inserted example: {:?}", database_record);



// select all *
//     let results = posts
//         .select(Post::as_select())
//         .load(connection)
//         .expect("Error loading posts");
//
//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.title);
//         println!("-----------\n");
//         println!("{}", post.body);
//     }




    // let results = posts
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .select(Post::as_select())
    //     .load(connection)
    //     .expect("Error loading posts");
    //
    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("-----------\n");
    //     println!("{}", post.body);
    // }
}


pub fn update_post_title_by_id(conn: & mut PgConnection, post_id: i32, new_title: &str) -> Post {
    // use schema::posts::dsl::{posts, id, title};
    use schema::posts::dsl::*;

    let target = posts.filter(id.eq(post_id));

    diesel::update(target)
        .set(title.eq(new_title))
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post with id {}", post_id))
}
