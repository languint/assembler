use crate::cli;

#[derive(Debug, PartialEq)]
pub enum CliError {
    IOError(String),
    MissingVar(String),
    CMDError(String),
}

impl CliError {
    pub fn print(&self, depth: Option<i32>) {
        cli::print_error(&get_print_error(self), if let Some(d) = depth { d as usize} else { 0 });
    }
}

pub fn get_print_error(error: &CliError) -> String {
    match error {
        CliError::IOError(msg) => format!("(C001): IO error: {msg}"),
        CliError::MissingVar(msg) => format!("(C002): Missing Variable: {msg}"),
        CliError::CMDError(msg) => format!("(C003): Command Error: {msg}")
    }
}
