use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::cli;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfo {
    name: String,
    version: String,
    title: String,
    author: String,
    factorio_version: String,
    dependencies: Vec<String>,
}

pub fn load_info_json() -> Result<ModInfo, String> {
    let info_file_path = Path::new("mod/info.json");
    let info_file_content = fs::read_to_string(info_file_path)
        .map_err(|e| format!("Failed to read info.json: {}", e))?;

    match serde_json::from_str(&info_file_content) {
        Ok(info) => Ok(info),
        Err(e) => Err(format!("Failed to parse info.json: {}", e)),
    }
}

pub fn write_info_json(info: &ModInfo) -> Result<(), String> {
    let info_file_path = Path::new("mod/info.json");
    let info_file_content = serde_json::to_string_pretty(info)
        .map_err(|e| format!("Failed to serialize info.json: {}", e))?;

    fs::write(info_file_path, info_file_content)
        .map_err(|e| format!("Failed to write info.json: {}", e))?;

    Ok(())
}

pub fn migrate_mod_version(version: &str) -> Result<(), String> {
    let mut info = load_info_json()?;
    info.version = version.to_string();

    if let Err(err) = write_info_json(&info) {
        return Err(format!("Failed to write updated info.json: {}", err));
    };

    cli::log_header(
        "PKG",
        format!("Successfully migrated assembler version to `{}`", version).as_str(),
        4,
        Some(cli::CLI_YELLOW_HEADER)
    );

    Ok(())
}
