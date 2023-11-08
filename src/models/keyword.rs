use salvo::{prelude::ToSchema, Error};

use crate::{
    db_connectors::get_postgres,
    models::{Deserialize, Serialize},
};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, Row, Type};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow, Type)]
pub struct Keyword {
    pub id: i32,
    pub keyword_name: String,
    pub date_created: NaiveDateTime,
}

impl Keyword {
    pub async fn get_keywords() -> Result<Vec<Keyword>, Error> {
        println!("28     Get_keywords()");

        const QUERY: &str = "SELECT id, keyword_name, date_created from keywords";

        let rows = sqlx::query(QUERY)
            .fetch_all(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let keywords_list = rows
            .iter()
            .map(|r| Keyword {
                id: r.get("id"),
                keyword_name: r.get("keyword_name"),
                // date_created: r.get<chrono::Utc>("date_created").date_naive(),
                date_created: r.get("date_created"),
            })
            .collect::<Vec<Keyword>>();
        // println!("{:?}", keywords_list[0]);

        Ok(keywords_list)
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewKeyword {
    pub keyword_name: String,
}
