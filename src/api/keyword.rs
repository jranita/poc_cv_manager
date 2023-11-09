use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::http::StatusError;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
use sqlx::Row;
use tokio::sync::Mutex;

use crate::models::keyword::Keyword;
use crate::models::keyword::NewKeyword;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<Keyword>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List keywords.
#[endpoint(
    tags("keywords"),
    parameters(
        ("offset", description = "Offset is an optional query parameter."),
        ("limit", description = "Limit is an optional query parameter."),
    )
)]
pub async fn list_keywords(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Result<Json<Vec<Keyword>>, salvo::Error> {
    println!("67     list_keywords()");
    let keywords_list = STORE.lock().await;

    let keywords_list: Vec<Keyword> = Keyword::get_keywords().await?;

    std::result::Result::Ok(Json(keywords_list))
}

/// Create new keyword.
#[endpoint(tags("keywords"), status_codes(201, 500))]
pub async fn create_keyword(new_keyword_json: JsonBody<NewKeyword>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(keyword = ?new_keyword_json, "create keyword");

    let JsonBody(new_keyword) = new_keyword_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_keyword.keyword_name);
    let new_keyword = Keyword::insert_keyword(new_keyword).await?;

    vec.push(new_keyword);
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
