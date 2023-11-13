use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::http::StatusError;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
use sqlx::Row;
use tokio::sync::Mutex;

use crate::models::cv::NewCV;
use crate::models::cv::CV;
use crate::models::keyword::Keyword;

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

    let cvs_list: Vec<CV> = CV::get_cvs().await?;

    std::result::Result::Ok(Json(cvs_list))
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

/// Update existing CV.
#[endpoint(tags("cvs"), status_codes(200, 404))]
pub async fn update_cv(
    id: PathParam<i32>,
    updated: JsonBody<CV>,
) -> Result<StatusCode, StatusError> {
    tracing::debug!(cv = ?updated, id = ?id, "update cv");
    let mut vec = STORE.lock().await;

    for cv in vec.iter_mut() {
        if cv.id == *id {
            *cv = (*updated).clone();
            return Ok(StatusCode::OK);
        }
    }

    tracing::debug!(id = ?id, "CV is not found");
    Err(StatusError::not_found())
}

/// Delete cv.
#[endpoint(tags("cvs"), status_codes(200, 401, 404))]
pub async fn delete_cv(id: PathParam<i32>) -> Result<StatusCode, StatusError> {
    tracing::debug!(id = ?id, "delete CV");

    let mut vec = STORE.lock().await;

    let len = vec.len();
    vec.retain(|cv| cv.id != *id);

    let deleted = vec.len() != len;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::debug!(id = ?id, "CV is not found");
        Err(StatusError::not_found())
    }
}
