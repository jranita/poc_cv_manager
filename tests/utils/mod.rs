use std::process::Command;

use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{self, HeaderMap, HeaderValue},
};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:5800";

pub fn create_fixture_company(client: &Client) -> Value {
    let response = client
        .post(format!("{}/api/clients", APP_HOST))
        .json(&json!({
          "company_name": "string"
        }))
        .send()
        .unwrap();

    response.json().unwrap()
}

pub fn delete_fixture_company(client: &Client, object: Value) {
    client
        .delete(format!("{}/api/clients/{}", APP_HOST, object["id"]))
        .send()
        .unwrap();
}

pub fn create_fixture_company(client: &Client) -> Value {
    let response = client
        .post(format!("{}/api/clients", APP_HOST))
        .json(&json!({
          "company_name": "string"
        }))
        .send()
        .unwrap();

    response.json().unwrap()
}

pub fn delete_fixture_company(client: &Client, object: Value) {
    client
        .delete(format!("{}/api/clients/{}", APP_HOST, object["id"]))
        .send()
        .unwrap();
}

// fn get_logged_in_client(username: &str, role: &str) -> Client {

//     let output = Command::new("cargo")
//         .arg("run")
//         .arg("--bin")
//         .arg("cli")
//         .arg("users")
//         .arg("create")
//         .arg(username)
//         .arg("1234")
//         .arg(role)
//         .output()
//         .unwrap();

//     println!("{:?}", output);

//     let client = Client::new();

//     let response = client
//         .post(format!("{}/login", APP_HOST))
//         .json(&json!({
//             "username": username, "password": "1234"
//         }))
//         .send()
//         .unwrap();

//     assert_eq!(response.status(), reqwest::StatusCode::OK);
//     let json: Value = response.json().unwrap();
//     assert!(json.get("token").is_some());

//     let header_value = format!("Bearer {}", json["token"].as_str().unwrap());
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         header::AUTHORIZATION,
//         HeaderValue::from_str(&header_value).unwrap(),
//     );
//     ClientBuilder::new()
//         .default_headers(headers)
//         .build()
//         .unwrap()
// }

// pub fn get_client_with_admin_role() -> Client {
//     get_logged_in_client("test_admin", "admin")
// }

// pub fn get_client_with_viewer_role() -> Client {
//     get_logged_in_client("test_viewer", "viewer")
// }
