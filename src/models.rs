use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub passwd: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InputUser {
    pub username: String,
    pub passwd: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub email: &'a str,
}