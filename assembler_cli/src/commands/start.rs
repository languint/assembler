use std::sync::Arc;

use crate::cli;
use crate::ipc::routes::handshake::{HandshakePayload, HandshakeState};
use crate::ipc::routes::observation::{ObservationPayload, ObservationState};
use crate::ipc::schema::{GeneralIpcMessage, IpcSchema};
use crate::ipc::{self};

use crate::lua_mod::AssemblerConfig;
use tokio::sync::{Mutex, mpsc};

pub async fn start_command(config: &AssemblerConfig) -> Result<(), String> {
    ipc::process::start_factorio(config.ipc.factorio_port)?;

    let handshake_state = Arc::new(Mutex::new(HandshakeState::Init));

    let handshake_handler = {
        let handshake_state = handshake_state.clone();
        move |msg: String, tx: mpsc::UnboundedSender<String>| {
            let handshake_state = handshake_state.clone();
            async move {
                let state = handshake_state.lock().await;

                let msg_json: GeneralIpcMessage =
                    serde_json::from_str(&msg).map_err(|_| "Received invalid JSON".to_string())?;

                if msg_json.schema != IpcSchema::HANDSHAKE {
                    let err_msg = format!("Received invalid schema: {}!", msg_json.schema);
                    return Err(err_msg);
                }

                let msg_json: HandshakePayload = serde_json::from_str(&msg).map_err(|e| {
                    format!("Could not parse GeneralIpcMessage as HandshakePayload: {e}")
                })?;

                ipc::routes::handshake::handle_packet(&tx, state, msg_json)?;
                Ok(())
            }
        }
    };

    let (_tx, handshake_handle) = ipc::create_socket(
        config.ipc.handshake_port,
        config.ipc.factorio_port,
        "IPC-HANDSHAKE",
        handshake_handler,
    )
    .await?;

    let observation_state = Arc::new(Mutex::new(ObservationState::new()));

    let observation_handler = {
        let observation_state = observation_state.clone();
        move |msg: String, tx: mpsc::UnboundedSender<String>| {
            let observation_state = observation_state.clone();
            async move {
                let state = observation_state.lock().await;

                let msg_json: GeneralIpcMessage =
                    serde_json::from_str(&msg).map_err(|_| "Received invalid JSON".to_string())?;

                if msg_json.schema != IpcSchema::OBSERVATION {
                    let err_msg = format!("Received invalid schema: {}!", msg_json.schema);
                    return Err(err_msg);
                }

                let msg_json: ObservationPayload = serde_json::from_str(&msg).map_err(|e| {
                    format!("Could not parse GeneralIpcMessage as ObservationPayload: {e}")
                })?;

                ipc::routes::observation::handle_packet(&tx, state, msg_json)?;
                Ok(())
            }
        }
    };

    // Wait until handshake is completed to start other routes
    handshake_handle.await.unwrap()?;

    cli::log_header(
        "IPC",
        "Handshake complete, starting routes.",
        0,
        Some(cli::CLI_BLUE_HEADER),
    );

    let (observation_sender, observation_handle) = ipc::create_socket(
        config.ipc.observation_port,
        config.ipc.factorio_port,
        "OBSERVATION",
        observation_handler,
    )
    .await?;

    // Collect entity observations from "chunks" around (0,0)

    observation_handle.await.unwrap()?;

    Ok(())
}
