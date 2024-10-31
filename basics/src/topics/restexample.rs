// add axum by cargo add axum
#![allow(dead_code)]

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tokio::time::Instant;

pub async fn rest_example() {

    async fn get_instant_view() -> (StatusCode, String) {
        let formatted_str = format!("Hello world {:?}", Instant::now().to_owned());
        (StatusCode::OK, formatted_str)
    }

    let app = Router::new()
    .route("/time", get(get_instant_view));

    println!("Listening on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn test_rest_example() {

    tokio::spawn(async move {
        rest_example().await;
    });

    let client = reqwest::Client::new();
    let res = client.get("http://127.0.0.1:3000/time").send().await.unwrap();
    println!("{}", res.text().await.unwrap());
}