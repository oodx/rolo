//! Stream processing errors
//!
//! Defines errors that can occur during stream operations, stdin/stdout handling,
//! and pipe processing. Integrates with RSB error patterns.

use std::fmt;

/// Errors that can occur during stream processing
#[derive(Debug, Clone)]
pub enum StreamError {
    /// Error reading from stdin
    StdinRead(String),
    /// Error writing to stdout
    StdoutWrite(String),
    /// Pipe broken (SIGPIPE or similar)
    PipeBroken(String),
    /// EOF encountered unexpectedly
    UnexpectedEof,
    /// Buffer overflow or size limit exceeded
    BufferOverflow(usize),
    /// Invalid UTF-8 in stream
    InvalidUtf8(String),
    /// Generic I/O error
    IoError(String),
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamError::StdinRead(msg) => write!(f, "Failed to read from stdin: {}", msg),
            StreamError::StdoutWrite(msg) => write!(f, "Failed to write to stdout: {}", msg),
            StreamError::PipeBroken(msg) => write!(f, "Pipe broken: {}", msg),
            StreamError::UnexpectedEof => write!(f, "Unexpected end of file"),
            StreamError::BufferOverflow(size) => write!(f, "Buffer overflow at {} bytes", size),
            StreamError::InvalidUtf8(msg) => write!(f, "Invalid UTF-8 encoding: {}", msg),
            StreamError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for StreamError {}

/// Result type for stream operations
pub type StreamResult<T> = Result<T, StreamError>;

/// Convert std::io::Error to StreamError
impl From<std::io::Error> for StreamError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;

        match err.kind() {
            ErrorKind::BrokenPipe => StreamError::PipeBroken(err.to_string()),
            ErrorKind::UnexpectedEof => StreamError::UnexpectedEof,
            ErrorKind::InvalidData => StreamError::InvalidUtf8(err.to_string()),
            _ => StreamError::IoError(err.to_string()),
        }
    }
}

/// Convert std::string::FromUtf8Error to StreamError
impl From<std::string::FromUtf8Error> for StreamError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        StreamError::InvalidUtf8(err.to_string())
    }
}

/// Convert std::str::Utf8Error to StreamError
impl From<std::str::Utf8Error> for StreamError {
    fn from(err: std::str::Utf8Error) -> Self {
        StreamError::InvalidUtf8(err.to_string())
    }
}