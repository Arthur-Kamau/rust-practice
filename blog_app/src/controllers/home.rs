use actix_web::{HttpResponse, Responder};
use crate::models::ui::{IndexTemplate};
use askama::Template;


pub async fn default_handler() -> impl Responder {
    let template = IndexTemplate ;

    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}