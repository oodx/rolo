//! Width calculation utilities per MODULE_SPEC

use crate::width::error::WidthError;

#[cfg(feature = "width-boxy")]
use crate::width::width_boxy_adapter;

/// Calculate display width of text
/// Delegates to boxy adapter when available, fallback otherwise
pub fn get_display_width(text: &str) -> Result<usize, WidthError> {
    #[cfg(feature = "width-boxy")]
    {
        width_boxy_adapter::get_display_width(text)
    }
    #[cfg(not(feature = "width-boxy"))]
    {
        // Basic fallback without external dependencies
        Ok(text.chars().count())
    }
}

/// Get terminal width
/// Uses boxy adapter when available, fallback otherwise
pub fn get_terminal_width() -> usize {
    #[cfg(feature = "width-boxy")]
    {
        width_boxy_adapter::get_terminal_width()
    }
    #[cfg(not(feature = "width-boxy"))]
    {
        // Simple fallback without external dependencies
        if let Ok(var) = std::env::var("COLUMNS") {
            if let Ok(width) = var.parse::<usize>() {
                if width >= 10 { return width; }
            }
        }
        80
    }
}

/// Validate width input
pub fn validate_width(width_str: &str) -> Result<usize, WidthError> {
    #[cfg(feature = "width-boxy")]
    {
        width_boxy_adapter::validate_width(width_str)
    }
    #[cfg(not(feature = "width-boxy"))]
    {
        match width_str.parse::<usize>() {
            Ok(w) if w >= 10 && w <= 200 => Ok(w),
            Ok(w) => Err(WidthError::InvalidRange(w, 10, 200)),
            Err(_) => Err(WidthError::InvalidInput(format!("Width must be a number: {}", width_str))),
        }
    }
}