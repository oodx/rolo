//! CLI dispatch system with RSB patterns

use crate::cli::{CliConfig, parse_args, execute_cli};
use crate::cli::error::CliError;
use std::process;

/// Main dispatch function following RSB patterns
pub fn dispatch_cli() -> ! {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let config = match parse_args(&args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use --help for usage information");
            process::exit(1);
        }
    };

    // Execute CLI action
    if let Err(e) = execute_cli(&config) {
        eprintln!("Execution error: {}", e);
        process::exit(1);
    }

    process::exit(0);
}

/// RSB-style command handler function type
pub type CommandHandler = fn(&CliConfig) -> Result<(), CliError>;

/// Execute specific command with configuration
pub fn execute_command(config: &CliConfig, handler: CommandHandler) -> Result<(), CliError> {
    handler(config)
}

/// Built-in command handlers following RSB patterns

pub fn handle_columns(config: &CliConfig) -> Result<(), CliError> {
    use crate::prelude::*;

    let cols = config.columns.unwrap_or(2);
    let input = "Sample input text for column formatting"; // TODO: Read from stdin

    match format_columns(input, cols) {
        Ok(formatted) => println!("{}", formatted),
        Err(e) => return Err(CliError::ParseError(format!("Column formatting failed: {}", e))),
    }

    Ok(())
}

pub fn handle_table(config: &CliConfig) -> Result<(), CliError> {
    use crate::prelude::*;

    let input = "Sample input text for table formatting"; // TODO: Read from stdin

    match format_table(input, "\t") {
        Ok(formatted) => println!("{}", formatted),
        Err(e) => return Err(CliError::ParseError(format!("Table formatting failed: {}", e))),
    }

    Ok(())
}

pub fn handle_list(_config: &CliConfig) -> Result<(), CliError> {
    use crate::prelude::*;

    let input = "Sample input text for list formatting"; // TODO: Read from stdin

    match format_list(input) {
        Ok(formatted) => println!("{}", formatted),
        Err(e) => return Err(CliError::ParseError(format!("List formatting failed: {}", e))),
    }

    Ok(())
}