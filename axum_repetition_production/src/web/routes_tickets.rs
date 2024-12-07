use crate::ctx::extractor::Ctx;
use crate::model::ticket::{ModelController, Ticket, Ticket2, TicketForCreate};
use axum::extract::{State, Path};
use axum::routing::{delete, post};
use axum::Json;
use axum::Router;

pub fn routes_tickets_2(mc:ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_ticket_2).get(list_tickets_2))
    .route("/tickets/:id", delete(delete_ticket_2))
    .with_state(mc)
}

async fn create_ticket_2(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
    Json(ticket_for_create): Json<TicketForCreate>
) -> Json<Ticket2> {
    let ticket = model_controller.create_ticket_2(ctx, ticket_for_create).await.unwrap();
    Json(ticket)
}


async fn list_tickets_2(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
) -> Json<Vec<Ticket>> {
    let tickets = model_controller.list_tickets_2(ctx).await.unwrap();
    Json(tickets)
}

async fn delete_ticket_2(
    State(model_controller): State<ModelController>,
    ctx: Ctx, 
    Path(id): Path<i32>
) -> Json<Ticket> {
    let ticket = model_controller.delete_ticket_2(ctx,id).await.unwrap();
    Json(ticket)
}

pub fn routes_tickets(model_controller: ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_ticket).get(list_tickets))
    .route("/tickets/:id", delete(delete_ticket))
    .with_state(model_controller)
}

async fn create_ticket(
    State(model_controller): State<ModelController>, 
    Json(ticket_for_create): Json<TicketForCreate>
) -> Json<Ticket> {
    let ticket = model_controller.create_ticket(ticket_for_create).await.unwrap();
    Json(ticket)
}

async fn list_tickets(
    State(model_controller): State<ModelController>
) -> Json<Vec<Ticket>> {
    let tickets = model_controller.list_tickets().await.unwrap();
    Json(tickets)
}

async fn delete_ticket(
    State(model_controller): State<ModelController>, 
    Path(id): Path<i32>
) -> Json<Ticket> {
    let ticket = model_controller.delete_ticket(id).await.unwrap();
    Json(ticket)
}