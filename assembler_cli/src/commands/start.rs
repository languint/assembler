use std::process::{Command, Stdio};
use std::sync::Arc;

use crate::ipc::Ipc;
use crate::ipc::handshake::{
    HANDSHAKE_ACK_MESSAGE, HANDSHAKE_OK_MESSAGE, HandshakePayload, HandshakePayloadState,
    HandshakeState,
};
use crate::ipc::schema::{GeneralIpcMessage, IpcSchema};

use crate::cli;
use crate::lua_mod::AssemblerConfig;
use tokio::sync::{Mutex, mpsc};

fn start_factorio(port: u32) -> Result<(), String> {
    let factorio_steam_id = 427520;

    let factorio_pid = Command::new("steam")
        .arg(format!(
            "steam://run/{}//--enable-lua-udp%20{}",
            factorio_steam_id, port
        ))
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start Factorio: {}", e))?
        .id();

    cli::log_header(
        "PEX",
        format!(
            "Factorio started with PID `{}`, and port `{}`",
            factorio_pid, port
        )
        .as_str(),
        0,
        Some(cli::CLI_GREEN_HEADER),
    );
    Ok(())
}

pub async fn start_command(config: &AssemblerConfig) -> Result<(), String> {
    let handshake_ipc: Ipc = Ipc::new(config.ipc.handshake_port, config.ipc.factorio_port).await?;

    start_factorio(config.ipc.factorio_port)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let handshake_state = Arc::new(Mutex::new(HandshakeState::Init));
    let handshake_state_recv = handshake_state.clone();
    let tx_recv = tx.clone();

    let ipc_recv = handshake_ipc.clone();
    let recv_task = tokio::spawn(async move {
        loop {
            let msg = ipc_recv.receive().await?;

            let mut state = handshake_state_recv.lock().await;

            let msg_json: GeneralIpcMessage =
                serde_json::from_str(msg.as_str()).map_err(|_| "Recieved invalid JSON")?;

            if msg_json.schema != IpcSchema::HANDSHAKE {
                cli::log_header(
                    "IPC-HANDSHAKE",
                    format!("Recieved invalid schema: {}!", msg_json.schema).as_str(),
                    0,
                    Some(cli::CLI_RED_HEADER),
                );

                return Err("Recieved invalid schema!".to_string());
            }

            let msg_json: HandshakePayload = serde_json::from_str(msg.as_str()).map_err(|e| {
                format!("Could not parse GeneralIpcMessage as HandshakePayload: {e}")
            })?;

            match &*state {
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
            }
        }

        #[allow(unreachable_code)]
        Ok(())
    });

    let ipc_send = handshake_ipc.clone();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            ipc_send.send(&msg).await.unwrap();
        }
    });

    let _ = tokio::try_join!(recv_task, send_task).map_err(|e| format!("Task failed: {}", e))?;

    Ok(())
}
