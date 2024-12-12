mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

use crate::model::ModelManager;

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