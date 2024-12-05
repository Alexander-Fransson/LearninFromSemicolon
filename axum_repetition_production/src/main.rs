#![allow(dead_code)]

use axum::{middleware, response::Response, Router};
use model::ticket::ModelController;
use web::{routes_basic::routes_basic, routes_tickets::routes_tickets};
use web::routes_static::routes_static;
use web::routes_login::routes_login;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result}; // so you can get if from crate

mod ctx;
mod log;
mod model;
mod web;
mod error;
mod main_test;

#[tokio::main]
async fn main() {

}

async fn server_4() -> Result<()> {

    let mc = ModelController::new().await?;

    let routes_all = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .nest("/api", routes_tickets(mc.clone()))
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static()); 
    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3003")
    .await
    .unwrap();

    axum::serve(listerner, routes_all).await.unwrap();

    Ok(())
}

async fn server_3() {
    let routes_all = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static()); 
    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3002")
    .await
    .unwrap();

    axum::serve(listerner, routes_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    res
}

async fn server_2() {
    let routes_all = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .fallback_service(routes_static()); // fall back to static files if no route matches

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
