use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub passwd: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub passwd: &'a str,
    pub email: &'a str,
}

// for incoming requesets json storage
#[derive(Deserialize, Serialize, Debug)]
pub struct InputUser {
    pub passwd: String,
    pub email: String,
}
