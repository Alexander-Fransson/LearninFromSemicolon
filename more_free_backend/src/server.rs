use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use axum::response::IntoResponse;

pub async fn server() {
    let app = Router::new()
    .route("/", get(test));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn test() -> impl IntoResponse {
    println!("test");

    (StatusCode::ACCEPTED, "Accepted")
}