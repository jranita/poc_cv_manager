use salvo::{
    prelude::ToSchema,
    Error
};

use crate::{models::{Serialize, Deserialize}, db_connectors::get_postgres};
use sqlx::{FromRow, Type, Row};
use sqlx::types::chrono::NaiveDateTime;

// use crate::{
//     api::block_no_admin,
//     api::{JsonErrResponse, JsonOkResponse},
//     models::tag::{TagCount, Tags},
//     utils::{from_code, parse_json_body, parse_last_path, parse_query, set_json_response},
//     Routers,
// };

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow, Type)]
pub struct ClientCompany {
    pub id: i32,
    pub company_name: String,
    pub date_created: NaiveDateTime,
}

impl ClientCompany {

    pub async fn get_clients() -> Result<Vec<ClientCompany>, Error> {
        println!("28     Get_clients()");

        const QUERY: &str = "SELECT id, company_name, date_created from clientcompanies";

        let rows = sqlx::query(QUERY)
            .fetch_all(get_postgres())
            .await.map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let clients_list = rows
            .iter()
            .map(|r| ClientCompany {
                id: r.get("id"),
                company_name: r.get("company_name"),
                // date_created: r.get<chrono::Utc>("date_created").date_naive(),
                date_created: r.get("date_created"),
            })
            .collect::<Vec<ClientCompany>>();
        // println!("{:?}", clients_list[0]);

        Ok(clients_list)
    }

    pub async fn insert_client(c: NewClientCompany) -> Result<ClientCompany, Error> {
        println!("56     insert_client() {:?}", c);

        let query: String = format!("INSERT INTO clientcompanies (company_name) VALUES ('{}') RETURNING id", c.company_name );
        println!("59     quwery {:?}", query);

        let inserted = sqlx::query(&query)
            .execute(get_postgres())
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| {
                tracing::error!("Failed to execute insert query: {:?}", e);
                anyhow::anyhow!("Failed to insert record")
            })?;

        Ok(ClientCompany { id: inserted as i32, company_name: c.company_name, date_created: NaiveDateTime::default() })
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewClientCompany {
    pub company_name: String,
}

