use actix_web::{Responder, HttpResponse};


// Handler for GET / for debugging
pub async fn status() -> impl Responder {
    let connection = crate::db::establish_connection();
    let out = crate::db::display_db(&connection);
    
    HttpResponse::Ok().body(out)
}

// Handler for GET /users
pub async fn get_users() -> impl Responder {
    format!("get users")
}
// Handler for GET /users
pub async fn get_user_by_id() -> impl Responder {
    format!("get users by id")
}
// Handler for POST /users
pub async fn add_user() -> impl Responder {
    format!("add user")
}
// Handler for POST /users
pub async fn update_user() -> impl Responder {
    format!("update user")
}

// Handler for DELETE /users
pub async fn delete_user() -> impl Responder {
    format!("delete user")
}