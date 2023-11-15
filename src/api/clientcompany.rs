use once_cell::sync::Lazy;
use salvo::http::StatusCode;
use salvo::http::StatusError;
use salvo::prelude::*;
use salvo::writing::Json;
use salvo::Error;
use salvo::{endpoint, oapi::extract::*};
use tokio::sync::Mutex;

use crate::models::client_company::{ClientCompany, NewClientCompany};

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<ClientCompany>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

/// List clients.
#[endpoint(
    tags("clients"),
    status_codes(200, 500),
    parameters(
        ("offset", description = "Offset is an optional query parameter."),
        ("limit", description = "Limit is an optional query parameter."),
    )
)]
pub async fn list_clients(
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
) -> Result<Json<Vec<ClientCompany>>, salvo::Error> {
    let clients_list = STORE.lock().await;

    let clients_list: Vec<ClientCompany> = ClientCompany::get_clients().await?;

    std::result::Result::Ok(Json(clients_list))
}

/// Client by ID.
#[endpoint(
    tags("clients"),
    status_codes(200, 500),
    parameters(
        ("id", description = "Database ID for the client"),
    )
)]
pub async fn get_client_by_id(
    id: QueryParam<i32, true>,
) -> Result<Json<ClientCompany>, salvo::Error> {
    tracing::debug!(id = ?id, "get client company");
    let mut client = STORE.lock().await;

    let target_client: ClientCompany = ClientCompany::get_client(id.into_inner()).await?;

    client.push(target_client.clone());

    std::result::Result::Ok(Json(target_client))
}

/// Find Clients by name.
// #[endpoint(
//     tags("clients"),
//     status_codes(200, 500),
//     parameters(
//         ("search_string", description = "string in client name"),
//     )
// )]
// pub async fn search_clients(
//     search_string: JsonBody<String>,
// ) -> Result<Json<Vec<ClientCompany>>, salvo::Error> {
//     tracing::debug!(search_string = ?search_string, "search client company");
//     let mut found_clients = STORE.lock().await;

//     let found_clients: Vec<ClientCompany> = ClientCompany::search_clients(search_string).await?;

//     std::result::Result::Ok(Json(found_clients))
// }

/// Create new client company.
#[endpoint(tags("clients"), status_codes(201, 500))]
pub async fn create_client_company(
    new_client_company_json: JsonBody<NewClientCompany>,
) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(client_company = ?new_client_company_json, "create client_company");

    let JsonBody(new_client_company) = new_client_company_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_client_company.company_name);
    let new_company = ClientCompany::insert_client(new_client_company).await?;

    vec.push(new_company);

    std::result::Result::Ok(StatusCode::CREATED)
}

/// Update existing client company.
#[endpoint(tags("clients"), status_codes(200, 500))]
pub async fn update_client_company(
    new_values_json: JsonBody<ClientCompany>,
) -> Result<StatusCode, Error> {
    tracing::debug!(client_company = ?new_values_json, "update client_company");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_company = ClientCompany::update_client(new_values).await?;
    // .map_err(|e| {
    //     tracing::error!("Failed to execute query: {:?}", e);
    //     anyhow::anyhow!("Failed to execute query")
    // })?;

    vec.push(updated_company);

    std::result::Result::Ok(StatusCode::OK)
}

/// Delete client_company.
#[endpoint(tags("clients"), status_codes(200, 401, 404))]
pub async fn delete_client_company(id: PathParam<i32>) -> Result<StatusCode, StatusError> {
    tracing::debug!(id = ?id, "delete client company");

    let mut vec = STORE.lock().await;

    let len = vec.len();
    vec.retain(|client_company| client_company.id != *id);

    let deleted = vec.len() != len;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::debug!(id = ?id, "client company is not found");
        Err(StatusError::not_found())
    }
}
