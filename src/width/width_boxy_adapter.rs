//! Boxy width adapter - feature-gated width calculations
//! Adapts boxy's width calculation logic with RSB MODULE_SPEC patterns

use crate::width::error::WidthError;
use std::process::{Command, Stdio};
use std::fs::File;

#[cfg(feature = "width-boxy")]
use strip_ansi_escapes;
#[cfg(feature = "width-boxy")]
use unicode_width::UnicodeWidthStr;

/// Get terminal width with fallback to 80 columns
/// Adapted from boxy's width_plugin.rs with RSB patterns
pub fn get_terminal_width() -> usize {
    #[cfg(feature = "width-boxy")]
    {
        get_terminal_width_boxy()
    }
    #[cfg(not(feature = "width-boxy"))]
    {
        get_terminal_width_fallback()
    }
}

/// Calculate display width of text
/// Uses ANSI stripping and Unicode width calculation when boxy feature enabled
pub fn get_display_width(text: &str) -> Result<usize, WidthError> {
    #[cfg(feature = "width-boxy")]
    {
        get_display_width_boxy(text)
    }
    #[cfg(not(feature = "width-boxy"))]
    {
        get_display_width_fallback(text)
    }
}

#[cfg(feature = "width-boxy")]
fn get_terminal_width_boxy() -> usize {
    // Helper to run with /dev/tty
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output().ok().and_then(|o| String::from_utf8(o.stdout).ok())
    }

    // Try tput cols with tty (preferred)
    {
        let mut c = Command::new("tput");
        c.arg("cols");
        if let Some(out) = run_with_tty(c) {
            if let Ok(width) = out.trim().parse::<usize>() {
                if width >= 10 { return width; }
            }
        }
    }

    // Try stty size with tty
    {
        let mut c = Command::new("stty");
        c.arg("size");
        if let Some(out) = run_with_tty(c) {
            let parts: Vec<&str> = out.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(width) = parts[1].trim().parse::<usize>() {
                    if width >= 10 { return width; }
                }
            }
        }
    }

    80
}

#[cfg(feature = "width-boxy")]
fn get_display_width_boxy(text: &str) -> Result<usize, WidthError> {
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);
    Ok(UnicodeWidthStr::width(clean_str.as_ref()))
}

#[cfg(not(feature = "width-boxy"))]
fn get_terminal_width_fallback() -> usize {
    // Simple fallback without external dependencies
    if let Ok(var) = std::env::var("COLUMNS") {
        if let Ok(width) = var.parse::<usize>() {
            if width >= 10 { return width; }
        }
    }
    80
}

#[cfg(not(feature = "width-boxy"))]
fn get_display_width_fallback(text: &str) -> Result<usize, WidthError> {
    // Basic fallback - char count (not accurate for Unicode/ANSI but functional)
    Ok(text.chars().count())
}

/// Validate width input (from boxy)
pub fn validate_width(width_str: &str) -> Result<usize, WidthError> {
    match width_str.parse::<usize>() {
        Ok(w) if w >= 10 && w <= 200 => Ok(w),
        Ok(w) => Err(WidthError::InvalidRange(w, 10, 200)),
        Err(_) => Err(WidthError::InvalidInput(format!("Width must be a number: {}", width_str))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_width_returns_valid() {
        let width = get_terminal_width();
        assert!(width >= 10, "Terminal width should be at least 10, got {}", width);
        assert!(width <= 1000, "Terminal width should be reasonable, got {}", width);
    }

    #[test]
    fn test_display_width_basic() {
        assert!(get_display_width("hello").unwrap() >= 5);
        assert_eq!(get_display_width("").unwrap(), 0);
    }

    #[test]
    fn test_validate_width() {
        assert!(validate_width("80").is_ok());
        assert_eq!(validate_width("80").unwrap(), 80);
        assert!(validate_width("5").is_err());
        assert!(validate_width("300").is_err());
        assert!(validate_width("not_a_number").is_err());
    }

    #[cfg(feature = "width-boxy")]
    #[test]
    fn test_display_width_with_ansi() {
        let text_with_ansi = "\x1b[32mgreen\x1b[0m";
        let width = get_display_width_boxy(text_with_ansi).unwrap();
        assert_eq!(width, 5); // "green" without ANSI codes
    }

    #[cfg(not(feature = "width-boxy"))]
    #[test]
    fn test_fallback_behavior() {
        // Test that fallback works without panicking
        let width = get_terminal_width_fallback();
        assert!(width >= 10);

        let display_width = get_display_width_fallback("test").unwrap();
        assert_eq!(display_width, 4);
    }
}