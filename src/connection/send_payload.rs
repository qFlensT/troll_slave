use std::{net::TcpStream, rc::Rc, cell::{RefCell}};

use tungstenite::{WebSocket, stream::MaybeTlsStream, Message};

use crate::models::payload::Payload;

pub struct SendPayload{
    sender: Rc<RefCell<WebSocket<MaybeTlsStream<TcpStream>>>>
}

impl SendPayload {
    pub fn new(sender: Rc<RefCell<WebSocket<MaybeTlsStream<TcpStream>>>>) -> Self{
        Self { sender }
    }

    pub fn send(&self, payload: Payload) -> Result<(), String>{
        self.sender.borrow_mut().write_message(Message::Text(payload.json())).map_err(|err|{err.to_string()})
    }
}