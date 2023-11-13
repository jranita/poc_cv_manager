use salvo::{prelude::ToSchema, Error};

use crate::{
    db_connectors::get_postgres,
    models::{Deserialize, Serialize},
};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, Row, Type};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow, Type)]
pub struct JobFunction {
    pub id: i32,
    pub job_function_name: String,
    pub keyword_list: Vec<i32>,
    pub date_created: NaiveDateTime,
}

impl JobFunction {
    pub async fn get_jobfunctions() -> Result<Vec<JobFunction>, Error> {
        println!("28     Get_job_functions()");

        const QUERY: &str = "SELECT id, job_function_name, date_created from jobfunctions";

        let rows = sqlx::query(QUERY)
            .fetch_all(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let job_functions_list = rows
            .iter()
            .map(|r| JobFunction {
                id: r.get("id"),
                job_function_name: r.get("job_function_name"),
                date_created: r.get("date_created"),
                keyword_list: vec![],
            })
            .collect::<Vec<JobFunction>>();
        // println!("{:?}", job_functions_list[0]);

        Ok(job_functions_list)
    }

    pub async fn insert_jobfunction(c: NewJobFunction) -> Result<JobFunction, Error> {
        println!(
            "56     insert_jobfunction() {:?} {:?}",
            c,
            NaiveDateTime::default()
        );

        let keywords: String = "{".to_owned()
            + &c.keyword_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        let query: String = format!("INSERT INTO jobfunctions (job_function_name, keyword_list) VALUES ('{}', '{}') RETURNING id", c.job_function_name, keywords );
        println!("59     query {:?}", query);

        let inserted = sqlx::query(&query)
            .execute(get_postgres())
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| {
                tracing::error!("Failed to execute insert query: {:?}", e);
                anyhow::anyhow!("Failed to insert record")
            })?;

        Ok(JobFunction {
            id: inserted as i32,
            job_function_name: c.job_function_name,
            keyword_list: c.keyword_list,
            date_created: NaiveDateTime::default(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NewJobFunction {
    pub job_function_name: String,
    pub keyword_list: Vec<i32>,
}
