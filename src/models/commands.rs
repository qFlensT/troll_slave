use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Command{
    CommandIdle,
    CommandCursorReverseOn,
    CommandCursorReverseOff,
    CommandCursorReverseStatus,
    ResponseCursorReverseStatus,
}

impl Default for Command {
    fn default() -> Self {
        Command::CommandIdle
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CommandCursorReverseOn" => Ok(Command::CommandCursorReverseOn),
            "CommandCursorReverseOff" => Ok(Command::CommandCursorReverseOff),
            "CommandCursorReverseStatus" => Ok(Command::CommandCursorReverseStatus),
            _ => Err(())
        }
    }
}