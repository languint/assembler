use clap::{command, Parser, Subcommand};
use owo_colors::OwoColorize;


#[derive(Subcommand, Debug, Clone)]
pub enum CliCommand {
    BuildMod,
}

#[derive(Parser, Debug)]
#[command(author = "longuint", about = "assember cli", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub quiet: bool,
}

#[inline]
pub fn print_error(error: &str, depth: usize) {
    println!(
        "{}{}{} {}",
        " ".repeat(depth),
        "Error".bold().red(),
        ":".bold(),
        error.red()
    );
}

#[inline]
pub fn print_warning(warning: &str, depth: usize) {
    println!(
        "{}{}{} {}",
        " ".repeat(depth),
        "warning".bold().yellow(),
        ":".bold(),
        warning
    );
}