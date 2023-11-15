use salvo::{hyper::body::Bytes, oapi::RequestBody, prelude::ToSchema, Error};

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
    pub target_companies: Vec<i32>,
    pub target_job_functions: Vec<i32>,
    pub date_created: NaiveDateTime,
}

impl CV {
    pub async fn get_cvs() -> Result<Vec<CV>, Error> {
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
                target_companies: vec![],
                target_job_functions: vec![],
            })
            .collect::<Vec<CV>>();

        Ok(cvs_list)
    }

    pub async fn get_cv(target_id: i32) -> Result<CV, Error> {
        let query_string = format!("SELECT * from cvs where id={}", target_id);

        //TODO use query_as
        let row = sqlx::query(&query_string)
            .fetch_one(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let cv = CV {
            id: row.get("id"),
            cv_name: row.get("cv_name"),
            date_created: row.get("date_created"),
            file_name: row.get("file_name"),
            keyword_list: row.get("keyword_list"),
            target_companies: row.get("target_companies"),
            target_job_functions: row.get("target_job_functions"),
        };

        Ok(cv)
    }

    pub async fn insert_cv(c: NewCV) -> Result<CV, Error> {
        println!("52 ======\n {:?} \n=======\n", c);

        println!("56     insert_cv() {:?} {:?}", c, NaiveDateTime::default());

        let keywords: String = "{".to_owned()
            + &c.keyword_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        let jobfunctions: String = "{".to_owned()
            + &c.target_job_functions
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        let targetcompanies: String = "{".to_owned()
            + &c.target_companies
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        let query: String = format!(
            "INSERT INTO cvs (cv_name, file_name, target_companies, keyword_list, target_job_functions) VALUES ('{}', '{}','{}','{}','{}') RETURNING id",
            c.cv_name, c.file_name, targetcompanies, keywords, jobfunctions
        );
        println!("59     query {:?}", query);

        let mut inserted = sqlx::query(&query)
            .execute(get_postgres())
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| {
                tracing::error!("Failed to execute insert query: {:?}", e);
                anyhow::anyhow!("Failed to insert record")
            })?;

        // if inserted == 1 {
        //     let query: String = format!(
        //         "INSERT INTO cvs (cv_name, file_name, target_companies, keyword_list, target_job_functions) VALUES ('{}', '{}','{}','{}','{}') RETURNING id",
        //         c.cv_name, c.file_name, targetcompanies, keywords, jobfunctions
        //     );
        //     inserted = sqlx::query(&query)
        //         .execute(get_postgres())
        //         .await
        //         .map(|r| r.rows_affected())
        //         .map_err(|e| {
        //             tracing::error!("Failed to execute insert query: {:?}", e);
        //             anyhow::anyhow!("Failed to insert record")
        //         })?;
        // }

        Ok(CV {
            //TODO find a way to get last_inserted_id() value in Postgres
            id: inserted as i32,
            cv_name: c.cv_name,
            date_created: NaiveDateTime::default(),
            file_name: c.file_name,
            keyword_list: c.keyword_list,
            target_companies: c.target_companies,
            target_job_functions: c.target_job_functions,
        })
    }

    pub async fn update_cv(c: CV) -> Result<CV, Error> {
        println!("101     update_cv() {:?}", c);

        let keywords: String = "{".to_owned()
            + &c.keyword_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        let jobfunctions: String = "{".to_owned()
            + &c.target_job_functions
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        // TODO implement a Vec<i32> to "sql query string array" function
        let targetcompanies: String = "{".to_owned()
            + &c.target_companies
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
            + "}";

        let query: String = format!(
            "UPDATE cvs SET cv_name='{}', file_name='{}', keyword_list='{}', target_companies='{}', target_job_functions='{}' WHERE id='{}'",
            c.cv_name, c.file_name, keywords, targetcompanies, jobfunctions, c.id
        );
        println!("179     query {:?}", query);

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
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewCV {
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<i32>,
    pub target_companies: Vec<i32>,
    pub target_job_functions: Vec<i32>,
}
