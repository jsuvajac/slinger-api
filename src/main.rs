#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{dev::ServiceRequest, web, App, HttpServer, Error};

use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::middleware::{HttpAuthentication};

use dotenv::dotenv;

pub mod handlers;
pub mod db;
pub mod models;
pub mod schema;

use db::*;


async fn bearer_auth_validator(req: ServiceRequest, auth: BearerAuth) -> Result<ServiceRequest, Error> {
    print!("{}", auth.token());
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match validate_token(auth.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

fn validate_token(token: &str) -> Result<bool, std::io::Error> {
    if token.eq("test-token") {
        return Ok(true);
    }
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let connection = establish_connection();
    display_db(&connection);

    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(bearer_auth_validator);
        App::new()
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
