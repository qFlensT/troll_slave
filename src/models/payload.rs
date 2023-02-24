use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::commands::Command;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Payload {
    pub command: Command,
    pub payload: Option<String>,
}

impl Default for Payload {
    fn default() -> Self {
        Self { command: Default::default(), payload: Default::default() }
    }
}

impl From<String> for Payload{
    fn from(value: String) -> Self {
        serde_json::from_str::<Payload>(&value).unwrap_or_default()
    }
}

impl From<&str> for Payload{
    fn from(value: &str) -> Self {
        serde_json::from_str::<Payload>(value).unwrap_or_default()
    }
}

impl Payload {
    pub fn new(command: Command, payload: Option<impl Serialize>) -> Self {
        let payload = match payload {
            Some(v) => Some(serde_json::to_string_pretty(&v).unwrap_or_default()),
            None => None,
        };

        Self { command, payload }
    }

    pub fn deserialize<E: Clone + DeserializeOwned>(&self) -> Result<E, String> {
        serde_json::from_str::<E>(
            &serde_json::to_string_pretty(&self.payload).map_err(|err| err.to_string())?,
        )
        .map_err(|err| err.to_string())
    }

    pub fn inner_json(&self) -> String {
        //Serializing inner T

        serde_json::to_string_pretty(&self.payload).unwrap_or_default()
    }

    pub fn json(&self) -> String{
        //Serializing self

        serde_json::to_string_pretty(&self).unwrap_or_default()
    }
}
