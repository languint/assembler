use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::cli;

#[derive(Clone)]
pub struct Ipc {
    sock: Arc<UdpSocket>,
    #[allow(unused)]
    pub(crate) port: u16,
}

impl Ipc {
    pub async fn new(port: u16, remote_port: u16) -> Result<Self, String> {
        let localhost_addr = format!("127.0.0.1:{}", port);
        let sock = Arc::new(
            UdpSocket::bind(&localhost_addr)
                .await
                .map_err(|e| format!("Failed to bind UDP socket: {}", e))?,
        );

        cli::log_header(
            "IPC",
            format!("IPC socket listening on {}", localhost_addr).as_str(),
            0,
            Some(cli::CLI_BLUE_HEADER),
        );

        let remote_addr = format!("127.0.0.1:{}", remote_port);
        sock.connect(&remote_addr)
            .await
            .map_err(|e| format!("Failed to connect to remote address {}: {}", remote_addr, e))?;

        cli::log_header(
            "IPC",
            format!("Connected to remote address {}", remote_addr).as_str(),
            0,
            Some(cli::CLI_BLUE_HEADER),
        );

        Ok(Ipc { port, sock })
    }

    pub async fn send(&self, message: &str) -> Result<(), String> {
        self.sock
            .send(message.as_bytes())
            .await
            .map_err(|e| format!("Failed to send message: {}", e))?;
        Ok(())
    }

    pub async fn receive(&self) -> Result<String, String> {
        let mut buf = [0; 65536]; // 64KB
        let len = self
            .sock
            .recv(&mut buf)
            .await
            .map_err(|e| format!("Failed to receive data: {}", e))?;
        let message = String::from_utf8_lossy(&buf[..len]).to_string();
        Ok(message)
    }
}

#[derive(Debug)]
pub enum HandshakeState {
    Init,
    Acked,
    Ready,
}

pub const HANDSHAKE_ACK_MESSAGE: &str = r#"
    {
        "type": "handshake",
        "msg": "ACK"
    }
"#;

pub const HANDSHAKE_OK_MESSAGE: &str = r#"
    {
        "type": "handshake",
        "msg": "OK"
    }
"#;

#[derive(Debug, serde::Deserialize)]
pub struct IPCMessage {
    pub r#type: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct IPCObservation {
    pub r#type: String,
    pub data: String,
}
