#![allow(dead_code)]

use sqlx::{PgPool, Error, query_as, query};
use sqlx::postgres::PgPoolOptions;
use chrono::{Utc,Duration};
use axum::Json;
use axum::http::StatusCode;
use jsonwebtoken::{encode, Header, EncodingKey};
use dotenv::dotenv;
use std::env;
use crate::model::{Claims, LoginResponse};
use crate::db_interactions::models::{
    Bird,
    BirdInfo
};

pub fn generate_drop_string(table_name: &str) -> String {
    format!("DROP TABLE IF EXISTS {}; ", table_name)
}

pub struct Services {
    pool: PgPool
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

    async fn is_bird_valid(&self, name: &str, password: &str) -> bool {
        let pool = &self.pool;
        let bird = query_as::<_, Bird>("SELECT * FROM birds WHERE name = $1 AND password = $2")
        .bind(name)
        .bind(password)
        .fetch_one(pool)
        .await;

        match bird {
            Ok(_bird) => true,
            Err(_) => false
        }
    }

    pub async fn login_as_bird(&self, name: &str, password: &str) -> Result<Json<LoginResponse>, StatusCode> {
        let is_valid = self.is_bird_valid(name, password).await;

        if is_valid {
            dotenv().ok();

            let claim = Claims {
                sub: name.to_string(),
                exp: (Utc::now() + Duration::days(1)).timestamp() as usize
            };

            let token = match encode(
                &Header::default(), 
                &claim,
                &EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_ref())
            ) {
                Ok(token) => token,
                Err(err) => {
                    println!("Error generating token: {}", err);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                },
            };

            Ok(Json(LoginResponse{token}))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
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

    pub async fn get_birds(&self) -> Result<Vec<Bird>, Error> {
        let pool = &self.pool;
        let birds = query_as::<_, Bird>("SELECT * FROM birds")
        .fetch_all(pool)
        .await?;
        Ok(birds)
    }

    pub async fn get_bird(&self, id: i32) -> Result<Bird, Error> {
        let pool = &self.pool;
        let bird = query_as::<_, Bird>("SELECT * FROM birds WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(bird)
    }

    pub async fn create_bird(&self, bird: BirdInfo) -> Result<Bird, Error> {
        let pool = &self.pool;
        let new_bird = query_as("INSERT INTO birds (name, password) VALUES ($1, $2) RETURNING *")
        .bind(bird.name)
        .bind(bird.password)
        .fetch_one(pool)
        .await?;
        Ok(new_bird)
    }

    pub async fn delete_bird(&self, id: i32) -> Result<(), Error> {
        let pool = &self.pool;
        query("DELETE FROM birds WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_bird(&self, id: i32, bird: BirdInfo) -> Result<Bird, Error> {
        let pool = &self.pool;
        let updated_bird =query_as("UPDATE birds SET name = $1, password = $2 WHERE id = $3 RETURNING *")
        .bind(bird.name)
        .bind(bird.password)
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(updated_bird)
    }
}
