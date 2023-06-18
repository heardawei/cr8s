use reqwest::Error;
use rocket::serde::json::{serde_json::json, Value};

enum MyError {
    ReqwestError(reqwest::Error),
    JsonError,
}

impl From<reqwest::Error> for MyError {
    fn from(e: reqwest::Error) -> Self {
        MyError::ReqwestError(e)
    }
}

fn create_rustacean(name: &str, email: &str) -> Result<Value, MyError> {
    let new_rustacean = json!({
      "name": name,
      "email": email,
    });
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustacean")
        .json(&new_rustacean)
        .send()?
        .error_for_status()?;

    let rustacean = response.json::<Value>()?;

    let expected_res = json!({
      "id": rustacean["id"],
      "name": new_rustacean["name"],
      "email": new_rustacean["email"],
      "create_at": rustacean["create_at"],
    });

    if rustacean == expected_res {
        Ok(rustacean)
    } else {
        Err(MyError::JsonError)
    }
}

#[test]
fn test_get_rustaceans() {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[test]
fn test_create_rustacean() {
    let res = create_rustacean("test_create_rustacean", "test_create_rustacean@qq.com");
    assert!(res.is_ok());
    // let new_rustacean = json!({
    //   "name": "test_create_rustacean",
    //   "email": "test_create_rustacean@qq.com",
    // });
    // let client = reqwest::blocking::Client::new();
    // let response = client
    //     .post("http://127.0.0.1:8000/rustacean")
    //     .json(&new_rustacean)
    //     .send()
    //     .unwrap();
    // assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    // let rustacean: Value = response.json().unwrap();
    // assert_eq!(
    //     rustacean,
    //     json!({
    //       "id": rustacean["id"],
    //       "name": new_rustacean["name"],
    //       "email": new_rustacean["email"],
    //       "create_at": rustacean["create_at"],
    //     })
    // );
}

#[test]
fn test_update_rustacean() {
    let new_rustacean = json!({
      "name": "test_update_rustacean",
      "email": "test_update_rustacean@qq.com",
    });
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustacean")
        .json(&new_rustacean)
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    let mut rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
          "id": rustacean["id"],
          "name": new_rustacean["name"],
          "email": new_rustacean["email"],
          "create_at": rustacean["create_at"],
        })
    );

    rustacean["name"] = format!("{}_updated", new_rustacean["name"]).into();
    let response = client
        .put(format!(
            "http://127.0.0.1:8000/rustacean/{}",
            rustacean["id"]
        ))
        .json(&rustacean)
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let updated_rustacean: Value = response.json().unwrap();
    assert_eq!(updated_rustacean, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let new_rustacean = json!({
      "name": "test_update_rustacean",
      "email": "test_update_rustacean@qq.com",
    });
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustacean")
        .json(&new_rustacean)
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    let mut rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
          "id": rustacean["id"],
          "name": new_rustacean["name"],
          "email": new_rustacean["email"],
          "create_at": rustacean["create_at"],
        })
    );

    rustacean["name"] = format!("{}_updated", new_rustacean["name"]).into();
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustacean/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let updated_rustacean: Value = response.json().unwrap();
    assert_eq!(updated_rustacean, 1);
}
