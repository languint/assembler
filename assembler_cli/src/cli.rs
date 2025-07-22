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
        #[arg(short, long, default_value = "false")]
        launch: bool,
    },
    Start,
}

pub fn log_header(header: &str, message: &str, depth: u8) {
    println!("{}[{}] {}", " ".repeat(depth as usize), header, message);
}

pub fn log_error(header: &str, message: &str, depth: u8) {
    eprintln!("{}[{}] {}", " ".repeat(depth as usize), header, message);
}
