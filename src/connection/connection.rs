use std::{collections::HashMap, thread, rc::Rc, cell::RefCell};
use tungstenite::connect;
use url::Url;

use crate::models::{commands::Command, connection::ConnectionSettings, payload::Payload};

use super::send_payload::SendPayload;

pub struct Connection {
    url: String,
    init_payloads: Vec<Payload>,
    handles: HashMap<Command, &'static dyn Fn(SendPayload, Option<String>)>,
    settings: ConnectionSettings,
}

impl Connection {
    pub fn new(
        url: String,
        init_payloads: Vec<Payload>,
        handles: HashMap<Command, &'static dyn Fn(SendPayload, Option<String>)>,
        settings: ConnectionSettings,
    ) -> Self {
        Self {
            url,
            init_payloads,
            handles,
            settings,
        }
    }

    fn try_connect(&self) -> Result<(), String> {
        let (socket, _) = connect(Url::parse(&self.url).map_err(|err| err.to_string())?)
            .map_err(|err| err.to_string())?;

        let sender = Rc::new(RefCell::new(socket));

        for payload in self.init_payloads.clone(){
            let send = SendPayload::new(sender.clone());
            if let Err(_) = send.send(payload){
                continue;
            }
        }

        loop{
            let msg = sender.borrow_mut().read_message().map_err(|err|{err.to_string()})?;
            let payload = Payload::from(match msg.into_text(){
                Ok(v) => v,
                Err(_) => {String::new()},
            });

            for (k, v) in self.handles.clone(){
                if payload.command == k{
                    v(SendPayload::new(sender.clone()), payload.payload.clone())
                }
            }
        }
    }

    pub fn run(&self) {
        for _ in 1..self.settings.retry_count {
            match self.try_connect() {
                Ok(_) => {}
                Err(_) => {
                }
            };
            thread::sleep(self.settings.retry_delay);
        }
    }

    pub fn run_forever(&self) {
        //Ignores `retry_count` from ConnectionSettings

        loop {
            match self.try_connect() {
                Ok(_) => {}
                Err(_) => {
                }
            };
            thread::sleep(self.settings.retry_delay);
        }
    }
}
