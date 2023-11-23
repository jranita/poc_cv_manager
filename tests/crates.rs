use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

use crate::utils::{
    create_fixture_crate, create_fixture_rustacean, delete_fixture_crate, delete_fixture_rustacean,
    APP_HOST,
};

pub mod utils;

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean = create_fixture_rustacean(&client);

    let crate0 = create_fixture_crate(&client, &rustacean);
    let crate1 = create_fixture_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json_response: Value = response.json().unwrap();
    assert!(json_response.as_array().unwrap().contains(&crate0));
    assert!(json_response.as_array().unwrap().contains(&crate1));

    delete_fixture_rustacean(&client, rustacean);
    delete_fixture_crate(&client, crate0);
    delete_fixture_crate(&client, crate1);
}

#[test]
fn test_create_fixture_crates() {
    let client = Client::new();
    let rustacean = create_fixture_rustacean(&client);

    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustaceans_id": rustacean["id"],
            "code": "X += 1",
            "name": "A Crate",
            "version": "0.1",
            "description": "A test Crate description",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let json_response: Value = response.json().unwrap();
    assert_eq!(
        json!({
            "id": json_response["id"],
            "rustaceans_id": rustacean["id"],
            "code": "X += 1",
            "name": "A Crate",
            "version": "0.1",
            "description": "A test Crate description",
            "created_at": json_response["created_at"]
        }),
        json_response
    );

    utils::delete_fixture_crate(&client, json_response);
    utils::delete_fixture_rustacean(&client, rustacean);
}

#[test]
fn test_view_crates() {
    let client = Client::new();

    let rustacean = create_fixture_rustacean(&client);

    let a_crate = utils::create_fixture_crate(&client, &rustacean);
    let view_response: Value = client
        .get(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap()
        .json()
        .unwrap();

    assert_eq!(
        json!({
            "id": a_crate["id"],
            "rustaceans_id": rustacean["id"],
            "code": "X += 1",
            "name": "A Crate",
            "version": "0.1",
            "description": "A test Crate description",
            "created_at": a_crate["created_at"]
        }),
        view_response
    );

    assert_eq!(a_crate, view_response);

    utils::delete_fixture_crate(&client, a_crate);
    utils::delete_fixture_rustacean(&client, rustacean);
}

#[test]
fn test_update_crates() {
    let client = Client::new();

    let rustacean = create_fixture_rustacean(&client);

    let a_crate = utils::create_fixture_crate(&client, &rustacean);
    let update_response: Value = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustaceans_id": a_crate["rustaceans_id"] ,"code":"X *= 3", "name": "MMM", "version": "0.3","description": a_crate["description"],
        }))
        .send()
        .unwrap()
        .json()
        .unwrap();

    assert_eq!(
        json!({
            "code": "X *= 3",
            "created_at":  a_crate["created_at"],
            "description": "A test Crate description",
            "id":  a_crate["id"],
            "name": "MMM",
            "rustaceans_id": a_crate["rustaceans_id"],
            "version": "0.3",
        }),
        update_response
    );

    utils::delete_fixture_crate(&client, a_crate);
    utils::delete_fixture_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crates() {
    let client = Client::new();

    let rustacean = create_fixture_rustacean(&client);
    let a_crate = create_fixture_crate(&client, &rustacean);

    let delete_response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    utils::delete_fixture_rustacean(&client, rustacean);
}
