use std::{fs, path::Path, process};

use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    lua_mod::AssemblerConfig,
};

mod cli;
mod commands;
mod ipc;
mod lua_mod;

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let config_file_path = Path::new("mod").join("Assembler.toml");
    let config: AssemblerConfig;

    match fs::exists(&config_file_path)
        .map_err(|e| format!("Failed to check if Assembler.toml exists: {}", e))
    {
        Ok(_) => {
            let config_file_contents = fs::read_to_string(&config_file_path)
                .map_err(|e| format!("Failed to read Assembler.toml {e}"))?;

            config = toml::from_str(config_file_contents.as_str())
                .map_err(|_| "Invalid Assembler.toml")?;
        }
        Err(e) => {
            cli::log_error("ERR", &e, 0, Some(cli::CLI_RED_HEADER));
            process::exit(1);
        }
    }

    let res = match cli.command {
        Commands::Package { version, launch } => {
            commands::package::package_command(version, launch, &config).await
        }
        Commands::Start => commands::start::start_command(&config).await,
    };

    if let Err(e) = res {
        cli::log_error("ERR", &e, 0, Some(cli::CLI_RED_HEADER));
    }

    Ok(())
}
