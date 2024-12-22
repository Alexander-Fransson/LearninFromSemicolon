use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use tracing_subscriber::fmt::format;
use crate::Fields;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub trait HasFields {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>);
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id:i64, data: E) 
-> Result<()> where MC: DbBmc, E:Fields + HasFields {
    let db = mm.db();

    let (keys, values) = data.get_not_null_keys_and_values();

    // create sql string
    let mut columns_set_string = String::new();
    for (index, value) in values.iter().enumerate() {
        let thing_to_add = format!("{} = '{}'",keys[index], value);
        columns_set_string.push_str(&thing_to_add);
        if index != values.len() - 1 {
            columns_set_string.push_str(", ");
        }
    }

    println!("keys: {:?}\n, values: {:?}\n, columns_set_string: {}", data.fields(), values, columns_set_string);

    let sql = format!("UPDATE {} SET {} WHERE id = $1 RETURNING id", MC::TABLE, columns_set_string);// this would only make sense of string type params
    let count = sqlx::query(&sql)
    .bind(id)
    .execute(db)
    .await?
    .rows_affected();

    if count == 0 {
        return Err(Error::EntityNotFound { entity: MC::TABLE, id });
    } else {
        Ok(())   
    }
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) 
-> Result<()> where MC: DbBmc {
    let db = mm.db();
    let count = sqlx::query("DELETE FROM task WHERE id = $1")
    .bind(id)
    .execute(db)
    .await?
    .rows_affected();

    if count == 0 {
        return Err(Error::EntityNotFound { entity: MC::TABLE, id });
    } else {
        Ok(())   
    }
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) 
-> Result<i64> where MC: DbBmc, E:Fields + HasFields {
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
-> Result<E> where MC: DbBmc, E: for <'r> FromRow<'r, PgRow> + Unpin + Send + Fields + HasFields {
    let db = mm.db();
    let sql = format!("SELECT {} FROM {} WHERE id = $1",E::get_fields().join(","), MC::TABLE);
    let entity = sqlx::query_as::<_, E>(&sql)
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(Error::EntityNotFound { entity: MC::TABLE, id })?;

    Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) 
-> Result<Vec<E>> where MC: DbBmc, E: for <'r> FromRow<'r, PgRow> + Unpin + Send + Fields + HasFields {
    let db = mm.db();

    let sql = format!("SELECT {} FROM {} ORDER BY id",E::get_fields().join(","), MC::TABLE);
    let tasks = sqlx::query_as(&sql)
    .fetch_all(db)
    .await?;

    Ok(tasks)
}