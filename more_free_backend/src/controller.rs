use axum::Json;
use crate::model::LoginInfo;
use axum::http::StatusCode;

pub async fn login_handler(Json(login_info): Json<LoginInfo>)
-> Result<Json<LoginInfo>, StatusCode> {

}

pub async fn get_info_handler() -> Result<Json<String>, StatusCode> {

}