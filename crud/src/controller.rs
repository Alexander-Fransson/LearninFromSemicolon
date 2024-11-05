use axum::{extract::Path, http::StatusCode, Json};
use serde_json::Value;
use crate::model:: UserInfo;

pub async fn list_users() -> (StatusCode, Json<Value>) {
    // get users
}

pub async fn get_user_by_id(Path(id): Path<i32>) -> (StatusCode, Json<Value>) {
    // get user
}

pub async fn create_user(Json(user):Json<UserInfo>) -> StatusCode {
    // create user
}

pub async fn update_user(Path(id): Path<i32>, Json(user):Json<UserInfo>) -> StatusCode {
    // put user    
}

pub async fn delete_user(Path(id): Path<i32>) -> StatusCode {
    // delete user
}

