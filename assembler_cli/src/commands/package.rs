use std::{env, path::Path, process::Command};

use crate::{commands, lua_mod};

pub async fn package_command(version: String, launch: bool) -> Result<(), String> {
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
                commands::start::start_command().await?
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