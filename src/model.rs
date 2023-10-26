use serde::{Serialize, Deserialize};

use crate::{Error, Result};

use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Debug)]
pub struct Ticket {
    pub id: u16,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, payload: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u16;

        let ticket = Ticket {
            title: payload.title,
            id,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let stores = self.tickets_store.lock().unwrap();
        let tickets = stores.iter().filter_map(|f| f.clone()).collect();
        Ok(tickets)
    }

    pub async fn remove_ticket(&self, id: u16) -> Result<Ticket> {
        let mut stores = self.tickets_store.lock().unwrap();

        let ticket = stores.get_mut(id as usize).and_then(|f| f.take());

        ticket.ok_or(Error::DeleteTicketFailed { id: id as usize })
    }
}
