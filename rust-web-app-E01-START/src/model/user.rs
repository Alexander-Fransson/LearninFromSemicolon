use crate::ctx::{self, Ctx};
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;
use crate::Fields;
use super::base::HasFields;

#[derive(Clone, Debug, Serialize, FromRow, Fields)]
pub struct User {
    pub id: i64,
    pub username: String,
}
impl HasFields for User {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>) {
        let keys = vec!["id".to_string(), "username".to_string()];
        let values = vec![self.id.to_string(), self.username.to_string()];
        (keys, values)
    }
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
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
impl HasFields for UserForLogin {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>) {
        let mut keys = vec!["id".to_string(), "username".to_string(), "pwd_salt".to_string(), "pwd_token_salt".to_string()];
        let mut values = vec![self.id.to_string(), self.username.to_string()];

        if let Some(password) = &self.password {
            keys.push("password".to_string());
            values.push(password.clone());
        }

        (keys, values)
    }
}

#[derive(FromRow, Fields)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,    
    pub token_salt: Uuid,
}
impl HasFields for UserForAuth {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>) {
        let keys = vec!["id".to_string(), "username".to_string(), "token_salt".to_string()];
        let values = vec![self.id.to_string(), self.username.to_string(), self.token_salt.to_string()];
        (keys, values)
    }
}

pub trait UserBy: for <'r> FromRow<'r, PgRow> + Unpin + Send + Fields + HasFields {}

impl UserBy for User {}
impl UserBy for UserForLogin {} 
impl UserBy for UserForAuth {}

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "\"user\"";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E> where E: UserBy {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx, 
        mm: &ModelManager, 
        username: &str
    ) -> Result<Option<E>> where E: UserBy {
        // fetch user by username using sqlx
        let db = mm.db();
        let sql = format!("SELECT * FROM {} WHERE username = $1", UserBmc::TABLE);
        let user = sqlx::query_as::<_, E>(&sql)
        .bind(username)
        .fetch_optional(db)
        .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use serial_test::serial;
    use anyhow::{Result, Context};

    #[ignore]
    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";
        
        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
        .await?.context("should have user demo1")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}