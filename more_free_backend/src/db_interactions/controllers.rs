#![allow(dead_code)]

use axum::Extension;
use axum::http::StatusCode;
use axum::Json;
use crate::db_interactions::models::{
    Bird,
    BirdInfo
};
use crate::db_interactions::service::Services;

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

pub async fn get_bird(service:&Extension<Services>, id: i32) -> Result<Json<Bird>, StatusCode> {
    if let Ok(bird) = service.get_bird(id).await {
        Ok(Json(bird))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn create_bird(service:&Extension<Services>, bird: BirdInfo) -> Result<Json<Bird>, StatusCode> {
    if let Ok(bird) = service.create_bird(bird).await {
        Ok(Json(bird))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn delete_bird(service:&Extension<Services>, id: i32) -> Result<StatusCode, StatusCode> {
    if let Ok(_) = service.delete_bird(id).await {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn update_bird(service:&Extension<Services>, id: i32, bird: BirdInfo) -> Result<Json<Bird>, StatusCode> {
    if let Ok(bird) = service.update_bird(id, bird).await {
        Ok(Json(bird))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}