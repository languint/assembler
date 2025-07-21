use std::{
    env, fs,
    path::Path,
    process::{Command, exit},
};

use clap::{Parser, Subcommand, arg};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Package {
        #[arg(short, long, default_value = "0.1.0")]
        version: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct ModInfo {
    name: String,
    version: String,
    title: String,
    author: String,
    factorio_version: String,
    dependencies: Vec<String>,
}

fn load_info_json() -> Result<ModInfo, String> {
    let info_file_path = Path::new("mod/info.json");
    let info_file_content = fs::read_to_string(info_file_path)
        .map_err(|e| format!("Failed to read info.json: {}", e))?;

    match serde_json::from_str(&info_file_content) {
        Ok(info) => Ok(info),
        Err(e) => Err(format!("Failed to parse info.json: {}", e)),
    }
}

fn write_info_json(info: &ModInfo) -> Result<(), String> {
    let info_file_path = Path::new("mod/info.json");
    let info_file_content = serde_json::to_string_pretty(info)
        .map_err(|e| format!("Failed to serialize info.json: {}", e))?;

    fs::write(info_file_path, info_file_content)
        .map_err(|e| format!("Failed to write info.json: {}", e))?;

    Ok(())
}

fn update_mod_version(version: &str) -> Result<(), String> {
    let mut info = load_info_json()?;
    info.version = version.to_string();

    write_info_json(&info)?;
    println!("Updated mod info to version: {}", version);

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Package { version } => {
            println!("Packaging mod with version: {}...", version);
            if let Err(e) = update_mod_version(&version) {
                eprintln!("Error: {}", e);
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
                    println!("Mod packaged successfully as {}", packaged_mod_name);
                }
                _ => {
                    eprintln!(
                        "Failed to package mod: {}",
                        String::from_utf8_lossy(&zip_command.stderr)
                    );
                    exit(1);
                }
            }

            let factorio_path = env::var("FACTORIO_PATH").unwrap_or_else(|_| {
                eprintln!("FACTORIO_PATH environment variable is not set.");
                exit(1);
            });

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
                    eprintln!(
                        "Failed to copy mod: {}",
                        String::from_utf8_lossy(&copy_command.stderr)
                    );
                    exit(1);
                }
            }

            let remove_command = Command::new("rm")
                .arg(&packaged_mod_name)
                .output()
                .expect("Failed to execute remove command");
            match remove_command.status {
                status if status.success() => {
                    println!("Temporary packaged mod file removed.");
                }
                _ => {
                    eprintln!(
                        "Failed to remove temporary packaged mod file: {}",
                        String::from_utf8_lossy(&remove_command.stderr)
                    );
                    exit(1);
                }
            }
        }
    }
}
