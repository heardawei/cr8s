use rocket::serde::json::{Value, serde_json::json};

#[test]
fn test_get_rustaceans()
{
  let client = reqwest::blocking::Client::new();
  let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
  assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[test]
fn test_create_rustacean()
{
  let new_rustacean = json!({
    "name": "test_create_rustacean",
    "email": "test_create_rustacean@qq.com",
  });
  let client = reqwest::blocking::Client::new();
  let response = client.post("http://127.0.0.1:8000/rustacean")
                           .json(&new_rustacean)
                           .send().unwrap();
  assert_eq!(response.status(), reqwest::StatusCode::CREATED);
  let rustacean: Value = response.json().unwrap();
  assert_eq!(rustacean, json!({
    "id": rustacean["id"],
    "name": new_rustacean["name"],
    "email": new_rustacean["email"],
    "create_at": rustacean["create_at"],
  }));
}

#[test]
fn test_update_rustacean()
{
  let new_rustacean = json!({
    "name": "test_update_rustacean",
    "email": "test_update_rustacean@qq.com",
  });
  let client = reqwest::blocking::Client::new();
  let response = client.post("http://127.0.0.1:8000/rustacean")
                           .json(&new_rustacean)
                           .send().unwrap();
  assert_eq!(response.status(), reqwest::StatusCode::CREATED);
  let mut rustacean: Value = response.json().unwrap();
  assert_eq!(rustacean, json!({
    "id": rustacean["id"],
    "name": new_rustacean["name"],
    "email": new_rustacean["email"],
    "create_at": rustacean["create_at"],
  }));

  rustacean["name"] = format!("{}_updated", new_rustacean["name"]).into();
  let response = client.put(format!("http://127.0.0.1:8000/rustacean/{}", 
                                                   rustacean["id"]))
                                 .json(&rustacean)
                                 .send()
                                 .unwrap();
  assert_eq!(response.status(), reqwest::StatusCode::OK);
  let updated_rustacean: Value = response.json().unwrap();
  assert_eq!(updated_rustacean, rustacean);
}