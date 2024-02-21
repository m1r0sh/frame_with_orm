use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}