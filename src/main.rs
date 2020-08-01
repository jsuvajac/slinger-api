#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};

pub mod handlers;
pub mod db;
pub mod models;
pub mod schema;

use db::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let connection = establish_connection();
    display_db(&connection);
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::get_users))
            .route("/user", web::get().to(handlers::get_users))
            .route("/user", web::put().to(handlers::add_user))
            .route("/user", web::delete().to(handlers::delete_user))
            .route("/user", web::post().to(handlers::update_user))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
