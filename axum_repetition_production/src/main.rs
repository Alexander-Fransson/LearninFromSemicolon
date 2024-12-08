#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use axum::Json;
use axum::http::{Method, Uri};
use axum::{middleware, response::Response, Router};
use ctx::extractor::Ctx;
use log::request_log_line::log_request;
use model::ticket::ModelController;
use serde_json::json;
use uuid::Uuid;
use web::mw_auth::{
    mw_ctx_resolver, mw_require_auth, mw_require_auth_2, mw_require_auth_3};
use web::{
    routes_basic::routes_basic, 
    routes_tickets::{routes_tickets, routes_tickets_2}};
use web::routes_static::routes_static;
use web::routes_login::routes_login;
use tower_cookies::CookieManagerLayer;
use middleware::from_fn;
use axum::response::IntoResponse;

pub use self::error::{Error, Result, ClientError}; // so you can get if from crate

mod ctx;
mod log;
mod model;
mod web;
mod error;
mod main_test;

#[tokio::main]
async fn main() {

}

async fn server_7() -> Result<()> {

    // this one handles the extracting only once while the ones bellow handles it multiple times

    let mc = ModelController::new().await?;
    let routes_api_with_mw = routes_tickets_2(mc.clone())
    .route_layer(from_fn(mw_require_auth_3));

    let routes_all = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .nest("/api", routes_api_with_mw)
    .layer(middleware::map_response(main_response_mapper_2))
    .layer(middleware::from_fn_with_state(
        mc.clone(), 
        mw_ctx_resolver    
    ))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static()); 
    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3005")
    .await
    .unwrap();

    axum::serve(listerner, routes_all).await.unwrap();

    Ok(())
}


async fn main_response_mapper_2(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response
) -> Response {
    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_and_error = service_error.map(|se| se.client_status_and_error());
    let error_response = client_status_and_error
    .as_ref()
    .map(|(status_code, client_error)| {
        let client_error_body =json!({
        "error": {
            "type": client_error.as_ref(),
            "uuid": uuid.to_string()
        }});

        (*status_code, Json(client_error_body)).into_response()
    });
    
    let client_error = client_status_and_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    error_response.unwrap_or(res)
    // could just return res if you dont want to map response and all
}

async fn server_6() -> Result<()> {

    let mc = ModelController::new().await?;

    let routes_api_with_mw = routes_tickets(mc.clone())
    .route_layer(from_fn(mw_require_auth_3));
    //.route_layer(from_fn(mw_require_auth_2));

    let routes_all = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .nest("/api", routes_api_with_mw)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static()); 
    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3005")
    .await
    .unwrap();

    axum::serve(listerner, routes_all).await.unwrap();

    Ok(())
}

async fn server_5() -> Result<()> {

    let mc = ModelController::new().await?;

    let routes_api_with_mw = routes_tickets(mc.clone())
    .route_layer(from_fn(mw_require_auth));

    let routes_all = Router::new()
    .merge(routes_basic())
    .merge(routes_login())
    .nest("/api", routes_api_with_mw)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static()); 
    let listerner = tokio::net::TcpListener::bind("127.0.0.1:3004")
    .await
    .unwrap();

    axum::serve(listerner, routes_all).await.unwrap();

    Ok(())
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
    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_and_error = service_error.map(|se| se.client_status_and_error());
    let error_response = client_status_and_error
    .as_ref()
    .map(|(status_code, client_error)| {
        let client_error_body =json!({
        "error": {
            "type": client_error.as_ref(),
            "uuid": uuid.to_string()
        }});

        (*status_code, Json(client_error_body)).into_response()
    });
    

    error_response.unwrap_or(res)
    // could just return res if you dont want to map response and all
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
