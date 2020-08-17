use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub passwd: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InputUser {
    pub passwd: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub passwd: &'a str,
    pub email: &'a str,
}
