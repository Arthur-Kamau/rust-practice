use chrono::Utc;
use crate::models::posts::{NewPost, Post};
use crate::models::users::{NewUser, Users, LoginForm};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::models::app_state::AppState;

pub async fn create_post(form: web::Form<LoginForm>) -> impl Responder {
    HttpResponse::Ok().finish()
}
pub async fn fetch_all_posts(state: web::Data<AppState>) -> impl Responder {
    log::info!("fetch all post to implment");
    let v :Vec<Post> = Vec::new();
    HttpResponse::Ok().json(&v)
}

pub async fn get_posts_item(state: web::Data<AppState>) -> impl Responder {
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

pub async fn update_post(item: web::Json<Post>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}

pub async fn delete_post_item(path: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().finish()
    log::info!("Register user to implment");
    HttpResponse::NotFound().finish()
}

