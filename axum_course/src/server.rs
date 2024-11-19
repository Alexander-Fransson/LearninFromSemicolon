use axum::extract::{Query, Path};
use axum::response::Html;
use axum::Router;
use tokio::net::TcpListener;
use axum::routing::get;
use crate::models::HelloParams;
use crate::static_routes::routes::routes_static;
use axum::response::IntoResponse;

pub async fn server() {
    let router = Router::new()
    .merge(basic_routes())
    .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await.unwrap();   
}

fn basic_routes() -> Router {
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