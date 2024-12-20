use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;
use fields_macro::Fields;

pub trait Fields {
	fn struct_name(&self) -> &'static str;
	fn fields(&self) -> Vec<&'static str>;
}

#[derive(Clone, Debug, Serialize, FromRow, Fields)]
pub struct User {
    pub id: i64,
    pub username: String,
}


#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

struct UserForInsert {
    pub username: String,
}

#[derive(FromRow, Clone, Debug, Fields)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,
    pub password: Option<String>,
    pub pwd_salt: Uuid,
    pub pwd_token_salt: Uuid,
}

#[derive(FromRow, Fields)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,    
    pub pwd_salt: Uuid,
}

pub trait UserBy: for <'r> FromRow<'r, PgRow> + Unpin + Send + Fields {}

impl UserBy for User {}
impl UserBy for UserForLogin {} 
impl UserBy for UserForAuth {}