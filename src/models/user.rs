use salvo::{prelude::ToSchema, Error};

use crate::{
    db_connectors::get_postgres,
    models::{Deserialize, Serialize},
};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, Row, Type};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow, Type)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub cv_id_list: Vec<i32>,
    pub date_created: NaiveDateTime,
}

impl User {
    pub async fn get_users() -> Result<Vec<User>, Error> {
        println!("28     Get_users()");

        const QUERY: &str = "SELECT id, firstname, lastname, date_created from users";

        let rows = sqlx::query(QUERY)
            .fetch_all(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let users_list = rows
            .iter()
            .map(|r| User {
                id: r.get("id"),
                first_name: r.get("firstname"),
                last_name: r.get("lastname"),
                date_created: r.get("date_created"),
                email: "".to_owned(),
                pass: "".to_owned(),
                cv_id_list: vec![],
            })
            .collect::<Vec<User>>();
        // println!("{:?}", users_list[0]);

        Ok(users_list)
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
}
