use actix_web::{web, Responder, HttpResponse};
use crate::models::*;


/// User db actions

// Handler for GET /user
pub async fn get_users() -> impl Responder {
    let connection = crate::db::establish_connection();
    let out = crate::db::display_db(&connection);
    
    HttpResponse::Ok().body(out)
}

// Handler for POST /user
pub async fn add_user(item: web::Json<InputUser>) -> impl Responder {
    let connection = crate::db::establish_connection();
    crate::db::create_user(&connection, &item.passwd, &item.email);
    format!("Created user: {:?}\n", item)
    // format!("add user")
}
// Handler for POST /user
pub async fn update_user(item: web::Json<InputUser>) -> impl Responder {
    let connection = crate::db::establish_connection();
    crate::db::update_user(&connection, &item.passwd, &item.email);
    format!("Update user: {:?}\n", item)
}

// Handler for DELETE /user
pub async fn delete_user(item: web::Json<InputUser>) -> impl Responder {
    let connection = crate::db::establish_connection();
    crate::db::delete_user(&connection, &item.email);
    format!("Deleted user: {:?}\n", item)
}