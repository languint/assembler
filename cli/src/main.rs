use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use owo_colors::OwoColorize;

use crate::{
    cli::{Cli, CliCommand},
    errors::CliError,
};

mod cli;
mod errors;
mod io;

#[derive(Debug, PartialEq)]
enum LogLevel {
    Verbose,
    Quiet,
    Default,
}

fn main() {
    let cli = Cli::parse();

    let log_level = match (cli.quiet, cli.verbose) {
        (true, true) => {
            cli::print_warning("quiet and verbose flags passed, using verbose", 0);
            LogLevel::Verbose
        }
        (true, false) => LogLevel::Quiet,
        (false, true) => LogLevel::Verbose,
        (false, false) => LogLevel::Default,
    };

    let current_dir = io::get_current_directory();

    if current_dir.is_err() {
        let err = current_dir.unwrap_err();
        err.print(None);
        process::exit(1);
    }

    let current_dir = current_dir.unwrap();

    match cli.command {
        CliCommand::BuildMod => build_mod(&current_dir.as_path(), log_level),
    }
}

fn build_mod(current_dir: &Path, log_level: LogLevel) {
    println!("{} `build-mod`", "Running".green().bold());

    let factorio_bin_path = env::var("FACTORIO_BIN_PATH");

    if factorio_bin_path.is_err() {
        CliError::MissingVar(
            "Couldn't find environment variable FACTORIO_BIN_PATH, make sure it's in your .bashrc"
                .into(),
        )
        .print(None);
        process::exit(1);
    }

    let factorio_bin_path = factorio_bin_path.unwrap();

    let factorio_path = env::var("FACTORIO_PATH");

    if factorio_path.is_err() {
        CliError::MissingVar(
            "Couldn't find environment variable FACTORIO_PATH, make sure it's in your .bashrc"
                .into(),
        )
        .print(None);
        process::exit(1);
    }

    let factorio_path = factorio_path.unwrap();

    if log_level == LogLevel::Verbose {
        println!("{}{}{}", " ".repeat(2), "paths".bold().blue(), ":".bold());
        println!("{}bin: {}", " ".repeat(4), factorio_bin_path);
        println!("{}data: {}", " ".repeat(4), factorio_path);
    }

    let mods_dir = PathBuf::from(factorio_path).join("mods");

    match io::folder_exists(current_dir, "assembler") {
        Ok(_) => {
            println!(
                "{}{} `assembler.zip`",
                " ".repeat(2),
                "zipping".green().bold()
            );
            let zip_command = process::Command::new("zip")
                .args(["-r", "assembler_0.1.0.zip", "assembler"])
                .output();

            match zip_command {
                Ok(_) => {}
                Err(e) => {
                    CliError::CMDError(format!("Couldn't zip mod {}", e).into()).print(None);
                    process::exit(1);
                }
            }

            println!(
                "{}{} `assembler_0.1.0.zip`",
                " ".repeat(2),
                "renaming".green().bold()
            );
            let copy_command = fs::rename(
                current_dir.join("assembler_0.1.0.zip"),
                mods_dir.join("assembler_0.1.0.zip"),
            );

            match copy_command {
                Ok(_) => {}
                Err(e) => {
                    CliError::CMDError(format!("Couldn't rename assembler.zip {}", e).into())
                        .print(None);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            e.print(None);
            process::exit(1)
        }
    }
}
