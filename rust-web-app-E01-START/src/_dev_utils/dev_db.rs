use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::{ctx::{self, Ctx}, model::{user::{User, UserBmc}, ModelManager}};

type DB = Pool<Postgres>;

const DEMO_PWD: &str = "welcome";

// Note: hardcode to prevent deployed system db update
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost:5433/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5433/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev-initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev-initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    // create the app_db & app_user with the postgres user
    {
        // its own scope becouse variables should not be accessed anyware else
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
    .filter_map(|entry| entry.ok().map(|e| e.path()))
    .collect();

    paths.sort();
    info!("Found {} sql files", paths.len());

    // sql execute each file

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/"); // to make it work on windows
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }

    // init model layer
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    // set demo pwd
    let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1").await?.unwrap();
    UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD).await?;
    info!("init dev db and set demo pwd");

    Ok(())
}

async fn pexec(db: &DB, file: &str) -> Result<(), sqlx::Error> {
    info!("Executing {}", file);

    // let experiment =  fs::read_to_string("sql/dev-initial/00-recreate-db.sql");
    // info!("{}", experiment.unwrap());

    let content = fs::read_to_string(file)?;
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url : &str) -> Result<DB, sqlx::Error> {
    info!("Connecting to {}", db_con_url);
    PgPoolOptions::new()
    .max_connections(1)
    .acquire_timeout(Duration::from_millis(500))
    .connect(db_con_url)
    .await
}