use crate::models::posts::{NewPost, Post};
use diesel::prelude::*;

pub fn get_all_posts(connection :& mut  PgConnection)->Vec<Post>{
    use crate::schema::posts::dsl::*;

    let mut all_posts :Vec<Post>=Vec::new();
    let results = posts
        .filter(is_published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection);
    match results {
        Ok(data)=>{
            for post in data.into_iter(){
                all_posts.push(post)
            }

         println!("todo")
        }
        Err(e)=>println!("Error occured {:?}",e)
    }

    return all_posts;

}


pub fn add_posts(new_post : NewPost, connection :& mut  PgConnection)->Result<bool,String>{
    let res = diesel::insert_into(crate::schema::posts::table)
        .values(&new_post)
        // .returning(Post::as_returning())
        .get_result::<Post>(connection);

    return  match res {
        Ok(data)=>{

            Ok(true)
        }
        Err(e)=>{
            let f= format!("Error occurred {:?}",e);
            println!("{}", f);
            Err(f)
        }
    }
}