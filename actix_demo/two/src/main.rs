use std::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MyStruct {
    id: usize,
    name: String,
    // Add other fields as needed
}

#[derive(Debug)]
struct AppState {
    items: Mutex<Vec<MyStruct>>,
}

async fn get_all_items(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.items)
}

async fn add_item(item: web::Json<MyStruct>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Ok().finish()
}

async fn delete_item(path: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(index) = items.iter().position(|item| item.id == *path) {
        items.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_item(item: web::Json<MyStruct>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(existing_item) = items.iter_mut().find(|it| it.id == item.id) {
        *existing_item = item.into_inner();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state
    let app_state = web::Data::new(AppState { items:  Mutex::new(Vec::new()) });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/items", web::get().to(get_all_items))
            .route("/items", web::post().to(add_item))
            .route("/items/{id}", web::delete().to(delete_item))
            .route("/items", web::put().to(update_item))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}






