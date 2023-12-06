use salvo::{prelude::ToSchema, Error};

use crate::{
    db_connectors::get_postgres,
    models::{number_vec_to_string, Deserialize, Serialize},
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
    pub async fn get_users(_limit: usize, _offset: usize) -> Result<Vec<User>, Error> {
        println!("28     Get_users()");

        const QUERY: &str = "SELECT id, firstname, lastname, date_created from users";

        let rows = sqlx::query(QUERY)
            .fetch_all(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                anyhow::anyhow!("Failed to execute query")
            })?;

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

        println!("{}\n{}", user.pass, user.pass);

        Ok(user)
    }

    pub async fn insert_user(c: NewUser) -> Result<User, Error> {
        println!(
            "56     insert_user() {:?} {:?}",
            c,
            NaiveDateTime::default()
        );

        let query: String = format!(
            "INSERT INTO users (email, firstname, lastname, password, role) VALUES ('{}', '{}', '{}', '{}', '{}') RETURNING *",
            c.email, c.first_name, c.last_name, c.pass, c.role
        );
        println!("59     query {:?}", query);

        let inserted = sqlx::query(&query)
            .fetch_one(get_postgres())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute insert query: {:?}", e);
                anyhow::anyhow!("Failed to insert record")
            })?;

        let hashed_password = crate::authentication::hash_password(inserted.get("password"))?;

        Ok(User {
            id: inserted.get("id"),
            first_name: inserted.get("firstname"),
            last_name: inserted.get("lastname"),
            email: inserted.get("email"),
            pass: hashed_password,
            date_created: inserted.get("date_created"),
            cv_id_list: inserted.get("cv_id_list"),
            role: inserted.get("role"),
        })
    }

    pub async fn update_user(c: User) -> Result<User, Error> {
        println!("101     update_user() {:?}", c);

        let cvs: String = number_vec_to_string(&c.cv_id_list);
        let query: String = format!(
            "UPDATE users SET firstname='{}', lastname='{}', email='{}' password='{}' role='{}' cv_id_list='{}' WHERE id='{}'",
            c.first_name, c.last_name, c.email, c.pass, c.role, cvs, c.id
        );
        println!("133     query {:?}", query);

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

    pub async fn delete_user(id: i32) -> Result<User, Error> {
        println!("130     delete_user() {:?}", id);

        let query: String = format!("DELETE FROM users WHERE id='{}'", id);
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
                "Failed delete query: probably the ID does not exist"
            )));
        }

        let ccc = User {
            id,
            date_created: NaiveDateTime::default(),
            first_name: "first_name".to_string(),
            last_name: "last_name".to_string(),
            email: "last_name".to_string(),
            pass: "last_name".to_string(),
            role: "last_name".to_string(),
            cv_id_list: vec![],
        };

        Ok(ccc)
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
