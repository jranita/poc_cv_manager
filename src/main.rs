use api::clientcompany::{
    create_client_company, delete_client_company, list_clients, update_client_company,
};
use api::cv::{create_cv, delete_cv, list_cvs, update_cv};
use api::fileupload::{upload, uploader};
use api::jobfunction::{
    create_job_function, delete_job_function, list_jobfunctions, update_job_function,
};
use api::keyword::{create_keyword, delete_keyword, list_keywords, update_keyword};
use api::user::{create_user, delete_user, list_users, update_user};
use db_connectors::{create_pg_pool, get_postgres};
use salvo::prelude::*;

pub mod models;
// use crate::models::*;

pub mod api;
pub mod db_connectors;
// use crate::api::clientcompany;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    create_pg_pool().await;

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
                    .push(Router::with_path("files").get(uploader).post(upload))
                    .push(Router::with_path("<id>").patch(update_cv).delete(delete_cv)),
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
