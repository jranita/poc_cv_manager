use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::writing::Json;
use salvo::{endpoint, oapi::extract::*};
use salvo::{Depot, Error};
use tokio::sync::Mutex;

use crate::authentication;
use crate::models::user::User;
use crate::models::user::{NewUser, PasswordStruct};

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<User>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List users.
#[endpoint(
    tags("users"),
    parameters(
        ("offset", description = "Offset is an optional query parameter. This is a integer value."),
        ("limit", description = "Limit is an optional query parameter. This is a integer value."),
        ("order_by", description = "OrderBy is an optional query parameter. Ex: 'id'."),
        ("order_direction", description = "Order Direction is an optional query parameter. Can be 'ASC' or 'DESC'."),
        ("filter", description = "Filter is an optional query parameter. String like: \"key1,value1,key2, value2 ...\""),
    )
)]
pub async fn list_users(
    depot: &mut super::Depot,
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
    order_by: QueryParam<String, false>,
    order_direction: QueryParam<String, false>,
    filter: QueryParam<String, false>,
) -> Result<Json<Vec<User>>, salvo::Error> {
    let users_list = STORE.lock().await;

    let filterstring: String =
        filter.into_inner().unwrap_or_else(|| "".to_string());

    let users_list: Vec<User> = User::get_users(
        depot,
        limit.into_inner().unwrap_or_else(|| 1000),
        offset.into_inner().unwrap_or_default(),
        order_by.into_inner().unwrap_or_else(|| "id".to_string()),
        order_direction
            .into_inner()
            .unwrap_or_else(|| "ASC".to_string()),
        super::string_to_filter(filterstring),
    )
    .await?;

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
pub async fn get_user_by_id(
    depot: &mut Depot,
    id: QueryParam<i32, true>,
) -> Result<Json<User>, salvo::Error> {
    tracing::debug!(id = ?id, "get User");
    let mut user = STORE.lock().await;

    let target_user: User = User::get_user(depot, id.into_inner()).await?;

    user.push(target_user.clone());

    std::result::Result::Ok(Json(target_user))
}

/// Create new user.
#[endpoint(tags("users"), status_codes(201, 500))]
pub async fn create_user(new_user_json: JsonBody<NewUser>) -> Result<StatusCode, Error> {
    tracing::debug!(user = ?new_user_json, "create user");

    let JsonBody(mut new_user) = new_user_json;
    new_user.pass = authentication::hash_password(new_user.pass)?;

    let mut vec = STORE.lock().await;

    let new_user = User::insert_user(new_user).await?;

    vec.push(new_user);
    Ok(StatusCode::CREATED)
}

/// Update existing user.
#[endpoint(tags("users"), status_codes(200, 500))]
pub async fn update_user(new_values_json: JsonBody<User>) -> Result<StatusCode, Error> {
    tracing::debug!(user = ?new_values_json, "update user");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_user = User::update_user(new_values).await?;

    vec.push(updated_user);

    std::result::Result::Ok(StatusCode::OK)
}

/// Change existing user password.
#[endpoint(tags("users"), status_codes(200, 500))]
pub async fn update_user_password(
    depot: &mut Depot,
    new_values_json: JsonBody<PasswordStruct>,
) -> Result<StatusCode, Error> {
    tracing::debug!(user = ?new_values_json, "change password");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_user = User::change_user_password(depot, new_values).await?;

    vec.push(updated_user);

    std::result::Result::Ok(StatusCode::OK)
}

/// Delete User.
#[endpoint(tags("users"), status_codes(200, 401, 404))]
pub async fn delete_user(id: PathParam<i32>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(id = ?id, "delete user");

    let mut vec = STORE.lock().await;

    let deleted_company = User::delete_user(id.into_inner()).await?;

    vec.push(deleted_company);
    std::result::Result::Ok(StatusCode::OK)
}
