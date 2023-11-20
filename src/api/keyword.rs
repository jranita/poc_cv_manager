use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
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

/// Keyword by ID.
#[endpoint(
    tags("keywords"),
    status_codes(200, 500),
    parameters(
        ("id", description = "Database ID for the Keyword"),
    )
)]
pub async fn get_keyword_by_id(id: QueryParam<i32, true>) -> Result<Json<Keyword>, salvo::Error> {
    tracing::debug!(id = ?id, "get Keyword");
    let mut keyword = STORE.lock().await;

    let target_keyword: Keyword = Keyword::get_keyword(id.into_inner()).await?;

    keyword.push(target_keyword.clone());

    std::result::Result::Ok(Json(target_keyword))
}

/// Create new keyword.
#[endpoint(tags("keywords"), status_codes(201, 500))]
pub async fn create_keyword(
    new_keyword_json: JsonBody<NewKeyword>,
) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(keyword = ?new_keyword_json, "create keyword");

    let JsonBody(new_keyword) = new_keyword_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_keyword.keyword_name);
    let new_keyword = Keyword::insert_keyword(new_keyword).await?;

    vec.push(new_keyword);
    Ok(StatusCode::CREATED)
}

/// Update existing keyword.
#[endpoint(tags("keywords"), status_codes(200, 500))]
pub async fn update_keyword(new_values_json: JsonBody<Keyword>) -> Result<StatusCode, Error> {
    tracing::debug!(keyword = ?new_values_json, "update keyword");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_keyword = Keyword::update_keyword(new_values).await?;

    vec.push(updated_keyword);

    std::result::Result::Ok(StatusCode::OK)
}

/// Delete Keyword.
#[endpoint(tags("keywords"), status_codes(200, 401, 404))]
pub async fn delete_keyword(id: PathParam<i32>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(id = ?id, "delete keyword");

    let mut vec = STORE.lock().await;

    let deleted_company = Keyword::delete_keyword(id.into_inner()).await?;

    vec.push(deleted_company);
    std::result::Result::Ok(StatusCode::OK)
}
