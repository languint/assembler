use std::process::Command;

use tokio::net::UdpSocket;

fn start_factorio() -> Result<(), String> {
    let factorio_steam_id = 427520;

    Command::new("steam")
        .arg(format!(
            "steam://run/{}//--enable-lua-udp%2012346",
            factorio_steam_id
        ))
        .spawn()
        .map_err(|e| format!("Failed to start Factorio: {}", e))?;

    println!("Factorio started with Lua UDP enabled on port 12345.");
    Ok(())
}

pub async fn start_command() -> Result<(), String> {
    let udp_server = tokio::spawn(async {
        let sock = UdpSocket::bind("127.0.0.1:12345")
            .await
            .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
        let mut buf = [0; 1024];

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

    start_factorio()?;

    println!("Factorio started with Lua UDP enabled on port 12345.");

    udp_server.await.unwrap()
}
