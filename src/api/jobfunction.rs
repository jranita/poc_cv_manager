use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
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
        ("order_by", description = "OrderBy is an optional query parameter. Ex: 'id'."),
        ("order_direction", description = "Order Direction is an optional query parameter. Can be 'ASC' or 'DESC'."),
        ("filter", description = "Filter is an optional query parameter. String like: \"key1,value1,key2, value2 ...\""),
    )
)]
pub async fn list_jobfunctions(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
    order_by: QueryParam<String, false>,
    order_direction: QueryParam<String, false>,
    filter: QueryParam<String, false>,
) -> Result<Json<Vec<JobFunction>>, salvo::Error> {
    println!("67     list_jobfunctions()");
    let jobfunctions_list = STORE.lock().await;

    let filterstring: String = filter.into_inner().unwrap_or_else(|| "".to_string());

    let jobfunctions_list: Vec<JobFunction> = JobFunction::get_jobfunctions(
        limit.into_inner().unwrap_or_else(|| 1000),
        offset.into_inner().unwrap_or_default(),
        order_by.into_inner().unwrap_or_else(|| "id".to_string()),
        order_direction
            .into_inner()
            .unwrap_or_else(|| "ASC".to_string()),
        super::string_to_filter(filterstring),
    )
    .await?;

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
pub async fn get_job_function_by_id(
    id: QueryParam<i32, true>,
) -> Result<Json<JobFunction>, salvo::Error> {
    tracing::debug!(id = ?id, "get Job Function");
    let mut job_function = STORE.lock().await;

    let target_job_function: JobFunction = JobFunction::get_job_function(id.into_inner()).await?;

    job_function.push(target_job_function.clone());

    std::result::Result::Ok(Json(target_job_function))
}

#[endpoint(tags("jobfunctions"), status_codes(201, 500))]
pub async fn create_job_function(
    new_job_function_json: JsonBody<NewJobFunction>,
) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(job_function = ?new_job_function_json, "create job_function");

    let JsonBody(new_job_function) = new_job_function_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_job_function.job_function_name);
    let new_job_function = JobFunction::insert_jobfunction(new_job_function).await?;

    vec.push(new_job_function);
    Ok(StatusCode::CREATED)
}

/// Update existing job function.
#[endpoint(tags("jobfunctions"), status_codes(200, 500))]
pub async fn update_job_function(
    new_values_json: JsonBody<JobFunction>,
) -> Result<StatusCode, Error> {
    tracing::debug!(job_function = ?new_values_json, "update job function");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_job_function = JobFunction::update_jobfunction(new_values).await?;

    vec.push(updated_job_function);

    std::result::Result::Ok(StatusCode::OK)
}

/// Delete job function.
#[endpoint(tags("jobfunctions"), status_codes(200, 401, 404))]
pub async fn delete_job_function(id: PathParam<i32>) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(id = ?id, "delete job function");

    let mut vec = STORE.lock().await;

    let deleted_job_function = JobFunction::delete_jobfunction(id.into_inner()).await?;

    vec.push(deleted_job_function);
    std::result::Result::Ok(StatusCode::OK)
}
