use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Value, Json};
use rocket::{get, post, put, delete};
use rocket::serde::json::serde_json::json;

use crate::DbConn;
use crate::models::*;
use crate::repositories::*;

#[get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    RustaceanRepository::find_multiple(c, 100)
    .map(|rustaceans| json!(rustaceans))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    RustaceanRepository::find(c, id)
    .map(|rustacean| json!(rustacean))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    RustaceanRepository::create(c, new_rustacean.into_inner())
    .map(|rustacean| json!(rustacean))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(db: DbConn, id: i32, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    RustaceanRepository::save(c, id, rustacean.into_inner())
    .map(|rustacean| json!(rustacean))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    RustaceanRepository::delete(c, id)
    .map(|n| json!(n))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}
