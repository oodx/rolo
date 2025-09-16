//! Internal stream processing helpers
//!
//! Private implementation details for stream processing that are not part of the public API.
//! These helpers support the main stream utilities with low-level operations.

#![allow(dead_code)] // Helper functions for future stream processing features

use super::error::{StreamError, StreamResult};
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

/// Buffer size for chunk reading operations
pub(crate) const CHUNK_SIZE: usize = 8192;

/// Maximum number of lines to read in memory at once
pub(crate) const MAX_LINES_BUFFER: usize = 10000;

/// Internal helper to detect if we're in a pipe
pub(crate) fn is_piped_input() -> bool {
    use std::os::unix::io::AsRawFd;

    let stdin_fd = io::stdin().as_raw_fd();
    unsafe {
        rsb::deps::libc::isatty(stdin_fd) == 0
    }
}

/// Internal helper to detect terminal width from environment
pub(crate) fn detect_terminal_width() -> Option<usize> {
    // Try COLUMNS environment variable first
    if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(width) = cols.parse::<usize>() {
            return Some(width);
        }
    }

    // Try to get terminal size using ioctl
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;

        let stdout_fd = io::stdout().as_raw_fd();
        let mut winsize = rsb::deps::libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        unsafe {
            if rsb::deps::libc::ioctl(stdout_fd, rsb::deps::libc::TIOCGWINSZ, &mut winsize) == 0 && winsize.ws_col > 0 {
                return Some(winsize.ws_col as usize);
            }
        }
    }

    // Default fallback
    Some(80)
}

/// Internal helper to read lines with memory protection
pub(crate) fn read_lines_chunked<R: BufRead>(reader: &mut R, max_lines: usize) -> StreamResult<Vec<String>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    for line_result in reader.lines() {
        if line_count >= max_lines {
            return Err(StreamError::BufferOverflow(max_lines));
        }

        let line = line_result.map_err(StreamError::from)?;
        lines.push(line);
        line_count += 1;
    }

    Ok(lines)
}

/// Internal helper to validate UTF-8 content
pub(crate) fn validate_utf8_content(content: &[u8]) -> StreamResult<String> {
    // First try direct conversion
    match String::from_utf8(content.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => {
            // Try to recover by replacing invalid sequences
            Ok(String::from_utf8_lossy(content).to_string())
        }
    }
}

/// Internal helper to handle SIGPIPE setup
pub(crate) fn setup_sigpipe_handler() {
    #[cfg(unix)]
    {
        extern "C" fn ignore_sigpipe(_: rsb::deps::libc::c_int) {
            // Do nothing - just ignore the signal
        }

        unsafe {
            rsb::deps::libc::signal(rsb::deps::libc::SIGPIPE, ignore_sigpipe as usize);
        }
    }
}

/// Internal helper to create a child process for piping
pub(crate) fn spawn_pipe_command(command: &str) -> StreamResult<std::process::Child> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| StreamError::IoError(format!("Failed to spawn command '{}': {}", command, e)))
}

/// Internal helper to estimate memory usage of string content
pub(crate) fn estimate_content_size(content: &str) -> usize {
    // Rough estimate: string length + overhead
    content.len() + (content.lines().count() * std::mem::size_of::<String>())
}

/// Internal helper to check if content might be binary
pub(crate) fn is_likely_binary(content: &[u8]) -> bool {
    // Simple heuristic: if more than 30% of the first 8KB contains null bytes or high-bit chars
    let sample_size = std::cmp::min(content.len(), 8192);
    let sample = &content[..sample_size];

    let suspicious_bytes = sample.iter()
        .filter(|&&b| b == 0 || b > 127)
        .count();

    (suspicious_bytes as f64 / sample_size as f64) > 0.30
}

/// Internal helper to split content preserving line endings
pub(crate) fn split_preserving_endings(content: &str) -> Vec<String> {
    if content.is_empty() {
        return Vec::new();
    }

    let mut lines = Vec::new();
    let mut start = 0;

    for (pos, ch) in content.char_indices() {
        if ch == '\n' {
            // Include the newline in the line
            lines.push(content[start..=pos].to_string());
            start = pos + 1;
        }
    }

    // Add any remaining content
    if start < content.len() {
        lines.push(content[start..].to_string());
    }

    lines
}

/// Internal helper for safe line joining
pub(crate) fn join_lines_safe(lines: &[String], separator: &str) -> String {
    if lines.is_empty() {
        return String::new();
    }

    // Pre-allocate capacity to avoid reallocations
    let total_len = lines.iter().map(|s| s.len()).sum::<usize>()
                    + (separator.len() * (lines.len().saturating_sub(1)));

    let mut result = String::with_capacity(total_len);

    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            result.push_str(separator);
        }
        result.push_str(line);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_content_size() {
        let content = "hello\nworld\n";
        let size = estimate_content_size(content);
        assert!(size > content.len()); // Should include overhead
    }

    #[test]
    fn test_is_likely_binary() {
        let text_content = b"Hello, world!\nThis is text.";
        assert!(!is_likely_binary(text_content));

        let binary_content = &[0u8; 100]; // All null bytes
        assert!(is_likely_binary(binary_content));
    }

    #[test]
    fn test_split_preserving_endings() {
        let content = "line1\nline2\nline3";
        let lines = split_preserving_endings(content);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line1\n");
        assert_eq!(lines[1], "line2\n");
        assert_eq!(lines[2], "line3"); // Last line without newline
    }

    #[test]
    fn test_join_lines_safe() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let result = join_lines_safe(&lines, " ");
        assert_eq!(result, "hello world");

        let empty_lines: Vec<String> = vec![];
        let result = join_lines_safe(&empty_lines, " ");
        assert_eq!(result, "");
    }
}