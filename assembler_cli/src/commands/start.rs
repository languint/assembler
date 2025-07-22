use std::process::{Command, Stdio};

use tokio::net::UdpSocket;

use crate::cli;

fn start_factorio(port: u16) -> Result<(), String> {
    let factorio_steam_id = 427520;

    let factorio_pid = Command::new("steam")
        .arg(format!(
            "steam://run/{}//--enable-lua-udp%20{}",
            factorio_steam_id,
            port
        ))
        .stdout(Stdio::null()) // Suppress output
        .spawn()
        .map_err(|e| format!("Failed to start Factorio: {}", e))?
        .id();

    cli::log_header(
        "PEX",
        format!("Factorio started with PID `{}`, and port `{}`", factorio_pid, port).as_str(),
        0,
    );
    Ok(())
}

pub async fn start_command(port: u16) -> Result<(), String> {
    let udp_server = tokio::spawn(async move {
        let localhost_addr = format!("127.0.0.1:{}", port);
        let sock = UdpSocket::bind(&localhost_addr)
            .await
            .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
        let mut buf = [0; 1024];

        cli::log_header(
            "UDP",
            format!("Socket listening on {}", localhost_addr).as_str(),
            0,
        );
        loop {
            let (len, addr) = sock
                .recv_from(&mut buf)
                .await
                .map_err(|e| format!("Failed to receive data: {}", e))?;
            println!("{:?} bytes received from {:?}", len, addr);
            println!("Data: {}", String::from_utf8_lossy(&buf[..len]));
        }
        #[allow(unreachable_code)]
        Ok::<(), String>(())
    });

    start_factorio(port + 1)?;

    udp_server.await.unwrap()
}
