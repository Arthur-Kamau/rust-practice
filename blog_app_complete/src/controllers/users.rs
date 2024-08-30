use actix_session::Session;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_web::web::Redirect;
use crate::models::users::{NewUser, Users, LoginForm, RegisterForm};
use crate::db_operations::users::{add_user, get_a_user_by_id, get_a_user_by_mail};
use crate::models::app_state::AppState;
use crate::models::ui::{LoginTemplate, DashboardTemplate, RegisterTemplate};
use bcrypt::{hash, DEFAULT_COST, verify};
use askama::Template;
use log::{error, info, debug};

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate { error: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn register_page(error: Option<String>) -> impl Responder {
    let template = RegisterTemplate { error };

    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}


async fn handle_login_information(error: &str) -> HttpResponse {
    let template = LoginTemplate { error: None, message: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn login_page(error: Option<String>, message: Option<String>) -> impl Responder {
    let template = LoginTemplate { error, message };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}


pub async fn dashboard_page(state: web::Data<AppState>, session: Session, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<String>("user_id") {
        Ok(Some(user_id)) => {
            info!("user_id found in session: {}", user_id);
            match user_id.parse::<i32>() {
                Ok(num) => {
                    debug!("Converted user_id to number: {}", num);
                    let mut connection_guard = state.db_connection.lock().map_err(|e| {
                        error!("Failed to lock database connection: {:?}", e);
                        actix_web::error::ErrorInternalServerError("Database error")
                    })?;

                    match get_a_user_by_id(&mut *connection_guard, num) {
                        Some(user) => {
                            let dashboard_template = DashboardTemplate {
                                email: user.email.clone(),
                            };
                            Ok(HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().map_err(|e| {
                                error!("Template rendering error: {:?}", e);
                                actix_web::error::ErrorInternalServerError("Template error")
                            })?))
                        }
                        None => {
                            info!("User not found in database");
                            Ok(HttpResponse::Found()
                                .append_header((actix_web::http::header::LOCATION, "/login"))
                                .finish())
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to parse user_id: {}", e);
                    Ok(HttpResponse::Found()
                        .append_header((actix_web::http::header::LOCATION, "/login"))
                        .finish())
                },
            }
        },
        Ok(None) => {
            info!("No user_id found in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        },
        Err(e) => {
            error!("Session error: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    };

    result.map_err(|e| {
        error!("Unexpected error in dashboard_page: {:?}", e);
        e
    })
}

pub async fn login_user(form: web::Form<LoginForm>, state: web::Data<AppState>, session: Session) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_a_user_by_mail(&mut *connection_guard, form.email.clone());
    match user_exist {
        Some(user) => {
            if verify(&form.password, &user.password).unwrap_or(false) {
                session.insert("user_id", form.email.clone())?;
                // Redirect to the dashboard route
                Ok(HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                    .finish())
            } else {
                let error_message = "Wrong password.".to_string();
                let template = LoginTemplate { error: Some(error_message), message: None };
                Ok(HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template.render().unwrap()))
            }
        }
        None => {
            let error_message = "Email not found".to_string();
            let template = LoginTemplate { error: Some(error_message), message: None };
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap()))
        }
    }
}

pub async fn register_user(item: web::Form<RegisterForm>, state: web::Data<AppState>) -> HttpResponse {
    println!("Data is {:#?}", item);
    if item.name.is_empty() || item.email.is_empty() || item.bio.is_empty() || item.password.is_empty() {
        println!("Empty fields detected");
        return handle_register_error("All fields are required").await;
    }

    println!("All fields have content");
    // Hash and salt the password
    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(er) => {
            println!("error {}", er);
            return handle_register_error("error hashing password").await;
        }
    };


    let new_user = NewUser {
        name: item.name.clone(),
        email: item.email.clone(),
        bio: item.bio.clone(),
        password: hashed_password,
        avatar_url: "".to_string(),
        blocked_reason:Some("".to_string())
    };

    let mut connection_guard = state.db_connection.lock().unwrap();
    let res = add_user(new_user, &mut *connection_guard);

    match res {
        Ok(user) => {

            return handle_login_information("Account created, please login to continue").await;
        }
        Err(err) => {
            println!("db error {:#?}", err);
            return handle_register_error("error creating account").await;
        }
    }
}


async fn update_user(item: web::Json<Users>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}