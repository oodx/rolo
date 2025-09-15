//! Rolo - Text layout tool for Unix pipelines
//!
//! The spiritual love child of pr, paste, and col.
//! ANSI-aware, Unicode-safe, streaming performance.

#![doc = include_str!("../README.md")]

// Module exports following RSB MODULE_SPEC
pub mod layout;
pub mod width;
pub mod stream;

/// Prelude with curated exports per MODULE_SPEC (RSB compliant)
pub mod prelude {
    // Layout functionality
    pub use crate::layout::{format_columns, format_columns_with_config, format_columns_with_delimiter, format_table, format_table_with_config, format_list, format_list_with_config, LayoutConfig, ListConfig, ListAlignment};

    // Width calculation functionality
    pub use crate::width::{get_display_width, get_terminal_width, validate_width, check_terminal_resize};

    // Stream processing functionality
    pub use crate::stream::{
        read_stdin, write_stdout, stdin_to_stream, stream_to_stdout,
        pipe_transform, pipe_lines, create_pipeline, Pipeline,
        StreamConfig, LineEnding
    };

    // Error types for comprehensive error handling
    pub use crate::width::error::WidthError;
    pub use crate::layout::error::LayoutError;
    pub use crate::stream::StreamError;

    // Module-owned macros
    pub use crate::layout_config;

    // Re-export RSB essentials for convenience
    pub use rsb::prelude::*;
}

// Re-export prelude at crate root for convenience
pub use prelude::*;