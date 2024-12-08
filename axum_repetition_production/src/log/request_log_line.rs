use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ctx::extractor::Ctx, log::request_log_line, ClientError, Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use tower_cookies::service;
use uuid::Uuid;

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>

) -> Result<()> {

    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
    .ok()
    .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let request_log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        req_path: uri.to_string(),
        req_method: req_method.to_string(),
        user_id: ctx.map(|c| c.user_id),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    println!("REQUEST_LOG_LINE \n{}", json!(request_log_line));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
pub struct RequestLogLine {
    uuid: String,
    timestamp: String,
    user_id: Option<i32>,
    req_path: String,
    req_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>
}