use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


use crate::schema;
use crate::models::*;
use schema::users::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, usr: &'a str, pass: &'a str, mail: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        username: usr,
        passwd: pass,
        email: mail,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn display_db(conn: &PgConnection) -> String{
    let results = users.limit(5)
        .load::<User>(conn)
        .expect("Error loading users");

    let mut out = format!("Displaying {} users", results.len());
    for user in results {
        out = format!("{}\n{}\n{}\n{}\n----------\n",
                     out,
                     user.username,
                     user.passwd,
                     user.email);
    }
    out
}

