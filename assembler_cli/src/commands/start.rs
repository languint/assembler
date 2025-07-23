use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::sync::Arc;

use crate::ipc::{self, Ipc};

use crate::cli;
use libdeflater::Decompressor;
use tokio::sync::{Mutex, mpsc};

fn start_factorio(port: u16) -> Result<(), String> {
    let factorio_steam_id = 427520;

    let factorio_pid = Command::new("steam")
        .arg(format!(
            "steam://run/{}//--enable-lua-udp%20{}",
            factorio_steam_id, port
        ))
        .stdout(Stdio::null()) // Suppress output
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

pub async fn start_command(port: u16) -> Result<(), String> {
    let ipc = Ipc::new(port, port + 1).await?;
    start_factorio(port + 1)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let handshake_state = Arc::new(Mutex::new(ipc::HandshakeState::Init));
    let handshake_state_recv = handshake_state.clone();
    let tx_recv = tx.clone();

    let ipc_recv = ipc.clone();
    let recv_task = tokio::spawn(async move {
        loop {
            let msg = ipc_recv.receive().await?;

            let mut state = handshake_state_recv.lock().await;
            match &*state {
                ipc::HandshakeState::Init => {
                    if msg.trim() == "ACK" {
                        cli::log_header(
                            "IPC-HANDSHAKE",
                            "Recieved ACK, sending ACK",
                            0,
                            Some(cli::CLI_PURPLE_HEADER),
                        );
                        tx_recv
                            .send(ipc::HANDSHAKE_ACK_MESSAGE.to_string())
                            .unwrap();

                        *state = ipc::HandshakeState::Acked;
                    }
                }
                ipc::HandshakeState::Acked => {
                    if msg.trim() == "OK" {
                        cli::log_header(
                            "IPC-HANDSHAKE",
                            "Recieved OK, sending OK!",
                            0,
                            Some(cli::CLI_PURPLE_HEADER),
                        );
                        tx_recv.send(ipc::HANDSHAKE_OK_MESSAGE.to_string()).unwrap();

                        *state = ipc::HandshakeState::Ready;
                        cli::log_header(
                            "IPC-HANDSHAKE",
                            "OK-OK, READY!",
                            0,
                            Some(cli::CLI_PURPLE_HEADER),
                        );
                    }
                }
                ipc::HandshakeState::Ready => {
                    let msg_json = serde_json::from_str::<ipc::IPCMessage>(msg.as_str());

                    if let Err(e) = msg_json {
                        cli::log_error(
                            "IPC",
                            "Recieved invalid JSON",
                            0,
                            Some(cli::CLI_RED_HEADER),
                        );
                        return Err(e.to_string());
                    }

                    let msg_json = msg_json.unwrap();

                    if msg_json.r#type == "observation" {
                        cli::log_header(
                            "IPC-OBSERVATION",
                            "Recieved observation data",
                            0,
                            Some(cli::CLI_YELLOW_HEADER),
                        );
                        let msg_json = serde_json::from_str::<ipc::IPCObservation>(msg.as_str());

                        match msg_json {
                            Ok(observation) => {
                                let data: serde_json::Value = serde_json::from_str(observation.data.as_str()).unwrap();

                                cli::log_header(
                                    "IPC-OBSERVATION",
                                    format!("Observation data: {}", serde_json::to_string_pretty(&data).unwrap()).as_str(),
                                    0,
                                    Some(cli::CLI_YELLOW_HEADER),
                                );
                                tx_recv
                                    .send(format!(
                                        "observation: {}",
                                        observation.data
                                    ))
                                    .unwrap();
                            }
                            Err(e) => {
                                cli::log_error(
                                    "IPC-OBSERVATION",
                                    format!("Failed to parse observation: {}", e).as_str(),
                                    0,
                                    Some(cli::CLI_RED_HEADER),
                                );
                            }
                        }
                    } else {
                        cli::log_error(
                            "IPC",
                            format!("Unknown message type: {}", msg_json.r#type).as_str(),
                            0,
                            Some(cli::CLI_RED_HEADER),
                        );
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        Ok::<(), String>(())
    });

    let ipc_send = ipc.clone();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            ipc_send.send(&msg).await.unwrap();
        }
    });

    let _ = tokio::try_join!(recv_task, send_task).map_err(|e| format!("Task failed: {}", e))?;

    Ok(())
}
