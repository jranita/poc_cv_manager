use salvo::{prelude::ToSchema, Error};

use crate::{
    db_connectors::get_postgres,
    models::{Deserialize, Serialize},
};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, Row, Type};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow, Type)]
pub struct CV {
    pub id: i32,
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<i32>,
    pub target_job_function: Vec<i32>,
    pub date_created: NaiveDateTime,
}

impl CV {
    pub async fn get_cvs() -> Result<Vec<CV>, Error> {
        println!("28     Get_cvs()");

        const QUERY: &str = "SELECT id, cv_name, date_created from cvs";

        let rows = sqlx::query(QUERY)
            .fetch_all(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        let cvs_list = rows
            .iter()
            .map(|r| CV {
                id: r.get("id"),
                cv_name: r.get("cv_name"),
                // date_created: r.get<chrono::Utc>("date_created").date_naive(),
                date_created: r.get("date_created"),
                file_name: "".to_owned(),
                keyword_list: vec![],
                target_job_function: vec![],
            })
            .collect::<Vec<CV>>();

        Ok(cvs_list)
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewCV {
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<i32>,
    pub target_job_function: Vec<i32>,
}
