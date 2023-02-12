use diesel::prelude::*;

#[derive(Queryable)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub rating: f64,
}