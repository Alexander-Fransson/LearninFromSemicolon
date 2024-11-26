// ctx stands for context
// here we will build an extractor

#![allow(dead_code)]

#[derive(Clone)]
pub struct Ctx {
    pub ticket_id: i32
}

// constructor
impl Ctx {
    pub fn new(ticket_id: i32) -> Self {
        Self {
            ticket_id
        }
    }
}

// property accessors
impl Ctx {
    pub fn ticket_id(&self) -> i32 {
        self.ticket_id
    }
}