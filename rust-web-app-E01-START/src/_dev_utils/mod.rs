mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

/// initialize environment for local development
/// (for early development, will be called from main)
pub async fn dev_init() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("Initializing dev environment");

        dev_db::init_dev_db().await.unwrap();
    }).await;
}