#![allow(dead_code)]

use axum::{extract::State, Json};
use axum::extract::Path;
use axum::routing::Router;
use axum::routing::{delete,post};

use crate::Result;
use super::model::{ModelController, Ticket, TicketForCreate};

pub fn routes_tickets(mc:ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_ticket).get(list_tickets))
    .route("/tickets/:id", delete(delete_ticket))
    .with_state(mc)
}

// region -- rest handler

async fn create_ticket(
    State(model_controller): State<ModelController>, 
    Json(ticket_for_create): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = model_controller.create_ticket(ticket_for_create).await?;
    Ok(Json(ticket))
}

async fn list_tickets(
    State(model_controller): State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = model_controller.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(model_controller): State<ModelController>, 
    Path(id): Path<i32>
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = model_controller.delete_ticket(id).await?;
    Ok(Json(ticket))
}

// endregion -- rest handler