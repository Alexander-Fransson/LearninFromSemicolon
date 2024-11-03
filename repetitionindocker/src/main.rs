use axum::http::StatusCode;
use axum::{Json,Router};
use axum::routing::get;
use serde_json::{json, Value};

async fn health_handler() -> (StatusCode, Json<Value>) {
    let response = json!({
        "Message": "App is running"
    });
    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    let router = Router::new()
    .route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    println!("Listening...");

    let server = axum::serve(listener, router)
    .await;

    if let Err(err) = server {
        eprintln!("Error: {}", err);
    }
}
