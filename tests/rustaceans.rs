use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::utils::APP_HOST;

pub mod utils;

#[test]
fn test_get_rustaceans() {
    let client = utils::get_client_with_admin_role();

    let response0 = utils::create_fixture_rustacean(&client);
    let response1 = utils::create_fixture_rustacean(&client);

    let client = utils::get_client_with_viewer_role();

    let response = client
        .get(format!("{}/rustaceans", APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json_response: Value = response.json().unwrap();
    assert!(json_response.as_array().unwrap().contains(&response0));
    assert!(json_response.as_array().unwrap().contains(&response1));

    let client = utils::get_client_with_admin_role();
    utils::delete_fixture_rustacean(&client, response0);
    utils::delete_fixture_rustacean(&client, response1);
}

#[test]
fn test_create_rustaceans() {
    let client = utils::get_client_with_admin_role();
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "JJJ", "email": "jjj@hotmail.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let json_response: Value = response.json().unwrap();
    assert_eq!(
        json!({
            "id": json_response["id"],
            "name": "JJJ",
            "email": "jjj@hotmail.com",
            "created_at": json_response["created_at"]
        }),
        json_response
    );

    utils::delete_fixture_rustacean(&client, json_response);
}

#[test]
fn test_view_rustaceans() {
    let client = utils::get_client_with_admin_role();

    let json_response: Value = utils::create_fixture_rustacean(&client);

    let client = utils::get_client_with_viewer_role();
    let view_response: Value = client
        .get(format!("{}/rustaceans/{}", APP_HOST, json_response["id"]))
        .send()
        .unwrap()
        .json()
        .unwrap();

    assert_eq!(
        json!({
            "id": json_response["id"],
            "name": "JJJ",
            "email": "jjj@hotmail.com",
            "created_at": json_response["created_at"]
        }),
        view_response
    );

    assert_eq!(json_response, view_response);

    utils::delete_fixture_rustacean(&client, json_response);
}

#[test]
fn test_update_rustaceans() {
    let client = utils::get_client_with_admin_role();

    let json_response: Value = utils::create_fixture_rustacean(&client);
    let update_response: Value = client
        .put(format!("{}/rustaceans/{}", APP_HOST, json_response["id"]))
        .json(&json!({
            "name": "MMM", "email": "mmm@hotmail.com"
        }))
        .send()
        .unwrap()
        .json()
        .unwrap();

    assert_eq!(
        json!({
            "id": json_response["id"],
            "name": "MMM",
            "email": "mmm@hotmail.com",
            "created_at": json_response["created_at"]
        }),
        update_response
    );

    utils::delete_fixture_rustacean(&client, json_response);
}

#[test]
fn test_delete_rustaceans() {
    let client = utils::get_client_with_admin_role();

    let json_response: Value = utils::create_fixture_rustacean(&client);
    let delete_response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, json_response["id"]))
        .send()
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}
