use crate::models::*;
use crate::Pool;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse, Responder};

// Handler for POST /login
pub async fn login(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, Error> {
    log::debug!("login triggered");
    log::debug!("{:?}", item);

    let connection = db.get().unwrap();
    let target = crate::db::get_user_by_email(&connection, &item.email);
    log::debug!("Match: {:?}", target);

    // check if user exists
    log::info!("stored {:?}", session.set("user", &item.email));

    Ok("login\n")
}

// Handler for POST /logout
pub async fn logout(session: Session) -> Result<impl Responder, Error> {
    log::debug!("logout triggered");
    validate_session(&session).unwrap();

    session.purge();

    Ok("logout\n")
}

// Handler for GET /user
pub async fn get_users(db: web::Data<Pool>, session: Session) -> Result<impl Responder, Error> {
    log::debug!("get_users triggered");
    validate_session(&session).unwrap();
    let connection = db.get().unwrap();
    let out = crate::db::display_db(&connection);
    Ok(HttpResponse::Ok().body(out))
}

// Handler for POST /user
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<impl Responder, Error> {
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
) -> Result<impl Responder, Error> {
    log::debug!("Updated user: {:?}", item);
    validate_session(&session).unwrap();

    let connection = db.get().unwrap();
    crate::db::update_user(&connection, &item.passwd, &item.email);
    Ok(format!("Update user: {:?}\n", item))
}

// Handler for DELETE /user
pub async fn delete_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, Error> {
    log::debug!("Deleted user: {:?}", item);
    validate_session(&session).unwrap();

    let connection = db.get().unwrap();
    crate::db::delete_user(&connection, &item.email);
    Ok(format!("Deleted user: {:?}\n", item))
}

pub fn validate_session(session: &Session) -> Result<String, std::io::Error> {
    let user_id: Option<String> = session.get("user").unwrap_or(None);

    match user_id {
        Some(id) => {
            log::debug!("session {} renewed", id);
            session.renew();
            Ok(id)
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Authentication failed!",
        )),
    }
}

// TODO: spell book
