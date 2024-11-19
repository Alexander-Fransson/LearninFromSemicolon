use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;

pub fn routes_static() -> Router {
    Router::new()
    .nest_service("/", get_service(ServeDir::new("./")))
}