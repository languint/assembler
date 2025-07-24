use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub enum IpcSchema {
    HANDSHAKE,
    OBSERVATION
}

impl From<&str> for IpcSchema {
    fn from(value: &str) -> Self {
        match value {
            "HANDSHAKE" => IpcSchema::HANDSHAKE,
            "OBSERVATION" => IpcSchema::OBSERVATION,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for IpcSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcSchema::HANDSHAKE => write!(f, "<handshake>"),
            IpcSchema::OBSERVATION => write!(f, "<observation>")
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GeneralIpcMessage {
    pub schema: IpcSchema,
}
