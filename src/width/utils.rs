//! Width calculation utilities per MODULE_SPEC

use crate::width::error::WidthError;

// Use libc for terminal detection on Unix systems
#[cfg(all(unix, feature = "libc"))]
use libc;

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

/// Get terminal width with enhanced detection
/// Integrates RSB host module for robust environment detection
pub fn get_terminal_width() -> usize {
    #[cfg(feature = "width-boxy")]
    {
        width_boxy_adapter::get_terminal_width()
    }
    #[cfg(not(feature = "width-boxy"))]
    {
        get_terminal_width_rsb()
    }
}

/// RSB-based terminal width detection (fallback implementation)
/// Provides robust width detection using multiple methods
pub fn get_terminal_width_rsb() -> usize {
    // Method 1: Check COLUMNS environment variable (set by shell)
    if let Ok(var) = std::env::var("COLUMNS") {
        if let Ok(width) = var.parse::<usize>() {
            if width >= 10 && width <= 500 {
                return width;
            }
        }
    }

    // Method 2: Query terminal via ioctl (Unix-like systems)
    #[cfg(all(unix, feature = "libc"))]
    {
        if let Some(width) = get_terminal_width_ioctl() {
            if width >= 10 && width <= 500 {
                return width;
            }
        }
    }

    // Method 3: Try tput command
    if let Some(width) = get_terminal_width_tput() {
        if width >= 10 && width <= 500 {
            return width;
        }
    }

    // Method 4: Check other common environment variables
    for var_name in &["TERM_WIDTH", "WIDTH", "TERMWIDTH"] {
        if let Ok(var) = std::env::var(var_name) {
            if let Ok(width) = var.parse::<usize>() {
                if width >= 10 && width <= 500 {
                    return width;
                }
            }
        }
    }

    // Default fallback
    80
}

/// Detect terminal width using ioctl system call (Unix-like systems)
#[cfg(all(unix, feature = "libc"))]
fn get_terminal_width_ioctl() -> Option<usize> {
    use std::os::unix::io::AsRawFd;

    unsafe {
        let mut winsize: libc::winsize = std::mem::zeroed();
        let result = libc::ioctl(
            std::io::stdout().as_raw_fd(),
            libc::TIOCGWINSZ,
            &mut winsize as *mut libc::winsize,
        );

        if result == 0 && winsize.ws_col > 0 {
            Some(winsize.ws_col as usize)
        } else {
            None
        }
    }
}

/// Detect terminal width using tput command
fn get_terminal_width_tput() -> Option<usize> {
    if let Ok(output) = std::process::Command::new("tput")
        .arg("cols")
        .output()
    {
        if output.status.success() {
            let width_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(width) = width_str.trim().parse::<usize>() {
                return Some(width);
            }
        }
    }
    None
}

/// Check if terminal size has changed since last check
/// Returns (width, height) if changed, None if unchanged
#[cfg(all(unix, feature = "libc"))]
pub fn check_terminal_resize() -> Option<(usize, usize)> {
    use std::sync::atomic::{AtomicU16, Ordering};

    static LAST_WIDTH: AtomicU16 = AtomicU16::new(0);
    static LAST_HEIGHT: AtomicU16 = AtomicU16::new(0);

    if let Some((width, height)) = get_terminal_size_ioctl() {
        let last_width = LAST_WIDTH.load(Ordering::Relaxed);
        let last_height = LAST_HEIGHT.load(Ordering::Relaxed);

        if width as u16 != last_width || height as u16 != last_height {
            LAST_WIDTH.store(width as u16, Ordering::Relaxed);
            LAST_HEIGHT.store(height as u16, Ordering::Relaxed);
            return Some((width, height));
        }
    }

    None
}

/// Get terminal size (width, height) using ioctl
#[cfg(all(unix, feature = "libc"))]
fn get_terminal_size_ioctl() -> Option<(usize, usize)> {
    use std::os::unix::io::AsRawFd;

    unsafe {
        let mut winsize: libc::winsize = std::mem::zeroed();
        let result = libc::ioctl(
            std::io::stdout().as_raw_fd(),
            libc::TIOCGWINSZ,
            &mut winsize as *mut libc::winsize,
        );

        if result == 0 && winsize.ws_col > 0 && winsize.ws_row > 0 {
            Some((winsize.ws_col as usize, winsize.ws_row as usize))
        } else {
            None
        }
    }
}

/// Terminal width change detection (fallback for non-Unix or no libc)
#[cfg(not(all(unix, feature = "libc")))]
pub fn check_terminal_resize() -> Option<(usize, usize)> {
    // On non-Unix systems or without libc, fall back to environment variable checking
    // This is less reliable but provides basic functionality
    None
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