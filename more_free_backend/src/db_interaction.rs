#![allow(dead_code)]

use axum::Extension;
use axum::http::StatusCode;
use axum::Json;
use sqlx::{PgPool, Error, query_as, query};
use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::FromRow;
use dotenv::dotenv;
use std::env;

#[derive(Debug, FromRow)]
pub struct Bird {
    id: i32,
    name: String,
    password: String
}

pub struct Services {
    pool: PgPool
}

pub fn generate_drop_string(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {}; ", table_name)
}

impl Services { // I could propbably implement different things in diferent files
    pub async fn new() -> Result<Self, Error> {
        dotenv().ok();

        let db_url = env::var("CONNECTION_STRING")
        .expect("CONNECTION_STRING must be set");

        let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url) 
        .await?;
        Ok(Self { pool: connection_pool })
    }

    pub async fn seed_birds(&self) -> Result<(), sqlx::Error> {
        let pool = &self.pool;

        let drop_string = generate_drop_string("birds");

        query(&drop_string)
        .execute(pool)
        .await?;

        query("CREATE TABLE IF NOT EXISTS birds (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL
        )")
        .execute(pool)
        .await?;

        query("INSERT INTO birds (name, password) VALUES ('bird', 'bro')")
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_birds(&self) -> Result<Vec<Bird>, sqlx::Error> {
        let pool = &self.pool;
        let birds = query_as::<_, Bird>("SELECT * FROM birds")
        .fetch_all(pool)
        .await?;
        Ok(birds)
    }
}

pub async fn seed_birds(service:&Extension<Services>) {
    match service.seed_birds().await {
        Ok(_) => println!("seeded"),
        Err(err) => println!("Error: {}", err)
    }
}

pub async fn get_birds(service:&Extension<Services>) -> Result<Json<Vec<Bird>>, StatusCode> {
    if let Ok(birds) = service.get_birds().await {
        Ok(Json(birds))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}