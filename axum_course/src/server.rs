use axum::extract::Query;
use axum::response::Html;
use axum::Router;
use tokio::net::TcpListener;
use axum::routing::get;
use crate::models::HelloParams;
use axum::response::IntoResponse;

pub async fn server() {
    let router = Router::new()
    .route("/", get(hello_handler))
    .route("/hello", get(hello_with_param_handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await.unwrap();   
}

pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

pub async fn hello_with_param_handler(params: Query<HelloParams>) -> impl IntoResponse {
    let name = &params.name.as_deref().unwrap_or("World");

    Html(format!("<h1>Hello, {}!</h1>", name))
}