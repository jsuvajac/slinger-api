#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


pub mod models;
pub mod schema;

use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, username: &'a str, passwd: &'a str) -> User {
    use schema::users;

    let new_user = NewPost {
        username: username,
        passwd: passwd,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

async fn status() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hey")]
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use schema::users::dsl::*;

    let connection = establish_connection();

    let user = create_post(&connection, "user", "pass");
    println!("\nSaved user with id {}", user.id);

    let results = users.limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for post in results {
        println!("{}", post.username);
        println!("----------\n");
        println!("{}", post.passwd);
    }

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
            .service(hey)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
