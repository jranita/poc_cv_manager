use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use std::process::Command;

pub mod utils;

#[test]
fn test_login() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("users")
        .arg("create")
        .arg("test_admin4")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", utils::APP_HOST))
        .json(&json!({
            "username": "test_admin", "password": "1234"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().expect("").len(), 128);
}

#[test]
fn test_failed_login() {
    let _output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("12345")
        .arg("admin")
        .output()
        .unwrap();

    let client = Client::new();

    let response = client
        .post(format!("{}/login", utils::APP_HOST))
        .json(&json!({
            "username": "test_admin3", "password": "12345"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_none());
}
