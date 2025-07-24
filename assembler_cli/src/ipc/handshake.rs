use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{cli, ipc::schema::IpcSchema};

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


pub fn handshake_handler(tx_recv: &mpsc::UnboundedSender<String>, mut state: tokio::sync::MutexGuard<'_, HandshakeState>, msg_json: HandshakePayload) -> Result<(), String> {
    Ok(match &*state {
        HandshakeState::Init => {
            if msg_json.data.state == HandshakePayloadState::ACK {
                cli::log_header(
                    "IPC-HANDSHAKE",
                    "Recieved ACK, sending ACK",
                    0,
                    Some(cli::CLI_PURPLE_HEADER),
                );

                let ack_message =
                    serde_json::to_string(&HANDSHAKE_ACK_MESSAGE).map_err(|e| {
                        format!("Failed to serialize HANDSHAKE_ACK_MESSAGE: {e}")
                    })?;

                tx_recv
                    .send(ack_message)
                    .map_err(|e| format!("Failed to send HANDSHAKE_ACK_MESSAGE {e}"))?;

                *state = HandshakeState::Acked;
            }
        }
        HandshakeState::Acked => {
            if msg_json.data.state == HandshakePayloadState::OK {
                cli::log_header(
                    "IPC-HANDSHAKE",
                    "Recieved OK, sending OK",
                    0,
                    Some(cli::CLI_PURPLE_HEADER),
                );

                let ok_message =
                    serde_json::to_string(&HANDSHAKE_OK_MESSAGE).map_err(|e| {
                        format!("Failed to serialize HANDSHAKE_OK_MESSAGE: {e}")
                    })?;

                tx_recv
                    .send(ok_message)
                    .map_err(|e| format!("Failed to send HANDSHAKE_OK_MESSAGE {e}"))?;

                *state = HandshakeState::Ready;
                cli::log_header(
                    "IPC-HANDSHAKE",
                    "OK-OK, READY!",
                    0,
                    Some(cli::CLI_PURPLE_HEADER),
                );
            }
        }
        HandshakeState::Ready => {}
    })
}