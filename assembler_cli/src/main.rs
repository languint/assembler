use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod commands;
mod lua_mod;

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Package { version, launch } => {
            commands::package::package_command(version, launch).await?
        }
        Commands::Start => commands::start::start_command().await?,
    }

    Ok(())
}
