use actix_session::Session;
use actix_web::{Error, HttpResponse, Responder};
use log::{error, info};
use crate::models::ui::DashboardTemplate;
use askama::Template;

pub async fn dashboard_page(session: Session) -> Result<HttpResponse, Error> {


    let result =  session.get::<String>("user_email");
    match result {
        Ok(data)=>{
             match data {
                 Some(user_email)=>{
                     info!("user email found  {}", user_email);
                     let dashboard_template = DashboardTemplate {
                         email: user_email.clone(),
                     };
                     Ok(
                         HttpResponse::Ok().content_type("text/html").
                             body(
                                 dashboard_template.render().map_err(|e| {
                                     error!("Template rendering error: {:?}", e);
                                     actix_web::error::ErrorInternalServerError("Template error")
                                 }
                                 )?
                             )
                     )
                 }
                 None=>{
                     error!("Failed to get email ");
                     Ok(HttpResponse::Found()
                         .append_header((actix_web::http::header::LOCATION, "/login"))
                         .finish())
                 }
             }
        }
        Err(error)=>{
            error!("Failed to parse {:#?}", error);
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        }
    }
    // if let Some(user_email) = session.get::<String>("user_email")? {
    //
    //     info!("user email found  {}", user_email);
    //     let dashboard_template = DashboardTemplate {
    //         email: user_email.clone(),
    //     };
    //     Ok(
    //         HttpResponse::Ok().content_type("text/html").
    //             body(
    //                 dashboard_template.render().map_err(|e| {
    //                     error!("Template rendering error: {:?}", e);
    //                     actix_web::error::ErrorInternalServerError("Template error")
    //                 }
    //                 )?
    //             )
    //     )
    // }else{
    //     error!("Failed to parse email");
    //     Ok(HttpResponse::Found()
    //         .append_header((actix_web::http::header::LOCATION, "/login"))
    //         .finish())
    // }


}
