use actix_session::Session;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, Error};
use crate::models::users::{NewUser, Users, LoginForm};
use crate::db_operations::users::get_a_user_by_mail;
use crate::models::app_state::AppState;
use crate::models::ui::{LoginTemplate, DashboardTemplate, RegisterTemplate};
use bcrypt::{hash, DEFAULT_COST, verify};
use askama::Template;
use serde::Serialize;

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}
pub async fn protected(session: Session) -> Result<HttpResponse, Error> {
    if let Some(user_id) = session.get::<String>("user_id")? {
        Ok(HttpResponse::Ok().json(LoginResponse {
            message: format!("Welcome to the protected route, {}!", user_id),
        }))
    } else {
        Ok(HttpResponse::Unauthorized().json(LoginResponse {
            message: "Access denied".to_string(),
        }))
    }
}

pub async fn unprotected() -> impl Responder {
    HttpResponse::Ok().json(LoginResponse {
        message: "This is an unprotected route".to_string(),
    })
}