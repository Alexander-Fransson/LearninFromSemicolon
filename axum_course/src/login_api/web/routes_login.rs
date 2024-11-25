use crate::{login_api::web, Error, Result};
use serde::Deserialize;
use axum::{ Json, Router};
use serde_json::{Value, json};
use axum::routing::post;
use tower_cookies::{Cookies, Cookie};

pub fn routes_login() -> Router {
    Router::new()
    .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // todo implement real db auth logic
    if payload.username != "test" || payload.pwd != "test" {
        return Err(Error::LoginFail);
    }

    // FIXME: implement real auth logic
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

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