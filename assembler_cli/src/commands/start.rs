use std::sync::Arc;

use crate::ipc::handshake::{HandshakePayload, HandshakeState};
use crate::ipc::schema::{GeneralIpcMessage, IpcSchema};
use crate::ipc::{self, Ipc};

use crate::cli;
use crate::lua_mod::AssemblerConfig;
use tokio::sync::{Mutex, mpsc};

pub async fn start_command(config: &AssemblerConfig) -> Result<(), String> {
    let handshake_ipc: Ipc = Ipc::new(config.ipc.handshake_port, config.ipc.factorio_port).await?;

    ipc::process::start_factorio(config.ipc.factorio_port)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let handshake_state = Arc::new(Mutex::new(HandshakeState::Init));
    let handshake_state_recv = handshake_state.clone();
    let tx_recv = tx.clone();

    let ipc_recv = handshake_ipc.clone();
    let recv_task = tokio::spawn(async move {
        loop {
            let msg = ipc_recv.receive().await?;

            let state = handshake_state_recv.lock().await;

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

            ipc::handshake::handshake_handler(&tx_recv, state, msg_json)?;
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
