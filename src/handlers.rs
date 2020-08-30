use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde_json;

use crate::errors::AppError;
use crate::models::*;
use crate::Pool;

// TODO: hashing
// TODO: field validation

/// Authentication
// Handler for POST /login
pub async fn login(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, AppError> {
    let connection = db.get().unwrap();
    let target = crate::db::get_user_by_email(&connection, &item.email);

    // check if user exists
    if target.passwd == item.passwd {
        session.set("user", &item.email).unwrap();
        log::debug!("{:} -- login", &item.email);
        Ok(HttpResponse::Ok().body("Logged in"))
    } else {
        Err(AppError::AuthenticationError(String::from(
            "Invalid user name of password",
        )))
    }
}

// Handler for POST /logout
pub async fn logout(session: Session) -> Result<impl Responder, AppError> {
    match validate_session(&session) {
        Ok(user_email) => {
            // Clear session and cookie
            log::debug!("{} -- logout", user_email);
            session.purge();
            Ok(HttpResponse::Ok().body("Logged out"))
        }
        Err(e) => Err(e),
    }
}

// checks if the session is in session storage
pub fn validate_session(session: &Session) -> Result<String, AppError> {
    let msg = "Not Logged in";
    let user_id = session
        .get::<String>("user")
        .map_err(|_| AppError::AuthenticationError(String::from(msg)))
        .unwrap();
    match user_id {
        Some(id) => {
            // log::debug!("session {} renewed", id);
            session.renew();
            Ok(id)
        }
        None => Err(AppError::AuthenticationError(String::from(msg))),
    }
}

/// User

// Handler for POST /user
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<impl Responder, AppError> {
    log::debug!("add_user triggered");
    let connection = db.get().unwrap();
    // TODO: Validate the given data
    crate::db::create_user(&connection, &item.passwd, &item.email);
    Ok(HttpResponse::Ok().body("User created"))
}

// Handler for POST /user
pub async fn update_user(
    db: web::Data<Pool>,
    item: web::Json<UpdateUser>,
    session: Session,
) -> Result<impl Responder, AppError> {
    match validate_session(&session) {
        Ok(user_email) => {
            // TODO: Validate the given data
            let connection = db.get().unwrap();
            crate::db::update_user(&connection, &item.passwd, &user_email);
            log::debug!("Updated user: {}", user_email);
            Ok(HttpResponse::Ok().body("Update user"))
        }
        Err(e) => Err(e),
    }
}

// Handler for DELETE /user
pub async fn delete_user(
    db: web::Data<Pool>,
    session: Session,
) -> Result<impl Responder, AppError> {
    match validate_session(&session) {
        Ok(user_email) => {
            // Clear session and cookie
            session.purge();
            let connection = db.get().unwrap();
            crate::db::delete_user(&connection, &user_email);
            log::debug!("Deleted user: {}", user_email);
            Ok(HttpResponse::Ok().body("Deleted user"))
        }
        Err(e) => Err(e),
    }
}

// Handler for PUT /spellbook
pub async fn add_spell_book(
    db: web::Data<Pool>,
    item: web::Json<InputSpellBook>,
    session: Session,
) -> Result<impl Responder, AppError> {
    match validate_session(&session) {
        Ok(user_email) => {
            let connection = db.get().unwrap();
            let user = crate::db::get_user_by_email(&connection, &user_email);
            crate::db::create_spell_book(&connection, &user.id, &item.name, &item.content);
            // TODO: handle dup spell books
            log::debug!("created new spell book {:?}", item);
            Ok(HttpResponse::Ok().body("Created spell book"))
        }
        Err(e) => Err(e),
    }
}

// GET /spellbook
pub async fn get_spell_book(
    db: web::Data<Pool>,
    session: Session,
) -> Result<impl Responder, AppError> {
    match validate_session(&session) {
        Ok(user_email) => {
            let connection = db.get().unwrap();
            let user = crate::db::get_user_by_email(&connection, &user_email);
            match crate::db::get_spell_book(&connection, &user.id) {
                Ok(books) => {
                    let json = serde_json::to_string(&books).unwrap();
                    log::info!("got {:}", json);
                    Ok(HttpResponse::Ok().body(json))
                }
                Err(_) => Err(AppError::NoDataFound(String::from("no Spellbook found"))),
            }
        }
        Err(e) => Err(e),
    }
}
