//! Width calculation errors per MODULE_SPEC

use std::fmt;

#[derive(Debug, Clone)]
pub enum WidthError {
    /// Invalid text input
    InvalidInput(String),
    /// Width calculation failed
    CalculationError(String),
    /// Width value out of valid range
    InvalidRange(usize, usize, usize),
    /// Terminal access error
    TerminalError(String),
}

impl fmt::Display for WidthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidthError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            WidthError::CalculationError(msg) => write!(f, "Width calculation error: {}", msg),
            WidthError::InvalidRange(value, min, max) => {
                write!(f, "Width {} out of range ({}-{})", value, min, max)
            },
            WidthError::TerminalError(msg) => write!(f, "Terminal access error: {}", msg),
        }
    }
}

impl std::error::Error for WidthError {}