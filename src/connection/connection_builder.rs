use std::{collections::HashMap, time::Duration};

use crate::models::payload::Payload;
use crate::models::{commands::Command, connection::ConnectionSettings};

use super::connection::Connection;
use super::send_payload::SendPayload;

#[derive(Clone)]
pub struct ConnectionBuilder {
    url: String,
    init_payloads: Vec<Payload>,
    handles: HashMap<Command, &'static dyn Fn(SendPayload, Option<String>)>,
    settings: ConnectionSettings,
}

impl ConnectionBuilder {
    pub fn builder(url: String) -> Self {
        Self {
            url,
            init_payloads: vec![],
            handles: HashMap::new(),
            settings: ConnectionSettings::default()
        }
    }

    pub fn retry_count(&self, count: i32) -> Self{
        let mut this = self.clone();
        this.settings.retry_count = count;

        this.clone()
    }

    pub fn retry_delay(&self, delay: Duration) -> Self{
        let mut this = self.clone();
        this.settings.retry_delay = delay;

        this.clone()
    }

    pub fn add_handle(&mut self, command: Command, func: &'static dyn Fn(SendPayload, Option<String>)) -> Self{
        self.handles.insert(command, func);

        self.clone()
    }

    pub fn add_init_payload(&mut self, payload: Payload) -> Self{
        self.init_payloads.push(payload);

        self.clone()
    }

    pub fn build(&self) -> Result<Connection, String> {
        Ok(Connection::new(
            self.url.clone(),
            self.init_payloads.clone(),
            self.handles.clone(),
            self.settings
        ))
    }
}
