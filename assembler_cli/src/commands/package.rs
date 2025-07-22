use std::{env, path::Path, process::Command};

use crate::{cli, commands, lua_mod};

pub async fn package_command(version: String, launch: bool, port: u16) -> Result<(), String> {
    cli::log_header(
        "PKG",
        format!("Starting packaging process for assembler `{}`", version).as_str(),
        0,
        Some(cli::CLI_YELLOW_HEADER),
    );

    if let Err(e) = lua_mod::migrate_mod_version(&version) {
        return Err(format!("Failed to migrate mod version: {}", e));
    }

    let packaged_mod_name = format!("assembler_{}.zip", version);

    let zip_command = Command::new("zip")
        .arg("-r")
        .arg(&packaged_mod_name)
        .arg("mod")
        .output()
        .map_err(|_| "Failed to execute zip command")?;

    match zip_command.status {
        status if status.success() => {
            cli::log_header(
                "PKG",
                format!("Mod packaged successfully as: `{}`", packaged_mod_name).as_str(),
                4,
                Some(cli::CLI_YELLOW_HEADER),
            );
        }
        _ => {
            return Err(format!(
                "Failed to package mod: {}",
                String::from_utf8_lossy(&zip_command.stderr)
            ));
        }
    }

    let factorio_path =
        env::var("FACTORIO_PATH").map_err(|_| "FACTORIO_PATH environment variable is not set.")?;

    let mods_path = Path::new(&factorio_path).join("mods");

    let copy_command = Command::new("mv")
        .arg(&packaged_mod_name)
        .arg(&mods_path)
        .output()
        .map_err(|_| "Failed to execute move command")?;

    match copy_command.status {
        status if status.success() => {
            cli::log_header(
                "PKG",
                format!(
                    "Mod moved to Factorio mods directory: `{}`",
                    mods_path.display()
                )
                .as_str(),
                4,
                Some(cli::CLI_YELLOW_HEADER),
            );
        }
        _ => {
            return Err(format!(
                "Failed to move mod: {}",
                String::from_utf8_lossy(&copy_command.stderr)
            ));
        }
    }

    if launch {
        cli::log_header(
            "PKG",
            "Launching Factorio through Steam",
            4,
            Some(cli::CLI_YELLOW_HEADER),
        );
        commands::start::start_command(port).await?
    }
    Ok(())
}
