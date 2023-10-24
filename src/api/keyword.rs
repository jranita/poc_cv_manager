use salvo::{endpoint, oapi::extract::*};
use salvo::writing::Json;
use salvo::http::StatusError;
use salvo::http::StatusCode;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::models::Keyword;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<Keyword>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List keywords.
#[endpoint(
    tags("keywords"),
    parameters(
        ("offset", description = "Offset is an optional query paramter."),
    )
)]
pub async fn list_keywords(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Json<Vec<Keyword>> {
    let keyword = STORE.lock().await;
    let keyword: Vec<Keyword> = keyword
        .clone()
        .into_iter()
        .skip(offset.into_inner().unwrap_or(0))
        .take(limit.into_inner().unwrap_or(std::usize::MAX))
        .collect();
    Json(keyword)
}

/// Create new keyword.
#[endpoint(tags("keywords"), status_codes(201, 409))]
pub async fn create_keyword(
    new_keyword: JsonBody<Keyword>,
) -> Result<StatusCode, StatusError> {
    tracing::debug!(keyword = ?new_keyword, "create keyword");

    let mut vec = STORE.lock().await;

    for keyword_x in vec.iter() {
        if keyword_x.id == new_keyword.id {
            tracing::debug!(id = ?new_keyword.id, "keyword already exists");
            return Err(StatusError::bad_request().brief("keyword already exists"));
        }
    }

    vec.push(new_keyword.into_inner());
    Ok(StatusCode::CREATED)
}

/// Update existing keyword.
#[endpoint(tags("keywords"), status_codes(200, 404))]
pub async fn update_keyword(
    id: PathParam<i32>,
    updated: JsonBody<Keyword>,
) -> Result<StatusCode, StatusError> {
    tracing::debug!(keyword = ?updated, id = ?id, "update keyword");
    let mut vec = STORE.lock().await;

    for keyword in vec.iter_mut() {
        if keyword.id == *id {
            *keyword = (*updated).clone();
            return Ok(StatusCode::OK);
        }
    }

    tracing::debug!(id = ?id, "keyword is not found");
    Err(StatusError::not_found())
}

/// Delete keyword.
#[endpoint(tags("keywords"), status_codes(200, 401, 404))]
pub async fn delete_keyword(id: PathParam<i32>) -> Result<StatusCode, StatusError> {
    tracing::debug!(id = ?id, "delete keyword");

    let mut vec = STORE.lock().await;

    let len = vec.len();
    vec.retain(|keyword| keyword.id != *id);

    let deleted = vec.len() != len;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::debug!(id = ?id, "keyword is not found");
        Err(StatusError::not_found())
    }
}
