use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use sqlx::query;
use sqlx::query_as;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Cat {
    id: i32,
    name: String,
    age: i32,
}

async fn seed_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // create cat table

    query("DROP TABLE IF EXISTS cats")
    .execute(pool)
    .await?;

    query(
        "CREATE TABLE IF NOT EXISTS cats (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            age INT NOT NULL
    )")
    .execute(pool)
    .await?;

    // seed cats table with some data
    query("INSERT INTO cats (name, age) VALUES ('Mittens', 3)")
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_cats(pool: &Pool<Postgres>) -> Result<Vec<Cat>, sqlx::Error> {
    let cats = query_as("SELECT id, name, age FROM cats")
    .fetch_all(pool)
    .await?;
    Ok(cats)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("CONNECTION_STRING").expect("FAILED TO GET CONNECTION_STRING");
    let connection_pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
    .expect("Failed to connect to Postgres");

    seed_db(&connection_pool).await.expect("Failed to seed db");

    let cats = get_cats(&connection_pool).await.expect("Failed to get cats");
    println!("{:?}", cats);

    println!("db_url: {}", db_url);
}
