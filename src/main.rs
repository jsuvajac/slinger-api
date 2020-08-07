#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate env_logger;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web_httpauth::middleware::HttpAuthentication;

use dotenv::dotenv;
use diesel::{
        prelude::*, 
        r2d2::{self, ConnectionManager}
    };

pub mod handlers;
pub mod db;
pub mod models;
pub mod schema;
pub mod auth;

// type alias to reduce verbosity
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool.");

    std::env::set_var("RUST_LOG", std::env::var("RUST_LOG").expect("RUST_LOG"));
    env_logger::init();

    log::info!("starting server...");
    // Start Server
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::bearer_auth_validator);
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(auth)
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
