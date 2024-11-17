#![allow(dead_code)]

use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub password: String
}

pub struct BirdInfo {
    pub name: String,
    pub password: String
}
