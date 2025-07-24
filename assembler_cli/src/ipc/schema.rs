use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub enum IpcSchema {
    HANDSHAKE,
}

impl From<&str> for IpcSchema {
    fn from(value: &str) -> Self {
        match value {
            "HANDSHAKE" => IpcSchema::HANDSHAKE,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for IpcSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcSchema::HANDSHAKE => write!(f, "<handshake>"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GeneralIpcMessage {
    pub schema: IpcSchema,
}
