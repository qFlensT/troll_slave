use ws::Sender;

use crate::models::payload::Payload;

pub struct SendPayload{
    sender: Sender
}

impl SendPayload {
    pub fn new(sender: Sender) -> Self{
        Self { sender }
    }

    pub fn send(&self, payload: Payload) -> Result<(), String>{
        self.sender.send(payload.json()).map_err(|err|{err.to_string()})
    }
}