pub mod main_test;
pub mod ctx;
pub mod log;
pub mod model;
pub mod web;
pub mod error;

pub use crate::error::{Error, Result, ClientError};

use tokio::net::TcpListener;
use axum::Router;
use tower_cookies::CookieManagerLayer;
use crate::web::routes_basic::routes_basic;
use crate::web::routes_login::routes_login;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

pub async fn server() {
    let router = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .layer(CookieManagerLayer::new());

    let listener = TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, router)
    .await
    .unwrap();
}

pub fn test_tests() -> u32 {
    16
}
