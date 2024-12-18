use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

use super::base::HasFields;


pub trait HasFiledsForUser {

}



#[derive(Clone, Debug, Serialize, FromRow, HasFiledsForUser)]
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

#[derive(FromRow, Clone, Debug, HasFiledsForUser)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,
    pub password: Option<String>,
    pub pwd_salt: Uuid,
    pub pwd_token_salt: Uuid,
}

#[derive(HasFiledsForUser)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,    
    pub pwd_salt: Uuid,
}

pub trait UserBy: for <'r> FromRow<'r, PgRow> + Unpin + Send + HasFiledsForUser {}

impl UserBy for User {}
impl UserBy for UserForLogin {} 
impl UserBy for UserForAuth {}