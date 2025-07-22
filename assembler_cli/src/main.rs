use std::{env, path::Path, process::Command};

use clap::Parser;
use tokio::net::UdpSocket;

use crate::cli::{Cli, Commands};

mod cli;
mod lua_mod;

async fn package_command(version: String, launch: bool) -> Result<(), String> {
    println!("Packaging mod with version: {}...", version);
    if let Err(e) = lua_mod::update_mod_version(&version) {
        return Err(format!("Failed to update mod version: {}", e));
    }

    let packaged_mod_name = format!("assembler_{}.zip", version);

    let zip_command = Command::new("zip")
        .arg("-r")
        .arg(&packaged_mod_name)
        .arg("mod")
        .output()
        .expect("Failed to execute zip command");

    match zip_command.status {
        status if status.success() => {
            println!("Mod packaged successfully as {}", packaged_mod_name)
        }
        _ => {
            return Err(format!(
                "Failed to package mod: {}",
                String::from_utf8_lossy(&zip_command.stderr)
            ));
        }
    }

    let factorio_path =
        env::var("FACTORIO_PATH").expect("FACTORIO_PATH environment variable is not set.");

    let mods_path = Path::new(&factorio_path).join("mods");

    let copy_command = Command::new("cp")
        .arg(&packaged_mod_name)
        .arg(&mods_path)
        .output()
        .expect("Failed to execute copy command");

    match copy_command.status {
        status if status.success() => {
            println!(
                "Mod copied to Factorio mods directory: {}",
                mods_path.display()
            );
        }
        _ => {
            return Err(format!(
                "Failed to copy mod: {}",
                String::from_utf8_lossy(&copy_command.stderr)
            ));
        }
    }

    let remove_command = Command::new("rm")
        .arg(&packaged_mod_name)
        .output()
        .expect("Failed to execute remove command");
    match remove_command.status {
        status if status.success() => {
            println!("Temporary packaged mod file removed.");
            if launch {
                println!("Launching Factorio...");
                start_command().await?
            }

            Ok(())
        }
        _ => {
            return Err(format!(
                "Failed to remove temporary packaged mod file: {}",
                String::from_utf8_lossy(&remove_command.stderr)
            ));
        }
    }
}

async fn start_command() -> Result<(), String> {
    let factorio_steam_id = 427520;

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

    Command::new("steam")
        .arg(format!(
            "steam://run/{}//--enable-lua-udp%2012346",
            factorio_steam_id
        ))
        .spawn()
        .map_err(|e| format!("Failed to start Factorio: {}", e))?;

    println!("Factorio started with Lua UDP enabled on port 12345.");

    udp_server.await.unwrap()
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Package { version, launch } => package_command(version, launch).await?,
        Commands::Start => start_command().await?,
    }

    Ok(())
}
