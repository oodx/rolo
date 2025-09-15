//! Layout error types per MODULE_SPEC

use std::fmt;

/// Layout module errors
#[derive(Debug, Clone)]
pub enum LayoutError {
    /// Invalid column count
    InvalidColumnCount(usize),
    /// Invalid width specification
    InvalidWidth(usize),
    /// Text processing error
    ProcessingError(String),
    /// IO error during layout
    IoError(String),
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutError::InvalidColumnCount(count) => {
                write!(f, "Invalid column count: {}", count)
            }
            LayoutError::InvalidWidth(width) => {
                write!(f, "Invalid width: {}", width)
            }
            LayoutError::ProcessingError(msg) => {
                write!(f, "Text processing error: {}", msg)
            }
            LayoutError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
        }
    }
}

impl std::error::Error for LayoutError {}