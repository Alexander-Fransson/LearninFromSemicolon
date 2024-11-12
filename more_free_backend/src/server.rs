use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};
use axum::response::IntoResponse;

use crate::controller::{get_info_handler, login_handler};
use crate::websockets::websocket_handler;

pub async fn server() {
    let app = Router::new()
    .route("/", get(test))
    .route("/login", post(login_handler))
    .route("/info", get(get_info_handler))
    .route("/send", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

pub async fn test() -> impl IntoResponse {
    println!("test");

    (StatusCode::ACCEPTED, "Accepted")
}