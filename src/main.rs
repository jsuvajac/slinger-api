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
            .route("/", web::get().to(handlers::status))
            .route("/users", web::get().to(handlers::get_users))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users", web::delete().to(handlers::delete_user))
            .route("/users/update", web::post().to(handlers::update_user))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
