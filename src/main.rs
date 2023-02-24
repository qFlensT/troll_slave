use connection::connection_builder::ConnectionBuilder;
use models::{payload::Payload, commands::Command};
use usecases::cursor_reverse::{cursor_reverse_on, cursor_reverse_off, cursor_reverse_status};

pub mod connection;
pub mod models;
pub mod usecases;

fn main() {
    let connection_builder = ConnectionBuilder::builder()
        .url("ws://127.0.0.1:8765".to_string())
        .add_handle(Command::CommandCursorReverseOn, &|_, _|{cursor_reverse_on();})
        .add_handle(Command::CommandCursorReverseOff, &|_, _|{cursor_reverse_off();})
        .add_handle(Command::CommandCursorReverseStatus, &|sender, _|{
            sender.send(Payload::new(Command::ResponseCursorReverseStatus, Some(cursor_reverse_status()))).unwrap();
        });
    //     .add_handle(Command::CommandCursorReverseStatus, &|_, _|{});

    let connection = connection_builder.build().unwrap();

    connection.run_forever();

    // println!("{}", Payload::new(Command::CommandCursorReverseOn, None::<String>).json());
}
