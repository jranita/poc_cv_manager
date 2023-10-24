use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::http::StatusError;
use salvo::writing::Json;
use salvo::{endpoint, oapi::extract::*};
use tokio::sync::Mutex;

use crate::models::User;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<User>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List clients.
#[endpoint(
    tags("users"),
    parameters(
        ("offset", description = "Offset is an optional query paramter."),
    )
)]
pub async fn list_users(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Json<Vec<User>> {
    let user = STORE.lock().await;
    let user: Vec<User> = user
        .clone()
        .into_iter()
        .skip(offset.into_inner().unwrap_or(0))
        .take(limit.into_inner().unwrap_or(std::usize::MAX))
        .collect();
    Json(user)
}

/// Create new user.
#[endpoint(tags("users"), status_codes(201, 409))]
pub async fn create_user(new_user: JsonBody<User>) -> Result<StatusCode, StatusError> {
    tracing::debug!(user = ?new_user, "create user");

    let mut vec = STORE.lock().await;

    for user_x in vec.iter() {
        if user_x.id == new_user.id {
            tracing::debug!(id = ?new_user.id, "user already exists");
            return Err(StatusError::bad_request().brief("user already exists"));
        }
    }

    vec.push(new_user.into_inner());
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
