use std::process::{Command, Stdio};

use crate::cli;

pub fn start_factorio(port: u32) -> Result<(), String> {
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
