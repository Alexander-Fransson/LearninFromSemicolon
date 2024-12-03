#![allow(dead_code)]

use axum::Router;
use web::routes_basic::routes_basic;

mod ctx;
mod log;
mod model;
mod web;
mod error;
mod main_test;

#[tokio::main]
async fn main() {

}

async fn server_2() {
    let routes_all = Router::new()
    .merge(routes_basic());

    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3001")
    .await
    .unwrap();

    axum::serve(listerner, routes_all).await.unwrap();
}

async fn server_1() {
    // let routes_hello = Router::new()
    // .route("/hello", get(handle_hello))
    // .route("/hello_with_param", get(handle_hello_with_query_param))
    // .route("/hello_with_path_param/:name", get(handle_hello_with_path_param));

    let routes_hello = Router::new().merge(routes_basic());

    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    axum::serve(listerner, routes_hello).await.unwrap();
}
