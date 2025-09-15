//! Layout error types per MODULE_SPEC

use std::fmt;

/// Layout module errors
#[derive(Debug, Clone)]
pub enum LayoutError {
    /// Invalid column count
    InvalidColumnCount(usize),
    /// Invalid width specification
    InvalidWidth(usize),
    /// Width too small for the specified number of columns and gaps
    WidthTooSmall(usize, usize),
    /// Individual column width is too narrow to be useful
    ColumnTooNarrow(usize),
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
            LayoutError::WidthTooSmall(width, gap_space) => {
                write!(f, "Width {} is too small for gaps requiring {} spaces", width, gap_space)
            }
            LayoutError::ColumnTooNarrow(width) => {
                write!(f, "Column width {} is too narrow (minimum 3)", width)
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