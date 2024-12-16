use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlx::postgres::PgRow;
use sqlx::FromRow;


pub trait DbBmc {
    const TABLE: &'static str;
}

pub trait HasFields {
    fn get_fields() -> String;
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>);
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) 
-> Result<i64> where MC: DbBmc, E:HasFields {
    let db = mm.db();

    let (keys, values) = data.get_not_null_keys_and_values();
    
    let sql = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING id", MC::TABLE, keys.join(","), "'".to_string() + &values.join("','") + "'");

    let (id,) = sqlx::query_as::<_, (i64,)>(&sql)
    .bind(values)
    .fetch_one(db)
    .await?;

    Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id:i64) 
-> Result<E> where MC: DbBmc, E: for <'r> FromRow<'r, PgRow> + Unpin + Send + HasFields {
    let db = mm.db();
    let sql = format!("SELECT {} FROM {} WHERE id = $1",E::get_fields(), MC::TABLE);
    let entity = sqlx::query_as::<_, E>(&sql)
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(Error::EntityNotFound { entity: MC::TABLE, id })?;

    Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) 
-> Result<Vec<E>> where MC: DbBmc, E: for <'r> FromRow<'r, PgRow> + Unpin + Send + HasFields {
    let db = mm.db();

    let sql = format!("SELECT {} FROM {} ORDER BY id",E::get_fields(), MC::TABLE);
    let tasks = sqlx::query_as(&sql)
    .fetch_all(db)
    .await?;

    Ok(tasks)
}