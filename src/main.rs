#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate log;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenv::dotenv;

pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;
pub mod schema;

// type alias to reduce verbosity
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // create db connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool.");

    // logging setup
    std::env::set_var("RUST_LOG", std::env::var("RUST_LOG").expect("RUST_LOG"));
    env_logger::init();

    // ssl setup
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

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
    .bind_openssl("127.0.0.1:4000", builder)?
    .run()
    .await
}
