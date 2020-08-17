use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::*;
use crate::schema;
use schema::users::dsl::*;

/// Create user based on email and passwd
pub fn create_user<'a>(conn: &PgConnection, pass: &'a str, mail: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        passwd: pass,
        email: mail,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

/// Update passwd
pub fn update_user<'a>(conn: &PgConnection, new_pass: &'a str, mail: &'a str) -> User {
    let target = users.filter(email.eq(mail));
    diesel::update(target)
        .set(passwd.eq(new_pass))
        .get_result(conn)
        .expect("Error updateing user")
}

/// Delete user from db
pub fn delete_user<'a>(conn: &PgConnection, mail: &'a str) {
    diesel::delete(users.filter(email.like(mail)))
        .execute(conn)
        .expect("Error deleting user");
}

/// Get list of users
pub fn display_db(conn: &PgConnection) -> String {
    let results = users.load::<User>(conn).expect("Error loading users");

    let mut out = format!("Displaying {} users", results.len());
    for user in results {
        out = format!("{}\n{}\n{}\n----------\n", out, user.passwd, user.email);
    }
    out
}
