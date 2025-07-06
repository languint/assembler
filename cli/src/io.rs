use std::{
    env, fs,
    path::Path,
};

use crate::errors::CliError;

pub fn get_current_directory() -> Result<std::path::PathBuf, CliError> {
    let result = env::current_dir();

    if result.is_err() {
        return Err(CliError::IOError(format!(
            "Failed to get current directory: {}",
            result.unwrap_err()
        )));
    }

    Ok(result.unwrap())
}

pub fn make_folder(current_dir: &Path, name: &str) -> Result<(), CliError> {
    fs::create_dir_all(current_dir.join(name))
        .map_err(|e| CliError::IOError(format!("Failed to create folder: {}", e)))
}

pub fn folder_exists(current_dir: &Path, name: &str) -> Result<(), CliError> {
    let path = current_dir.join(name);

    match path.exists() {
        true => Ok(()),
        false => Err(CliError::IOError(format!(
            "Folder {} does not exist!",
            name
        ))),
    }
}

pub fn read_file(file_path: &Path) -> Result<String, CliError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| CliError::IOError(format!("Failed to read file: {}", e)))?;

    Ok(content)
}
