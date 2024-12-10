use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

type DB = Pool<Postgres>;

// Note: hardcode to prevent deployed system db update
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

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
    
    Ok(())
}

async fn pexec(db: &DB, file: &str) -> Result<(), sqlx::Error> {
    let content = fs::read_to_string(file)?;
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url : &str) -> Result<DB, sqlx::Error> {
    PgPoolOptions::new()
    .max_connections(1)
    .acquire_timeout(Duration::from_millis(500))
    .connect(db_con_url)
    .await
}