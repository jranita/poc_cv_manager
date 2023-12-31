use once_cell::sync::Lazy;
use salvo::http::StatusCode;
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
        ("offset", description = "Offset is an optional query parameter. This is a integer value."),
        ("limit", description = "Limit is an optional query parameter. This is a integer value."),
        ("order_by", description = "OrderBy is an optional query parameter. Ex: 'id'."),
        ("order_direction", description = "Order Direction is an optional query parameter. Can be 'ASC' or 'DESC'."),
        ("filter", description = "Filter is an optional query parameter. String like: \"key1,value1,key2, value2 ...\""),
    )
)]
pub async fn list_clients(
    depot: &mut super::Depot,
    offset: QueryParam<usize, false>,
    limit: QueryParam<usize, false>,
    order_by: QueryParam<String, false>,
    order_direction: QueryParam<String, false>,
    filter: QueryParam<String, false>,
) -> Result<Json<Vec<ClientCompany>>, salvo::Error> {
    let clients_list = STORE.lock().await;

    let filterstring: String =
        filter.into_inner().unwrap_or_else(|| "".to_string());

    let clients_list: Vec<ClientCompany> = ClientCompany::get_clients(
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
    depot: &mut super::Depot,
    id: QueryParam<i32, true>,
) -> Result<Json<ClientCompany>, salvo::Error> {
    tracing::debug!(id = ?id, "get client company");
    let mut client = STORE.lock().await;

    let target_client: ClientCompany = ClientCompany::get_client(depot, id.into_inner()).await?;

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
    depot: &mut super::Depot,
    new_client_company_json: JsonBody<NewClientCompany>,
) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(client_company = ?new_client_company_json, "create client_company");

    let JsonBody(new_client_company) = new_client_company_json;

    let mut vec = STORE.lock().await;

    println!("50  {:?}", new_client_company.company_name);
    let new_company = ClientCompany::insert_client(depot, new_client_company).await?;

    vec.push(new_company);

    std::result::Result::Ok(StatusCode::CREATED)
}

/// Update existing client company.
#[endpoint(tags("clients"), status_codes(200, 500))]
pub async fn update_client_company(
    depot: &mut super::Depot,
    new_values_json: JsonBody<ClientCompany>,
) -> Result<StatusCode, Error> {
    tracing::debug!(client_company = ?new_values_json, "update client_company");

    let JsonBody(new_values) = new_values_json;

    let mut vec = STORE.lock().await;
    let updated_company = ClientCompany::update_client(depot, new_values).await?;
    // .map_err(|e| {
    //     tracing::error!("Failed to execute query: {:?}", e);
    //     anyhow::anyhow!("Failed to execute query")
    // })?;

    vec.push(updated_company);

    std::result::Result::Ok(StatusCode::OK)
}

/// Delete client_company.
#[endpoint(tags("clients"), status_codes(200, 401, 404))]
pub async fn delete_client_company(
    depot: &mut super::Depot,
    id: PathParam<i32>,
) -> Result<StatusCode, salvo::Error> {
    tracing::debug!(id = ?id, "delete client company");

    let mut vec = STORE.lock().await;

    // let len = vec.len();
    // vec.retain(|client_company| client_company.id != *id);

    // let deleted = vec.len() != len;
    // if deleted {
    //     Ok(StatusCode::NO_CONTENT)
    // } else {
    //     tracing::debug!(id = ?id, "client company is not found");
    //     Err(StatusError::not_found())
    // }
    let deleted_company = ClientCompany::delete_client(depot, id.into_inner()).await?;
    // .map_err(|e| Err(StatusError::not_found()))?;

    vec.push(deleted_company);
    std::result::Result::Ok(StatusCode::OK)
}
