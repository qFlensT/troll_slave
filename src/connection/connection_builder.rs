use std::{collections::HashMap, time::Duration};

use crate::models::{commands::Command, connection::ConnectionSettings};

use super::connection::Connection;
use super::send_payload::SendPayload;

#[derive(Clone)]
pub struct ConnectionBuilder {
    url: Option<String>,
    // handles: HashMap<Command, &'static dyn Fn(&'static dyn Fn(Payload), Option<Payload>) -> ()>,
    handles: HashMap<Command, &'static dyn Fn(SendPayload, Option<String>)>,
    settings: ConnectionSettings,
}

impl ConnectionBuilder {
    pub fn builder() -> Self {
        Self {
            url: None,
            handles: HashMap::new(),
            settings: ConnectionSettings::default()
        }
    }

    pub fn url(&self, url: String) -> Self {
        let mut this = self.clone();
        this.url = Some(url);
        
        this.clone()
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

    pub fn build(&self) -> Result<Connection, String> {
        if self.url.is_none() {
            return Err("ConnectionBuilder Error - Url is not set".to_string());
        }

        Ok(Connection::new(
            self.url.clone().unwrap(),
            self.handles.clone(),
            self.settings
        ))
    }
}
