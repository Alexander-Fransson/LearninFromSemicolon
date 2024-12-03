use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, Router};
use serde::Deserialize;
use axum::routing::get;

pub fn routes_basic() -> Router {
    Router::new()
    .route("/hello", get(handle_hello))
    .route("/hello_with_param", get(handle_hello_with_query_param))
    .route("/hello_with_path_param/:name", get(handle_hello_with_path_param))
}

pub async fn handle_hello() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

pub async fn handle_hello_with_query_param(Query(parmas): Query<HelloParams>) -> impl IntoResponse {
    let name = parmas.name.as_deref().unwrap_or("World");

    println!("Name: {}", name);

    Html(format!("<h1>Hello, {}</h1>", name))
}

pub async fn handle_hello_with_path_param(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Hello, {name}</h1>"))
}