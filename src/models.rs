use crate::schema::*;

#[derive(diesel::Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub create_at: chrono::NaiveDateTime,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(diesel::Queryable, diesel::Associations)]
#[diesel(belongs_to(Rustacean))]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub create_at: chrono::NaiveDateTime,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
