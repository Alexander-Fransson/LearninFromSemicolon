use crate::{Error, Result};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use axum::Router;
use axum::routing::post;
use tower_cookies::{Cookie, Cookies};
use super::AUTH_TOKEN;

pub fn routes_login() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/login/v2", post(api_login_2))
}

async fn api_login_2(cookies: Cookies ,payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "test" || payload.password != "test" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    cookies.add(Cookie::new(AUTH_TOKEN,"user-1.exp.sign"));
    
    Ok(body)
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    
    if payload.username != "test" || payload.password != "test" {
        return Err(Error::LoginFail);
    }

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
    password: String
}