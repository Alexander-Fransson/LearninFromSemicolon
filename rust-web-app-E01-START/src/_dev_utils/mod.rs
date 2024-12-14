mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

use crate::model::ModelManager;
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::{ctx, model};

/// initialize environment for local development
/// (for early development, will be called from main)
pub async fn dev_init() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("Initializing dev environment");

        dev_db::init_dev_db().await.unwrap();
    }).await;
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    
    let mm = INIT.get_or_init(|| async {
        dev_init().await;
        ModelManager::new().await.unwrap()
    }).await;

    mm.clone()
}

pub async fn seed_tasks(
    ctx: &ctx::Ctx,
    mm: &ModelManager,
    titles: &[&str]
) ->model::Result<Vec<Task>> {
    let mut tasks:Vec<Task> = Vec::new();

    for title in titles {
        let id = TaskBmc::create(
            ctx, 
            mm, 
            TaskForCreate { 
                title: title.to_string() 
            }
        )
        .await?;

        let task = TaskBmc::get(ctx, mm, id).await?;
        tasks.push(task);
    }

    Ok(tasks)
}