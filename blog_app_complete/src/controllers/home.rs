use actix_web::{HttpResponse,HttpRequest, Responder};
use crate::models::ui::{HomeTemplate};
use askama::Template;

// pub async fn default_handler() -> impl Responder {
//     let template = HomeTemplate { };
//
//     // HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
//     HttpResponse::Ok().json()
// }

pub async fn default_handler(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}