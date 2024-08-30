
mod db_operations;
mod models;
mod schema;
mod controllers;



use db_operations::db;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use diesel::prelude::*;
use dotenvy::dotenv;
use chrono::Utc;
use crate::controllers::posts::{fetch_all_posts, get_posts_item , create_post, update_post, delete_post_item};
use crate::controllers::users::{login_page, login_user, register_page, register_user};
use crate::controllers::home::{default_handler};
use crate::models::users::{Users, LoginForm};
use crate::models::app_state;
use log::info;

use std::sync::Mutex;
use actix_session::config::PersistentSession;
use actix_web::{cookie::{time::Duration, Key}, web, App, HttpServer, Responder,  middleware, HttpResponse};
use askama::Template;
use crate::models::app_state::AppState;

use actix_web::{Error, dev::ServiceRequest, dev::ServiceResponse, HttpMessage};
use actix_web::cookie::SameSite;
use crate::controllers::dashboard::dashboard_page;


const ONE_MINUTE: Duration = Duration::minutes(1);




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    info!("starting HTTP server at http://localhost:8080");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        // Initialize application state
        let app_state = web::Data::new(AppState { db_connection: Mutex::new(db::establish_connection())  });
        // todo improve above to use a  pool not a single connection

        App::new()
            .app_data(app_state.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // set to true if using HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build()
            )
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login_user))
            .route("/register", web::get().to(register_page))
            .route("/register", web::post().to(register_user))
            .route("/dashboard", web::get().to(dashboard_page))
            .default_service(web::to(default_handler))

            // .service(web::resource("/restricted").route(web::get().to(restricted)))
            // .route("/post", web::post().to(add_post))
            // .route("/posts", web::get().to(fetch_all_posts))
            // .route("/posts/{id}", web::get().to(get_posts_item))
            // .route("/posts/{id}", web::delete().to(delete_post_item))
            // .route("/posts/update", web::put().to(update_post))
            // .route("/post/create", web::put().to(create_post))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}