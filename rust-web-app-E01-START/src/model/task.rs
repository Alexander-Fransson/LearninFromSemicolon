use std::collections::HashMap;
use std::vec;

use crate::ctx::Ctx;
use crate::model::base;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use super::base::DbBmc;
use super::base::HasFields;
use super::error::Error;
use crate::Fields;

#[derive(Clone, Debug, Serialize, FromRow, Fields)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

impl HasFields for Task {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>) {
        (vec!["title".to_string()], vec![self.title.clone()])
    }
}

#[derive(Deserialize, Fields)]
pub struct TaskForCreate {
    pub title: String
}
impl HasFields for TaskForCreate {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>) {
        (vec!["title".to_string()], vec![self.title.clone()])
    }
}

#[derive(Deserialize, Fields)]
pub struct TaskForUpdate {
    pub title: Option<String>
}

impl HasFields for TaskForUpdate {
    fn get_not_null_keys_and_values(&self) -> (Vec<String>, Vec<String>) {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        if let Some(title) = &self.title {
            keys.push("title".to_string());
            values.push(title.clone());
        }
        (keys, values)
    }
}

pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn get(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64
    ) -> Result<Task> {

        base::get::<Self, Task>(ctx, mm, id).await

        // // before we hade the Db bmc trait
        // let db = mm.db();
        // let task = sqlx::query_as::<_, Task>("SELECT id, title FROM task WHERE id = $1")
        // .bind(id)
        // .fetch_optional(db)
        // .await?
        // .ok_or(Error::EntityNotFound { entity: "task", id })?;

        // Ok(task)
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        data: TaskForUpdate
    ) -> Result<()> {
        base::update::<Self, TaskForUpdate>(ctx, mm, id, data).await
    }

    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64
    ) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate
    ) -> Result<i64> {

        base::create::<Self, TaskForCreate>(ctx, mm, task_c).await

        // let db = mm.db();
        // let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) VALUES ($1) RETURNING id")
        // .bind(task_c.title)
        // .fetch_one(db)
        // .await?;

        // Ok(id)
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        
        base::list::<Self, Task>(ctx, mm).await

        // //alternatively
        // let db = mm.db();

        // let tasks = sqlx::query_as("SELECT * FROM task ORDER BY id")
        // .fetch_all(db)
        // .await?;

        // Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils;
    use serial_test::serial;
    use crate::model::error::Error;

    use super::*;
    use anyhow::{Ok, Result};

    #[serial]
    #[ignore]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        TaskBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    //#[ignore]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        let task_u = TaskForUpdate {
            title: Some("updated title".to_string())
        };
        TaskBmc::update(&ctx, &mm, id, task_u).await?;

        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, "updated title");

        TaskBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[ignore]
    #[tokio::test]
    async fn test_get_error_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = 100;
        let res = TaskBmc::get(&ctx, &mm, id).await;
        assert!(res.is_err());
        assert!(
            matches!(
                res.unwrap_err(), Error::EntityNotFound { 
                    entity: "task",
                    id
                }
            )
        );
        Ok(())
    }

    #[serial]
    #[ignore]
    #[tokio::test]
    async fn test_delete_error_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = 10000;
        let res = TaskBmc::delete(&ctx, &mm, id).await;
        assert!(res.is_err());
        assert!(
            matches!(
                res.unwrap_err(), Error::EntityNotFound { 
                    entity: "task",
                    id
                }
            )
        );
        Ok(())
    }

    #[serial]
    #[ignore]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let titles = ["title 1", "title 2", "title 3"];
        _dev_utils::seed_tasks(&ctx, &mm, &titles).await?;
        let tasks = TaskBmc::list(&ctx, &mm).await?;

        println!("tasks: {:#?}", tasks);
        
        let tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|t| titles.contains(&t.title.as_str()))
        .collect();

        assert_eq!(tasks.len(), titles.len());

        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }
}