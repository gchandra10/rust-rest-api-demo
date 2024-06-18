use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

async fn home_page() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

async fn get_item(item_id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    let item = items.iter().find(|&item| item.id == *item_id);
    match item {
        Some(item) => HttpResponse::Ok().json(item),
        None => {
            let mut response = HashMap::new();
            response.insert("message", "Item not found");
            HttpResponse::NotFound().json(response)
        },
    }
}

async fn create_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let new_item = Item {
        id: items.len() as u32 + 1,
        name: item.name.clone(),
    };
    items.push(new_item.clone());
    HttpResponse::Created().json(new_item)
}

async fn update_item(item_id: web::Path<u32>, item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if let Some(existing) = items.iter_mut().find(|item| item.id == *item_id) {
        existing.name = item.name.clone();
        return HttpResponse::Ok().json(existing);
    }
    let mut response = HashMap::new();
    response.insert("message", "Item not found");
    HttpResponse::NotFound().json(response)
}

async fn delete_item(item_id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if items.iter().any(|item| item.id == *item_id) {
        items.retain(|item| item.id != *item_id);
        let mut response = HashMap::new();
        response.insert("message", "Item deleted");
        HttpResponse::Ok().json(response)
    } else {
        let mut response = HashMap::new();
        response.insert("message", "Item not found");
        HttpResponse::NotFound().json(response)
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        items: Mutex::new(vec![
            Item { id: 1, name: "Item 1".to_string() },
            Item { id: 2, name: "Item 2".to_string() },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(home_page))
            .route("/items", web::get().to(get_items))
            .route("/items/{item_id}", web::get().to(get_item))
            .route("/items", web::post().to(create_item))
            .route("/items/{item_id}", web::put().to(update_item))
            .route("/items/{item_id}", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:5002")?
    .run()
    .await
}
