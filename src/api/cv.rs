use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
use tokio::sync::Mutex;

use crate::models::cv::{NewCV, CV};

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<CV>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List cvs.
#[endpoint(
    tags("cvs"),
    parameters(
        ("offset", description = "Offset is an optional query parameter."),
        ("limit", description = "Limit is an optional query parameter."),
    )
)]
pub async fn list_cvs(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Result<Json<Vec<CV>>, salvo::Error> {
    println!("67     list_cvs()");
    let cvs_list = STORE.lock().await;

    let cvs_list: Vec<CV> = CV::get_cvs(
        limit.into_inner().unwrap_or_default(),
        offset.into_inner().unwrap_or_default(),
    )
    .await?;

    return std::result::Result::Ok(Json(cvs_list));
}

/// CV by ID.
#[endpoint(
    tags("cvs"),
    status_codes(200, 500),
    parameters(
        ("id", description = "Database ID for the CV"),
    )
)]
pub async fn get_cv_by_id(id: QueryParam<i32, true>) -> Result<Json<CV>, salvo::Error> {
    tracing::debug!(id = ?id, "get CV");
    let mut cv = STORE.lock().await;

    let target_cv: CV = CV::get_cv(id.into_inner()).await?;

    cv.push(target_cv.clone());

    std::result::Result::Ok(Json(target_cv))
}

/// Create new CV.
#[endpoint(tags("cvs"), status_codes(201, 500))]
pub async fn create_cv(new_cv_json: JsonBody<NewCV>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(cv = ?new_cv_json, "create cv");

    let JsonBody(new_cv) = new_cv_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_cv.file_name);
    let new_cv = CV::insert_cv(new_cv).await?;
    println!("52 new cv  {:?}", new_cv);
    vec.push(new_cv);
    Ok(StatusCode::CREATED)
}

/// Update existing cv.
#[endpoint(tags("cvs"), status_codes(200, 500))]
pub async fn update_cv(new_values_json: JsonBody<CV>) -> Result<StatusCode, Error> {
    tracing::debug!(cv = ?new_values_json, "update cv");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_cv = CV::update_cv(new_values).await?;

    vec.push(updated_cv);

    std::result::Result::Ok(StatusCode::OK)
}

/// Delete CV.
#[endpoint(tags("cvs"), status_codes(200, 401, 404))]
pub async fn delete_cv(id: PathParam<i32>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(id = ?id, "delete cv");

    let mut vec = STORE.lock().await;

    let deleted_company = CV::delete_cv(id.into_inner()).await?;

    vec.push(deleted_company);
    std::result::Result::Ok(StatusCode::OK)
}
