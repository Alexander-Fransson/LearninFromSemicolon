use crate::{Error, Result};
use serde::Deserialize;
use axum::{Json, Router};
use serde_json::{Value, json};
use axum::routing::post;

pub fn routes_login() -> Router {
    Router::new()
    .route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    println!("payload: {:#?}", payload);

    // todo implement real db auth logic
    if payload.username != "test" || payload.pwd != "test" {
        return Err(Error::LoginFail);
    }

    // Todo set cookies

    // generate success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}