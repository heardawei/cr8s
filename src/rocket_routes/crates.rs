use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{delete, get, post, put};

use crate::models::NewCrate;
use crate::repositories::CrateRepository;
use crate::DbConn;

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[get("/crate/<id>")]
pub async fn view_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|crt| json!(crt))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[post("/crate", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    db: DbConn,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|crt| Custom(Status::Created, json!(crt)))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[put("/crate/<id>", format = "json", data = "<new_crate>")]
pub async fn update_crate(
    db: DbConn,
    id: i32,
    new_crate: Json<NewCrate>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::save(c, id, new_crate.into_inner())
            .map(|c| json!(c))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[delete("/crate/<id>")]
pub async fn delete_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|n| json!(n))
            .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}
