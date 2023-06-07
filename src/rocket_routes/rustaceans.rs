use rocket::serde::json::Value;
use rocket::{get, post, put, delete};
use rocket::serde::json::serde_json::json;

#[get("/rustaceans")]
pub fn get_rustaceans() -> Value {
  json!([])
}

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {
  
}

#[post("/rustaceans")]
pub fn create_rustacean() {

}

#[put("/rustaceans/<id>")]
pub fn update_rustacean(id: i32) {
  
}

#[delete("/rustaceans/<id>")]
pub fn delete_rustacean(id: i32) {
  
}
