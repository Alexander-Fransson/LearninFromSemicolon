use axum::response::Html;
use axum::response::IntoResponse;
use axum::extract::{Query, Path};
use axum::routing::get;
use axum::Router;
use crate::model::params_basic::HelloParams;

pub fn routes_basic() -> Router {
    Router::new()
    .route("/", get(hello_handler))
    .route("/hello", get(hello_with_param_handler))
    .route("/hello/:name", get(hello_with_path_param_handler))
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn hello_with_param_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("<h1>Hello, {}!</h1>", name))
}

async fn hello_with_path_param_handler(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Hello, {name}!</h1>"))
}