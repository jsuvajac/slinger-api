use crate::models::*;
use crate::Pool;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse, Responder};

// Handler for POST /login
pub async fn login(item: web::Json<InputUser>, session: Session) -> Result<impl Responder, Error> {
    log::debug!("{:?}", item);

    log::info!(
        "SESSION value (logout): {:?}",
        Some(session.get::<String>("user"))
    );

    // check if user exists
    session.set("user", &item.email).unwrap();

    Ok(format!("login\n"))
}

// Handler for POST /logout
pub async fn logout(session: Session) -> Result<impl Responder, Error> {
    log::info!(
        "SESSION value (logout): {:?}",
        Some(session.get::<String>("user"))
    );

    session.clear();

    Ok(format!("logout\n"))
}

// Handler for GET /user
pub async fn get_users(db: web::Data<Pool>, session: Session) -> Result<impl Responder, Error> {
    log::info!(
        "SESSION value (get user): {:?}",
        Some(session.get::<String>("user"))
    );
    let connection = db.get().unwrap();
    let out = crate::db::display_db(&connection);
    log::debug!("get_users triggered");
    Ok(HttpResponse::Ok().body(out))
}

// Handler for POST /user
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<impl Responder, Error> {

    let connection = db.get().unwrap();
    crate::db::create_user(&connection, &item.passwd, &item.email);
    Ok(format!("Created user: {:?}\n", item))
}

// Handler for POST /user
pub async fn update_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, Error> {
    log::info!(
        "SESSION value (update user passwd): {:?}",
        Some(session.get::<String>("user"))
    );
    let connection = db.get().unwrap();
    crate::db::update_user(&connection, &item.passwd, &item.email);
    log::debug!("Updated user: {:?}", item);
    Ok(format!("Update user: {:?}\n", item))
}

// Handler for DELETE /user
pub async fn delete_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
    session: Session,
) -> Result<impl Responder, Error> {
    log::info!(
        "SESSION value (delete user): {:?}",
        Some(session.get::<String>("user"))
    );

    let connection = db.get().unwrap();
    crate::db::delete_user(&connection, &item.email);
    log::debug!("Deleted user: {:?}", item);
    Ok(format!("Deleted user: {:?}\n", item))
}

// TODO: spell book
