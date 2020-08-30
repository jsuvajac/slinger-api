use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::*;
use crate::schema;
use schema::users::dsl::*;
use schema::spell_book::dsl::*;

// TODO: replace email with uuid
// TODO: setup sessions based on uuid

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
        .set((passwd.eq(new_pass), updated_at.eq(&Utc::now().naive_utc())))
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
pub fn get_user_by_email<'a>(conn: &PgConnection, mail: &'a str) -> User {
    users
        .filter(email.eq(mail))
        .get_result(conn)
        .expect("Error finding specific user")
}

/// Get list of users -- for debuging
pub fn display_db(conn: &PgConnection) -> String {
    let results = users.load::<User>(conn).expect("Error loading users");

    let mut out = format!("num users: {}\n", results.len());
    for user in results {
        out = format!("{}{:?}\n", out, user);
    }
    out
}

/// Create user based on email and passwd
pub fn create_spell_book<'a>(conn: &PgConnection, uuid: &'a Uuid, book_name: &'a str, book_content: &'a str) -> SpellBook {
    use schema::spell_book;
    let new_user = NewSpellBook {
        user_id: uuid,
        name: book_name,
        content: book_content,
    };

    diesel::insert_into(spell_book::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating spell book")
}

