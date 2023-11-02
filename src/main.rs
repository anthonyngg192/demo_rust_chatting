use std::net::SocketAddr;

mod error;
use crate::{log::log_request, model::ModelController};

pub use self::error::{Error, Result};

mod ctx;
mod database;
mod log;
mod model;
mod web;

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mc = ModelController::new().await;

    let routes_api = web::routes_ticket::routes(mc.clone().unwrap())
        .layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_hello = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mv_ctx_resolve,
        ))
        .layer(middleware::map_response(main_response_mapper));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("server run on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(uri: Uri, request_method: Method, res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();

    let client_status_error = service_error.map(|f| f.client_status_and_error());
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
              "error":{
                "type":client_error.as_ref(),
                "req_uuid":uuid.to_string()
              }
            });
            println!("    ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;

    let _ = log_request(
        uuid,
        request_method,
        uri,
        service_error,
        client_error.as_ref(),
    )
    .await;

    error_response.unwrap_or(res)
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or_default();
    Html(format!("<strong>{name}</strong>"))
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello with path {name}"))
}
