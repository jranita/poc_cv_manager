use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::http::StatusError;
use salvo::writing::Json;
use salvo::{endpoint, oapi::extract::*};
use tokio::sync::Mutex;

use crate::models::user::NewUser;
use crate::models::user::User;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<User>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List users.
#[endpoint(
    tags("users"),
    parameters(
        ("offset", description = "Offset is an optional query parameter."),
        ("limit", description = "Limit is an optional query parameter."),
    )
)]
pub async fn list_users(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Result<Json<Vec<User>>, salvo::Error> {
    println!("67     list_users()");
    let users_list = STORE.lock().await;

    let users_list: Vec<User> = User::get_users().await?;

    std::result::Result::Ok(Json(users_list))
}

/// User by ID.
#[endpoint(
    tags("users"),
    status_codes(200, 500),
    parameters(
        ("id", description = "Database ID for the User"),
    )
)]
pub async fn get_user_by_id(id: QueryParam<i32, true>) -> Result<Json<User>, salvo::Error> {
    tracing::debug!(id = ?id, "get User");
    let mut user = STORE.lock().await;

    let target_user: User = User::get_user(id.into_inner()).await?;

    user.push(target_user.clone());

    std::result::Result::Ok(Json(target_user))
}

/// Create new user.
#[endpoint(tags("users"), status_codes(201, 500))]
pub async fn create_user(new_user_json: JsonBody<NewUser>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(user = ?new_user_json, "create user");

    let JsonBody(new_user) = new_user_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_user.email);
    let new_user = User::insert_user(new_user).await?;

    vec.push(new_user);
    Ok(StatusCode::CREATED)
}

/// Update existing user.
#[endpoint(tags("users"), status_codes(200, 404))]
pub async fn update_user(
    id: PathParam<i32>,
    updated: JsonBody<User>,
) -> Result<StatusCode, StatusError> {
    tracing::debug!(user = ?updated, id = ?id, "update user");
    let mut vec = STORE.lock().await;

    for user in vec.iter_mut() {
        if user.id == *id {
            *user = (*updated).clone();
            return Ok(StatusCode::OK);
        }
    }

    tracing::debug!(id = ?id, "user is not found");
    Err(StatusError::not_found())
}

/// Delete user.
#[endpoint(tags("users"), status_codes(200, 401, 404))]
pub async fn delete_user(id: PathParam<i32>) -> Result<StatusCode, StatusError> {
    tracing::debug!(id = ?id, "delete user");

    let mut vec = STORE.lock().await;

    let len = vec.len();
    vec.retain(|user| user.id != *id);

    let deleted = vec.len() != len;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::debug!(id = ?id, "user is not found");
        Err(StatusError::not_found())
    }
}
