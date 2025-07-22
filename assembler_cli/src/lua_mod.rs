use std::{fs, path::Path};

use serde::{Serialize, Deserialize};

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

pub fn update_mod_version(version: &str) -> Result<(), String> {
    let mut info = load_info_json()?;
    info.version = version.to_string();

    write_info_json(&info)?;
    println!("Updated mod info to version: {}", version);

    Ok(())
}