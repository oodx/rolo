//! Rolo - Text layout tool for Unix pipelines
//!
//! The spiritual love child of pr, paste, and col.
//! ANSI-aware, Unicode-safe, streaming performance.

#![doc = include_str!("../README.md")]

// Module exports following RSB MODULE_SPEC
pub mod layout;
pub mod width;

// Stream and CLI modules (placeholder structure)
// TODO: Implement remaining modules in subsequent tasks

/// Prelude with curated exports per MODULE_SPEC
pub mod prelude {
    // Layout functionality
    pub use crate::layout::{format_columns, format_table, format_list, LayoutConfig};

    // Width calculation functionality
    pub use crate::width::{get_display_width, get_terminal_width, validate_width};

    // Error types for comprehensive error handling
    pub use crate::width::error::WidthError;
    pub use crate::layout::error::LayoutError;

    // Module-owned macros
    pub use crate::layout_config;

    // TODO: Add CLI and Stream modules as they are implemented
}

// Re-export prelude at crate root for convenience
pub use prelude::*;