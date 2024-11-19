use axum::response::Html;
use axum::Router;
use tokio::net::TcpListener;
use axum::routing::get;

pub async fn server() {
    let router = Router::new()
    .route("/", get(|| async { Html("<h1>Hello, world!</h1>") }));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await.unwrap();   
}