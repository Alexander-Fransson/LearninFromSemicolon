use axum::Json;
use tower_cookies::{Cookies, Cookie};
use serde_json::{Value, json};
use axum::routing::post;
use axum::Router;
use crate::{Error, Result};
use super::AUTH_TOKEN;

use crate::model::login::LoginPayload;

pub fn routes_login() -> Router {
    Router::new()
    .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // todo implement real db auth logic
    if payload.username != "test" || payload.pwd != "test" {
        return Err(Error::LoginFail);
    }

    // FIXME: implement real auth logic
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // generate success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    
    Ok(body)
}