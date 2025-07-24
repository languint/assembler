use serde::{Deserialize, Serialize};

use crate::ipc::schema::IpcSchema;

#[derive(Debug)]
pub enum HandshakeState {
    Init,
    Acked,
    Ready,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub enum HandshakePayloadState {
    ACK,
    OK,
}

#[derive(Deserialize, Serialize)]
pub struct HandshakePayloadData {
    pub state: HandshakePayloadState,
}

#[derive(Deserialize, Serialize)]
pub struct HandshakePayload {
    pub schema: IpcSchema,
    pub data: HandshakePayloadData,
}

pub const HANDSHAKE_OK_MESSAGE: HandshakePayload = HandshakePayload {
    schema: IpcSchema::HANDSHAKE,
    data: HandshakePayloadData {
        state: HandshakePayloadState::OK,
    },
};

pub const HANDSHAKE_ACK_MESSAGE: HandshakePayload = HandshakePayload {
    schema: IpcSchema::HANDSHAKE,
    data: HandshakePayloadData {
        state: HandshakePayloadState::ACK,
    },
};
