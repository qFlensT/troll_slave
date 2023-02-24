use std::{collections::HashMap, thread};

use ws::{connect, CloseCode};

use crate::models::{commands::Command, connection::ConnectionSettings, payload::Payload};

use super::send_payload::SendPayload;

pub struct Connection{
    url: String, 
    handles: HashMap<Command, &'static dyn Fn(SendPayload, Option<String>)>,
    settings: ConnectionSettings
}

impl Connection {
    pub fn new(url: String, handles: HashMap<Command, &'static dyn Fn(SendPayload, Option<String>)>, settings: ConnectionSettings) -> Self{
        Self { url, handles, settings }
    }

    fn try_connect(&self) -> Result<(), String>{
        if let Ok(_) = connect(self.url.clone(), |sender|{
            move |msg: ws::Message| {
                if !msg.is_text(){
                    return sender.close(CloseCode::Normal)
                }
                println!("Got message!\nMessage: {}", msg.as_text().unwrap_or_default());
                println!("Transforming to payload struct...");

                let payload = Payload::from(msg.as_text().unwrap_or_default());

                println!("Got struct: {:?}", payload);

                for (k, v) in self.handles.clone(){
                    if payload.command == k{
                        v(SendPayload::new(sender.clone()), payload.payload.clone())
                    }
                }

                Ok(())
            }
        }){
            return Err("Connection Error".to_string())
        }
        Ok(())
    }

    pub fn run(&self){
        for _ in 1..self.settings.retry_count{
            match self.try_connect(){
                Ok(_) => {},
                Err(err) => {
                    println!("{err}");
                },
            };
            thread::sleep(self.settings.retry_delay);
        }
    }

    pub fn run_forever(&self){
        //Ignores `retry_count` from ConnectionSettings

        loop {
            match self.try_connect(){
                Ok(_) => {},
                Err(err) => {
                    println!("{err}");
                },
            };
            thread::sleep(self.settings.retry_delay);
        }
    }
}