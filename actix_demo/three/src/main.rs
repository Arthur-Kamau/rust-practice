use std::sync::Mutex;
//examples -> https://github.com/actix/examples/tree/master
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod db_operations;
mod models;
mod schema;
use db_operations::db;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use chrono::Utc;
use crate::models::posts::{NewPost, Post};
use crate::models::users::{NewUser, Users};
use log::info;

struct AppState {
    db_connection:Mutex<PgConnection>
}

async fn update_user(item: web::Json<Users>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}
async fn update_post(item: web::Json<Post>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}
async fn register_user(item: web::Json<NewUser>, state: web::Data<AppState>) -> impl Responder {
    log::info!("Register user to implment");
    HttpResponse::Ok().finish()
}

async fn add_post(item: web::Json<NewPost>, state: web::Data<AppState>) -> impl Responder {
    log::info!("add post to implment");
    HttpResponse::Ok().finish()
}


async fn fetch_all_posts(state: web::Data<AppState>) -> impl Responder {
    log::info!("fetch all post to implment");
    let v :Vec<Post> = Vec::new();
    HttpResponse::Ok().json(&v)
}

async fn get_posts_item(state: web::Data<AppState>) -> impl Responder {
    log::info!("fetch all post to implment");

    let now = Utc::now().naive_utc();
    let a_post = Post{
        body:"test body".to_string(),
        title: "test title".to_string(),
        is_published: false,
        id:0,
        created_at: now,
        updated_at: now
    };
    HttpResponse::Ok().json(&a_post)
}

async fn delete_item(path: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().finish()
    log::info!("Register user to implment");
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    info!("starting HTTP server at http://localhost:8080");



    HttpServer::new(move || {
        // Initialize application state
        let app_state = web::Data::new(AppState { db_connection: Mutex::new(db::establish_connection())  });
        // todo improve above to use a  pool not a single connection

        App::new()
            .app_data(app_state.clone())
            .route("/register", web::post().to(register_user))
            .route("/post", web::post().to(add_post))
            .route("/posts", web::get().to(fetch_all_posts))
            .route("/posts/{id}", web::get().to(get_posts_item))
            .route("/posts/{id}", web::delete().to(delete_item))
            .route("/posts/update", web::put().to(update_post))
            .route("/users/update", web::put().to(update_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}