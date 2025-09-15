//! CLI error types per MODULE_SPEC

use std::fmt;

#[derive(Debug, Clone)]
pub enum CliError {
    /// Invalid command line argument
    InvalidArgument(String),
    /// Missing required argument
    MissingArgument(String),
    /// Invalid width value provided
    InvalidWidth(String),
    /// Unsupported command
    UnsupportedCommand(String),
    /// General CLI parsing error
    ParseError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            CliError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            CliError::InvalidWidth(val) => write!(f, "Invalid width value: {}", val),
            CliError::UnsupportedCommand(cmd) => write!(f, "Unsupported command: {}", cmd),
            CliError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}