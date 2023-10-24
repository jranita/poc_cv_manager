use salvo::{endpoint, oapi::extract::*};
use salvo::writing::Json;
use salvo::http::StatusError;
use salvo::http::StatusCode;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::models::CV;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<CV>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}


/// List cvs.
#[endpoint(
    tags("cvs"),
    parameters(
        ("offset", description = "Offset is an optional query paramter."),
    )
)]
pub async fn list_cvs(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Json<Vec<CV>> {
    let cv = STORE.lock().await;
    let cv: Vec<CV> = cv
        .clone()
        .into_iter()
        .skip(offset.into_inner().unwrap_or(0))
        .take(limit.into_inner().unwrap_or(std::usize::MAX))
        .collect();
    Json(cv)
}

/// Create new CV.
#[endpoint(tags("cvs"), status_codes(201, 409))]
pub async fn create_cv(
    new_cv: JsonBody<CV>,
) -> Result<StatusCode, StatusError> {
    tracing::debug!(cv = ?new_cv, "create cv");

    let mut vec = STORE.lock().await;

    for cv_x in vec.iter() {
        if cv_x.id == new_cv.id {
            tracing::debug!(id = ?new_cv.id, "cv already exists");
            return Err(StatusError::bad_request().brief("cv already exists"));
        }
    }

    vec.push(new_cv.into_inner());
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
