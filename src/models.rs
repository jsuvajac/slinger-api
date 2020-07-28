use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub passwd: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewPost<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
}