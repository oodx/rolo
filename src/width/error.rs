//! Width calculation errors per MODULE_SPEC

use std::fmt;

#[derive(Debug, Clone)]
pub enum WidthError {
    /// Invalid text input
    InvalidInput(String),
    /// Width calculation failed
    CalculationError(String),
}

impl fmt::Display for WidthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidthError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            WidthError::CalculationError(msg) => write!(f, "Width calculation error: {}", msg),
        }
    }
}

impl std::error::Error for WidthError {}