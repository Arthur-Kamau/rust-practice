use actix_session::Session;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_web::web::Redirect;
use actix_web_lab::__reexports::tracing::info;
use crate::models::users::{NewUser, Users, LoginForm, NewUserForm};
use crate::db_operations::users::{add_user, get_a_user_by_mail};
use crate::models::app_state::AppState;
use crate::models::ui::{LoginTemplate, DashboardTemplate, RegisterTemplate};
use bcrypt::{hash, DEFAULT_COST, verify};
use askama::Template;
use log::error;

pub async fn register_page(error: Option<String>) -> impl Responder {
    let template = RegisterTemplate { error };

    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}


pub async fn login_page(error: Option<String>, message : Option<String>) -> impl Responder {
    let template = LoginTemplate { error, message };

    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}


pub async fn login_user(form: web::Form<LoginForm>, state: web::Data<AppState>, session:Session) ->  Result<HttpResponse, actix_web::Error> {
    let mut connection = state.db_connection.lock().unwrap();
    let user_exist = get_a_user_by_mail(&mut *connection, form.email.clone());
    match user_exist {
        Some(user) => {
            if verify(&form.password, &user.password).unwrap_or(false) {
                let dashboard_template = DashboardTemplate {
                    email: form.email.clone(),
                };
                session.insert("user_email", form.email.clone())?;
                // HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().unwrap())
                Ok(HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                    .finish())
            } else {
                let error_message = "wrong password.".to_string();
                let template = LoginTemplate { error: Some(error_message), message : None };
                // HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
                Ok(HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template.render().unwrap()))
            }
        }
        None => {
            let error_message = "Email not found".to_string();
            let template = LoginTemplate { error: Some(error_message), message: None };
            // HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap()))
        }
    }
}

async fn handle_login_information(error: &str) -> HttpResponse {
    let template = LoginTemplate { error: None, message: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate { error: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn register_user(item: web::Form<NewUserForm>, state: web::Data<AppState>) -> impl Responder {
    if item.name.is_empty() || item.email.is_empty() || item.bio.is_empty() || item.password.is_empty() {
        return HttpResponse::BadRequest().body("All fields are required");
    }
    // Hash and salt the password
    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
    };
    // check if email exist
    let new_user = NewUser {
        name: item.name.clone(),
        email: item.email.clone(),
        bio: item.bio.clone(),
        password: hashed_password,
        avatar_url: "".to_string(),
        blocked_reason: "".to_string(),
    };
    let mut connection_guard = state.db_connection.lock().unwrap();
    let res = add_user(new_user, &mut *connection_guard);
    match res {
        Ok(user) => {
            return handle_login_information("Account created, please login to continue").await;
        }
        Err(err) => {
            error!("Error creating user {:#?}", err);
            return handle_register_error("Error Creating Acoout").await;
        }
    }


}


async fn update_user(item: web::Json<Users>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}