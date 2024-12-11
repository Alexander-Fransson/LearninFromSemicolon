use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>
}