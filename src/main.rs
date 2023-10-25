use std::net::SocketAddr;

mod error;
pub use self::error::{Error, Result};

mod web;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router, middleware,
};
use serde::Deserialize;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let routes_hello = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("server run on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
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
