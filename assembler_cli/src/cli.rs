use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Package {
        #[arg(short, long, default_value = "0.1.0")]
        version: String,
        #[arg(short, long, default_value = "true")]
        launch: bool,
    },
    Start,
}
