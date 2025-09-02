use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{web,App,HttpServer, Responder};
use serde::{Deserialize,Serialize};
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}
type UserDb = Arc<Mutex<HashMap<u32,User>>>;

#[actix_web::get("/greet/{id}")]
async fn greet(user_id:web::Path<u32>) -> impl Responder {
    format!("Hello {user_id}!")
}

#[actix_web::post("/users")]
async fn create_user(user: web::Json<User>, userdb: web::Data<UserDb>) -> impl Responder {
    let mut db = userdb.lock().unwrap();
    let new_id = db.len() as u32 + 1;
    db.insert(new_id, user.into_inner());
    format!("User created with ID: {}", new_id)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {port}");
    let userdb:UserDb = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move|| {
        let app_data = web::Data::new(userdb.clone());
        App::new().app_data(app_data).service(greet).service(create_user)
    })
    .bind(("127.0.0.1", port))?  
    .workers(2)
    .run()
    .await
}

