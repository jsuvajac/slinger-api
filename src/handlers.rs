use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

use crate::errors::AppError;
use crate::models::*;
use crate::Pool;


/// Authentication
// Handler for POST /login
pub async fn login(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, AppError> {
    log::debug!("login triggered");
    log::debug!("{:?}", item);

    let connection = db.get().unwrap();
    let target = crate::db::get_user_by_email(&connection, &item.email);
    log::debug!("Match: {:?}", target);

    // check if user exists
    // TODO: validate passwd
    log::info!("stored {:?}", session.set("user", &item.email));

    Ok("login\n")
}

// Handler for POST /logout
pub async fn logout(session: Session) -> Result<impl Responder, AppError> {
    log::debug!("logout triggered");
    match validate_session(&session) {
        Ok(_) => {
            // Clear session and cookie
            session.purge();
            Ok("logout\n")
        }
        Err(_) => Err(AppError::AuthenticationError(String::from(
            "Could not retrieve user from session",
        ))),
    }
}

// checks if the session is in session storage
pub fn validate_session(session: &Session) -> Result<String, AppError> {
    let msg = "Could not retrieve user from session";
    let user_id = session
        .get::<String>("user")
        .map_err(|_| AppError::AuthenticationError(String::from(msg)))
        .unwrap();
    match user_id {
        Some(id) => {
            log::debug!("session {} renewed", id);
            session.renew();
            Ok(id)
        }
        None => Err(AppError::AuthenticationError(String::from(msg))),
    }
}

/// User
// Handler for GET /user
pub async fn get_users(db: web::Data<Pool>, session: Session) -> Result<impl Responder, AppError> {
    log::debug!("get_users triggered");

    match validate_session(&session) {
        Ok(_) => {
            let connection = db.get().unwrap();
            let out = crate::db::display_db(&connection);
            Ok(HttpResponse::Ok().body(out))
        }
        Err(_) => Err(AppError::AuthenticationError(String::from(
            "Could not retrieve user from session",
        ))),
    }
}

// Handler for POST /user
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<impl Responder, AppError> {
    log::debug!("add_user triggered");
    let connection = db.get().unwrap();
    // TODO: Validate the given data
    crate::db::create_user(&connection, &item.passwd, &item.email);
    Ok(format!("Created user: {:?}\n", item))
}

// Handler for POST /user
pub async fn update_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, AppError> {
    log::debug!("Updated user: {:?}", item);

    match validate_session(&session) {
        Ok(_) => {
            // TODO: Validate the given data
            let connection = db.get().unwrap();
            crate::db::update_user(&connection, &item.passwd, &item.email);
            Ok(format!("Update user: {:?}\n", item))
        }
        Err(_) => Err(AppError::AuthenticationError(String::from(
            "Could not retrieve user from session",
        ))),
    }
}

// Handler for DELETE /user
pub async fn delete_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, AppError> {
    log::debug!("Deleted user: {:?}", item);

    match validate_session(&session) {
        Ok(_) => {
            // Clear session and cookie
            session.purge();
            let connection = db.get().unwrap();
            crate::db::delete_user(&connection, &item.email);
            Ok(format!("Deleted user: {:?}\n", item))
        }
        Err(_) => Err(AppError::AuthenticationError(String::from(
            "Could not retrieve user from session",
        ))),
    }
}


// Handler for PUT /spellbook
pub async fn add_spell_book(
    db: web::Data<Pool>,
    item: web::Json<InputSpellBook>,
    session: Session,
) -> Result<impl Responder, AppError> {
    log::debug!("add_spell_book triggered");

    match validate_session(&session) {
        Ok(user) => {
            log::info!("spell book: {:?}", item);
            log::info!("user form session token: {:?}", user);
            let connection = db.get().unwrap();
            let user = crate::db::get_user_by_email(&connection, &user);
            log::info!("user after lookup: {:?}", user);
            crate::db::create_spell_book(&connection, &user.id, &item.name, &item.content);
            Ok(format!("Created spell book: {:?}\n", item))
        }
        Err(_) => Err(AppError::AuthenticationError(String::from(
            "Could not retrieve user from session",
        ))),
    }
}
