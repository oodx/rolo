//! Stream processing module - handles stdin/stdout and pipe operations
//!
//! This module provides buffered reading from stdin, writing to stdout,
//! and proper handling of pipe breaks and EOF conditions following RSB patterns.

mod utils;
mod helpers;
mod error;

pub use utils::*;
pub use error::*;

// Internal helpers are not re-exported
// use helpers::*; // Currently unused