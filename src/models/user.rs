use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}