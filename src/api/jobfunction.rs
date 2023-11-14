use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::http::StatusError;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
use sqlx::Row;
use tokio::sync::Mutex;

use crate::models::job_function::JobFunction;
use crate::models::job_function::NewJobFunction;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<JobFunction>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List jobfunctions.
#[endpoint(
    tags("jobfunctions"),
    parameters(
        ("offset", description = "Offset is an optional query parameter."),
        ("limit", description = "Limit is an optional query parameter."),
    )
)]
pub async fn list_jobfunctions(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Result<Json<Vec<JobFunction>>, salvo::Error> {
    println!("67     list_jobfunctions()");
    let jobfunctions_list = STORE.lock().await;

    let jobfunctions_list: Vec<JobFunction> = JobFunction::get_jobfunctions().await?;

    std::result::Result::Ok(Json(jobfunctions_list))
}

/// Job Function by ID.
#[endpoint(
    tags("jobfunctions"),
    status_codes(200, 500),
    parameters(
        ("id", description = "Database ID for the Job Function"),
    )
)]
pub async fn get_job_function_by_id(id: QueryParam<i32, true>) -> Result<Json<JobFunction>, salvo::Error> {
    tracing::debug!(id = ?id, "get Job Function");
    let mut job_function = STORE.lock().await;

    let target_job_function: JobFunction = JobFunction::get_job_function(id.into_inner()).await?;

    job_function.push(target_job_function.clone());

    std::result::Result::Ok(Json(target_job_function))
}

#[endpoint(tags("jobfunctions"), status_codes(201, 500))]
pub async fn create_job_function(new_job_function_json: JsonBody<NewJobFunction>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(job_function = ?new_job_function_json, "create job_function");

    let JsonBody(new_job_function) = new_job_function_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_job_function.job_function_name);
    let new_job_function = JobFunction::insert_jobfunction(new_job_function).await?;

    vec.push(new_job_function);
    Ok(StatusCode::CREATED)
}

/// Update existing job function.
#[endpoint(tags("jobfunctions"), status_codes(200, 404))]
pub async fn update_job_function(
    id: PathParam<i32>,
    updated: JsonBody<JobFunction>,
) -> Result<StatusCode, StatusError> {
    tracing::debug!(job_function = ?updated, id = ?id, "update job_function");
    let mut vec = STORE.lock().await;

    for job_function in vec.iter_mut() {
        if job_function.id == *id {
            *job_function = (*updated).clone();
            return Ok(StatusCode::OK);
        }
    }

    tracing::debug!(id = ?id, "job function is not found");
    Err(StatusError::not_found())
}

/// Delete job_function.
#[endpoint(tags("jobfunctions"), status_codes(200, 401, 404))]
pub async fn delete_job_function(id: PathParam<i32>) -> Result<StatusCode, StatusError> {
    tracing::debug!(id = ?id, "delete job function");

    let mut vec = STORE.lock().await;

    let len = vec.len();
    vec.retain(|job_function| job_function.id != *id);

    let deleted = vec.len() != len;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::debug!(id = ?id, "job function is not found");
        Err(StatusError::not_found())
    }
}
