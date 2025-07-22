use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod commands;
mod lua_mod;
mod ipc;

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let res = match cli.command {
        Commands::Package { version, launch, port } => {
            commands::package::package_command(version, launch, port).await
        }
        Commands::Start { port} => commands::start::start_command(port).await,
    };

    if let Err(e) = res {
        cli::log_error("ERR", &e, 0);
    }

    Ok(())
}
