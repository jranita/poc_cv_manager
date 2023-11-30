use api::clientcompany::{
    create_client_company, delete_client_company, get_client_by_id, list_clients,
    update_client_company,
};
use api::cv::{create_cv, delete_cv, get_cv_by_id, list_cvs, update_cv};
use api::fileupload::{upload, uploader};
use api::jobfunction::{
    create_job_function, delete_job_function, get_job_function_by_id, list_jobfunctions,
    update_job_function,
};
use api::keyword::{
    create_keyword, delete_keyword, get_keyword_by_id, list_keywords, update_keyword,
};
use api::user::{create_user, delete_user, get_user_by_id, list_users, update_user};
use authentication::Validator;
use db_connectors::create_pg_pool;
use salvo::prelude::*;

pub mod models;

pub mod api;
pub mod authentication;
pub mod db_connectors;
pub mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    create_pg_pool().await;

    // let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    // let auth_handler = BasicAuth::new(Validator {
    //     username: "Clara".to_string(),
    //     password: "1234".to_string(),
    // });

    let router = Router::with_hoop(auth_handler).get(index).push(
        // Router::with_path("login").post(auth)
        Router::with_path("api")
            .push(
                Router::with_path("clients")
                    .get(list_clients)
                    .post(create_client_company)
                    .push(
                        Router::with_path("<id>")
                            .get(get_client_by_id)
                            .patch(update_client_company)
                            .delete(delete_client_company),
                    ),
                // .push(Router::with_path("search").post(search_clients)),
            )
            .push(
                Router::with_path("keywords")
                    .get(list_keywords)
                    .post(create_keyword)
                    .push(
                        Router::with_path("<id>")
                            .get(get_keyword_by_id)
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
                            .get(get_job_function_by_id)
                            .patch(update_job_function)
                            .delete(delete_job_function),
                    ),
            )
            .push(
                Router::with_path("cvs")
                    .get(list_cvs)
                    .post(create_cv)
                    .push(Router::with_path("files").get(uploader).post(upload))
                    .push(
                        Router::with_path("<id>")
                            .get(get_cv_by_id)
                            .patch(update_cv)
                            .delete(delete_cv),
                    ), // .push(Router::with_path("search").post(search_cvs)),
            )
            .push(
                Router::with_path("users")
                    .get(list_users)
                    .post(create_user)
                    .push(
                        Router::with_path("<id>")
                            .get(get_user_by_id)
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
