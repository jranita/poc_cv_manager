use salvo::{prelude::ToSchema, Error};

use crate::{
    db_connectors::get_postgres,
    models::{user::CurrentUser, Deserialize, Serialize},
    Depot,
};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, Row, Type};

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

// TODO: limit and offset are needed for sorting and pagination
impl ClientCompany {
    pub async fn get_clients(
        depot: &mut Depot,
        limit: usize,
        offset: usize,
        order_by: String,
        order_direction: String,
        filter: String,
    ) -> Result<Vec<ClientCompany>, Error> {
        let query: String = format!("SELECT id, company_name, date_created FROM clientcompanies {} ORDER BY {} {} OFFSET {} LIMIT {}", filter, order_by, order_direction, offset, limit);

        println!("39 +++++ {}", query);

        let current_user: &CurrentUser = depot
            .get("currentuser")
            .expect("missing current user in depot");
        if current_user.role != "admin" {
            println!("No permissions to see this info, returning empty array");
            return Ok(vec![]);
        }

        let rows = sqlx::query(&query)
            .fetch_all(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        let clients_list = rows
            .iter()
            .map(|r| ClientCompany {
                id: r.get("id"),
                company_name: r.get("company_name"),
                // date_created: r.get<chrono::Utc>("date_created").date_naive(),
                date_created: r.get("date_created"),
            })
            .collect::<Vec<ClientCompany>>();

        Ok(clients_list)
    }

    pub async fn get_client(depot: &mut Depot, target_id: i32) -> Result<ClientCompany, Error> {
        let query_string = format!("SELECT * from clientcompanies where id={}", target_id);

        let current_user: &CurrentUser = depot
            .get("currentuser")
            .expect("missing current user in depot");
        if current_user.role != "admin" {
            println!("No permissions to see this info, stub result");
            return Ok(ClientCompany {
                id: 9999,
                company_name: "No permissions to see this info".to_string(),
                date_created: NaiveDateTime::default(),
            });
        }

        //TODO use query_as
        let row = sqlx::query(&query_string)
            .fetch_one(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let client = ClientCompany {
            id: row.get("id"),
            company_name: row.get("company_name"),
            date_created: row.get("date_created"),
        };

        Ok(client)
    }

    pub async fn insert_client(
        depot: &mut Depot,
        c: NewClientCompany,
    ) -> Result<ClientCompany, Error> {
        println!("56     insert_client() {:?}", c);

        let current_user: &CurrentUser = depot
            .get("currentuser")
            .expect("missing current user in depot");
        if current_user.role != "admin" {
            println!("No permissions to see this info, stub result");
            return Ok(ClientCompany {
                id: 9999,
                company_name: "No permissions to see this info".to_string(),
                date_created: NaiveDateTime::default(),
            });
        }

        let query: String = format!(
            "INSERT INTO clientcompanies (company_name) VALUES ('{}') RETURNING *",
            c.company_name
        );
        println!("59     quwery {:?}", query);

        let inserted = sqlx::query(&query)
            .fetch_one(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute insert query: {:?}", e);
                anyhow::anyhow!("Failed to insert record")
            })?;

        Ok(ClientCompany {
            id: inserted.get("id"),
            company_name: inserted.get("company_name"),
            date_created: inserted.get("date_created"),
        })
    }

    pub async fn update_client(
        depot: &mut Depot,
        c: ClientCompany,
    ) -> Result<ClientCompany, Error> {
        println!("101     update_client() {:?}", c);

        let current_user: &CurrentUser = depot
            .get("currentuser")
            .expect("missing current user in depot");
        if current_user.role != "admin" {
            println!("No permissions to see this info, stub result");
            return Ok(ClientCompany {
                id: 9999,
                company_name: "No permissions to see this info".to_string(),
                date_created: NaiveDateTime::default(),
            });
        }

        let query: String = format!(
            "UPDATE clientcompanies SET company_name='{}' WHERE id='{}'",
            c.company_name, c.id
        );
        println!("108     query {:?}", query);

        let updated = sqlx::query(&query)
            .execute(get_postgres())
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| {
                tracing::error!("Failed to execute update query: {:?}", e);
                anyhow::anyhow!("Failed to update record")
            })?;

        // TODO improve error creation/handling
        if updated == 0 {
            tracing::error!("Failed update query: probably the ID does not exist");
            return Err(Error::from(anyhow::anyhow!(
                "Failed update query: probably the ID does not exist"
            )));
        }

        Ok(c)
    }

    pub async fn delete_client(depot: &mut Depot, id: i32) -> Result<ClientCompany, Error> {
        println!("130     delete_client() {:?}", id);

        let current_user: &CurrentUser = depot
            .get("currentuser")
            .expect("missing current user in depot");
        if current_user.role != "admin" {
            println!("No permissions to see this info, stub result");
            return Ok(ClientCompany {
                id: 9999,
                company_name: "No permissions to see this info".to_string(),
                date_created: NaiveDateTime::default(),
            });
        }

        let query: String = format!("DELETE FROM clientcompanies WHERE id='{}'", id);
        println!("133     query {:?}", query);

        let deleted = sqlx::query(&query)
            .execute(get_postgres())
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| {
                tracing::error!("Failed to execute delete query: {:?}", e);
                anyhow::anyhow!("Failed to delete record")
            })?;

        // TODO improve error creation/handling
        if deleted == 0 {
            tracing::error!("Failed delete record: probably the ID does not exist");
            return Err(Error::from(anyhow::anyhow!(
                "Failed update query: probably the ID does not exist"
            )));
        }

        let ccc = ClientCompany {
            id,
            company_name: "company_name".to_string(),
            date_created: NaiveDateTime::default(),
        };

        Ok(ccc)
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewClientCompany {
    pub company_name: String,
}
