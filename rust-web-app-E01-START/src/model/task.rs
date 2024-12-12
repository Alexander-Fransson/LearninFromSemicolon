use crate::ctx::Ctx;
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

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate
    ) -> Result<i64> {
        let db = mm.db();
        let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) VALUES ($1) RETURNING id")
        .bind(task_c.title)
        .fetch_one(db)
        .await?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let (title,): (String,) = sqlx::query_as("SELECT title FROM task WHERE id = $1")
        .bind(id)
        .fetch_one(mm.db())
        .await?;
        
        assert_eq!(title, fx_title);

        let count = sqlx::query("DELETE FROM task WHERE id = $1")
        .bind(id)
        .execute(mm.db())
        .await?
        .rows_affected();

        assert_eq!(count, 1, "deleted more or less than 1 row");

        Ok(())
    }
}