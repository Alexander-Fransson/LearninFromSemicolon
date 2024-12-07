use crate::{ctx::extractor::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct  Ticket2 {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
    ticket_store_2: Arc<Mutex<Vec<Option<Ticket2>>>>
}

// constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(
            Self {
                ticket_store: Arc::default(),
                ticket_store_2: Arc::default()
            }
        )
    }
}

// crud implementation

impl ModelController {

    pub async fn create_ticket_2(&self, ctx: Ctx, ticket_for_create: TicketForCreate) -> Result<Ticket2> {
        let mut store = self.ticket_store_2.lock().unwrap();
        let id = store.len() as i32;

        let ticket = Ticket2 {
            id,
            creator_id: ctx.user_id(),
            title: ticket_for_create.title
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets_2(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets: Vec<Ticket> = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket_2(&self, _ctx: Ctx, id: i32) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }

    pub async fn create_ticket(&self, ticket_for_create: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as i32;

        let ticket = Ticket {
            id,
            title: ticket_for_create.title
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets: Vec<Ticket> = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: i32) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }
}