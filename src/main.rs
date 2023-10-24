use api::clientcompany::{
    create_client_company, delete_client_company, list_clients, update_client_company,
};
use api::cv::{list_cvs, create_cv, update_cv, delete_cv};
use api::jobfunction::{list_jobfunctions, create_job_function, update_job_function, delete_job_function};
use api::keyword::{list_keywords, create_keyword, update_keyword, delete_keyword};
use api::user::{list_users, create_user, update_user, delete_user};
use once_cell::sync::{Lazy, OnceCell};
use salvo::oapi::{extract::*, ToSchema};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool, Row};
use tokio::sync::Mutex;

pub mod models;
use crate::models::*;

pub mod api;
use crate::api::clientcompany;

static STORE: Lazy<Db> = Lazy::new(new_store);
pub type Db = Mutex<Vec<User>>;

pub fn new_store() -> Db {
    Mutex::new(Vec::new())
}

static POSTGRES: OnceCell<PgPool> = OnceCell::new();

#[inline]
pub fn get_postgres() -> &'static PgPool {
    unsafe { POSTGRES.get_unchecked() }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Cannot load DB url from env");
    POSTGRES.set(pool).unwrap();

    let _ = sqlx::migrate!("./migrations").run(get_postgres()).await;

    let router = Router::new().get(index).push(
        Router::with_path("api")
            .push(
                Router::with_path("clients")
                    .get(list_clients)
                    .post(create_client_company)
                    .push(
                        Router::with_path("<id>")
                            .patch(update_client_company)
                            .delete(delete_client_company),
                    ),
            )
            .push(
                Router::with_path("clients")
                    .get(list_clients)
                    .post(create_client_company)
                    .push(
                        Router::with_path("<id>")
                            .patch(update_client_company)
                            .delete(delete_client_company),
                    ),
            )
            .push(
                Router::with_path("keywords")
                    .get(list_keywords)
                    .post(create_keyword)
                    .push(
                        Router::with_path("<id>")
                            .patch(update_keyword)
                            .delete(delete_keyword),
                    ),
            )
            .push(
                Router::with_path("jobfunctions")
                    .get(list_jobfunctions)
                    .post(create_job_function)
                    .push(
                        Router::with_path("<id>")
                            .patch(update_job_function)
                            .delete(delete_job_function),
                    ),
            )
            .push(
                Router::with_path("cvs")
                    .get(list_cvs)
                    .post(create_cv)
                    .push(
                        Router::with_path("<id>")
                            .patch(update_cv)
                            .delete(delete_cv),
                    ),
            )
            .push(
                Router::with_path("users")
                    .get(list_users)
                    .post(create_user)
                    .push(
                        Router::with_path("<id>")
                            .patch(update_user)
                            .delete(delete_user),
                    ),
            ),
    );

    let doc = OpenApi::new("CV Manager api", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
        .unshift(RapiDoc::new("/api-doc/openapi.json").into_router("/rapidoc"))
        .unshift(ReDoc::new("/api-doc/openapi.json").into_router("/redoc"));

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
pub async fn index() -> Text<&'static str> {
    Text::Html(INDEX_HTML)
}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>Oapi</title>
    </head>
    <body>
        <ul>
        <li><a href="swagger-ui" target="_blank">swagger-ui</a></li>
        <li><a href="rapidoc" target="_blank">rapidoc</a></li>
        </ul>
    </body>
</html>
"#;
