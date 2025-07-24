pub mod routes;
pub mod process;
pub mod schema;

use std::sync::Arc;
use tokio::{net::UdpSocket, sync::mpsc};

use crate::cli;

#[derive(Clone)]
pub struct Ipc {
    sock: Arc<UdpSocket>,
    #[allow(unused)]
    pub(crate) port: u32,
}

impl Ipc {
    pub async fn new(port: u32, remote_port: u32) -> Result<Self, String> {
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

pub async fn create_socket<F, Fut>(
    port: u32,
    factorio_port: u32,
    header: &str,
    mut handler: F,
) -> Result<
    (
        mpsc::UnboundedSender<String>,
        tokio::task::JoinHandle<Result<(), String>>,
    ),
    String,
>
where
    F: FnMut(String, mpsc::UnboundedSender<String>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<(), String>> + Send + 'static,
{
    let ipc = Ipc::new(port, factorio_port).await?;
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let ipc_recv = ipc.clone();
    let tx_for_handler = tx.clone();
    let header_recv = header.to_string();

    let recv_task = tokio::spawn(async move {
        loop {
            let msg = ipc_recv.receive().await?;
            if let Err(e) = handler(msg, tx_for_handler.clone()).await {
                cli::log_header(
                    &header_recv,
                    format!("Error in handler: {}", e).as_str(),
                    0,
                    Some(cli::CLI_RED_HEADER),
                );
                return Err(e);
            }
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    let ipc_send = ipc.clone();
    let header_send = header.to_string();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = ipc_send.send(&msg).await {
                cli::log_header(
                    &header_send,
                    format!("Error sending message: {}", e).as_str(),
                    0,
                    Some(cli::CLI_RED_HEADER),
                );
            }
        }
    });

    let join_handle = tokio::spawn(async move {
        let (recv_res, _) = tokio::join!(recv_task, send_task);
        recv_res.unwrap()?;
        Ok(())
    });

    Ok((tx, join_handle))
}
