use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::spell_book;
use crate::schema::users;

/// User
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

#[derive(Deserialize, Serialize, Debug)]
pub struct InputUser {
    pub passwd: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUser {
    pub passwd: String,
}

/// Spell Book

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct SpellBook {
    #[serde(skip_serializing)]
    pub user_id: Uuid,
    pub name: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "spell_book"]
pub struct NewSpellBook<'a> {
    pub user_id: &'a Uuid,
    pub name: &'a str,
    pub content: &'a str,
}

// for incoming requesets json storage
#[derive(Deserialize, Serialize, Debug)]
pub struct InputSpellBook {
    pub name: String,
    pub content: String,
}

// for incoming requesets json storage
#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteSpellBook {
    pub name: String,
}
