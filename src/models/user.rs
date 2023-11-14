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
    pub role: String,
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
                role: "".to_owned(),
                pass: "".to_owned(),
                cv_id_list: vec![],
            })
            .collect::<Vec<User>>();
        // println!("{:?}", users_list[0]);

        Ok(users_list)
    }

    pub async fn get_user(target_id: i32) -> Result<User, Error> {
        let query_string = format!("SELECT * from users where id={}", target_id);

        //TODO use query_as
        let row = sqlx::query(&query_string)
            .fetch_one(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

        // println!("{:?}", rows[0].columns());

        let user = User {
            id: row.get("id"),
            first_name: row.get("firstname"),
            last_name: row.get("lastname"),
            date_created: row.get("date_created"),
            email: row.get("email"),
            pass: row.get("password"),
            role: row.get("role"),
            cv_id_list: row.get("cv_id_list"),
        };

        Ok(user)
    }

    pub async fn insert_user(c: NewUser) -> Result<User, Error> {
        println!(
            "56     insert_user() {:?} {:?}",
            c,
            NaiveDateTime::default()
        );

        let query: String = format!(
            "INSERT INTO users (firstname, lastname, email, password, role) VALUES ('{}', '{}', '{}', '{}', '{}') RETURNING id", 
            c.first_name, c.last_name, c.email, c.pass, c.role
        );
        println!("59     query {:?}", query);

        let inserted = sqlx::query(&query)
            .execute(get_postgres())
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| {
                tracing::error!("Failed to execute insert query: {:?}", e);
                anyhow::anyhow!("Failed to insert record")
            })?;

        Ok(User {
            id: inserted as i32,
            first_name: c.first_name,
            last_name: c.last_name,
            email: c.email,
            pass: "".to_string(),
            date_created: NaiveDateTime::default(),
            cv_id_list: vec![],
            role: c.role,
        })
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub role: String,
}
