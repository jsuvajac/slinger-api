use actix_web::{web, Responder, HttpResponse};

use crate::models::*;
use crate::Pool;

/// User db actions

// Handler for GET /user
pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    let connection = db.get().unwrap();
    let out = crate::db::display_db(&connection);
    log::error!("test info log in get_users");
    HttpResponse::Ok().body(out)
}

// Handler for POST /user
pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> impl Responder {
    let connection = db.get().unwrap();
    crate::db::create_user(&connection, &item.passwd, &item.email);
    format!("Created user: {:?}\n", item)
}
// Handler for POST /user
pub async fn update_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> impl Responder {
    let connection = db.get().unwrap();
    crate::db::update_user(&connection, &item.passwd, &item.email);
    format!("Update user: {:?}\n", item)
}

// Handler for DELETE /user
pub async fn delete_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> impl Responder {
    let connection = db.get().unwrap();
    crate::db::delete_user(&connection, &item.email);
    format!("Deleted user: {:?}\n", item)
}