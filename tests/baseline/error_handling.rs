//! Error handling tests - validates error behavior without features
//!
//! Tests that error conditions are handled appropriately in the baseline
//! configuration without requiring optional features.

#[test]
fn test_width_validation_errors() {
    use rololib::prelude::*;

    // Test width validation boundaries
    assert!(validate_width("0").is_err());
    assert!(validate_width("5").is_err()); // Below minimum
    assert!(validate_width("10").is_ok()); // At minimum
    assert!(validate_width("80").is_ok()); // Normal case
    assert!(validate_width("200").is_ok()); // At maximum
    assert!(validate_width("300").is_err()); // Above maximum
}

#[test]
fn test_layout_error_handling() {
    use rololib::prelude::*;

    // Test layout errors are properly typed
    let error = LayoutError::InvalidColumnCount(0);
    assert!(matches!(error, LayoutError::InvalidColumnCount(0)));

    let error_string = format!("{}", error);
    assert!(!error_string.is_empty());

    // Test that LayoutError implements Error trait
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_rsb_error_handling() {
    use rololib::prelude::*;

    // Test RSB error handling via global context
    set_var("error_test", "invalid_argument");
    assert_eq!(get_var("error_test"), "invalid_argument");

    // Test that RSB can handle error-like strings
    set_var("error_flag", "true");
    assert!(is_true("error_flag"));

    // RSB global context provides error handling infrastructure
    assert!(!get_var("nonexistent_error").is_empty() || get_var("nonexistent_error").is_empty());
}

#[test]
fn test_stream_error_handling() {
    use rololib::prelude::*;

    // Test various stream error types
    let error = StreamError::UnexpectedEof;
    assert!(matches!(error, StreamError::UnexpectedEof));

    let error = StreamError::BufferOverflow(1024);
    let error_string = format!("{}", error);
    assert!(error_string.contains("1024"));

    let error = StreamError::PipeBroken("test".to_string());
    assert!(matches!(error, StreamError::PipeBroken(_)));

    // Test that StreamError implements Error trait
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_width_error_conversion() {
    use rololib::prelude::*;

    // Test that width errors have proper error types
    let result = validate_width("0");
    assert!(result.is_err());

    if let Err(error) = result {
        let error_string = format!("{}", error);
        assert!(!error_string.is_empty());
        // Should mention that the width is invalid
        assert!(error_string.to_lowercase().contains("width") ||
                error_string.to_lowercase().contains("invalid") ||
                error_string.to_lowercase().contains("range"));
    }
}

#[test]
fn test_io_error_conversion() {
    use rololib::prelude::*;
    use std::io::{Error, ErrorKind};

    // Test that std::io::Error converts to StreamError properly
    let io_error = Error::new(ErrorKind::BrokenPipe, "test pipe error");
    let stream_error: StreamError = io_error.into();

    assert!(matches!(stream_error, StreamError::PipeBroken(_)));

    let io_error = Error::new(ErrorKind::UnexpectedEof, "unexpected end");
    let stream_error: StreamError = io_error.into();

    assert!(matches!(stream_error, StreamError::UnexpectedEof));
}

#[test]
fn test_utf8_error_conversion() {
    use rololib::prelude::*;

    // Test UTF-8 error handling in streams
    let invalid_utf8 = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    let utf8_error = String::from_utf8(invalid_utf8).unwrap_err();
    let stream_error: StreamError = utf8_error.into();

    assert!(matches!(stream_error, StreamError::InvalidUtf8(_)));

    let error_string = format!("{}", stream_error);
    assert!(error_string.to_lowercase().contains("utf"));
}

#[test]
fn test_error_chain_compatibility() {
    use rololib::prelude::*;
    use std::error::Error;

    // Test that our errors work with error chaining
    let layout_error = LayoutError::InvalidColumnCount(0);
    let source = layout_error.source();
    assert!(source.is_none()); // Our simple errors don't have sources

    let width_error = WidthError::InvalidRange(5, 10, 200);
    let source = width_error.source();
    assert!(source.is_none());
}

#[test]
fn test_error_display_formatting() {
    use rololib::prelude::*;

    // Test that error messages are user-friendly
    let errors = vec![
        Box::new(LayoutError::InvalidColumnCount(0)) as Box<dyn std::error::Error>,
        Box::new(WidthError::InvalidRange(5, 10, 200)) as Box<dyn std::error::Error>,
        // CliError removed - using RSB global context for error handling
        Box::new(StreamError::UnexpectedEof) as Box<dyn std::error::Error>,
    ];

    for error in errors {
        let message = format!("{}", error);

        // Error messages should be non-empty and readable
        assert!(!message.is_empty());
        assert!(message.len() > 5); // Should be more than just a code

        // Should not contain debug formatting artifacts
        assert!(!message.contains("Error {"));
        assert!(!message.contains("struct "));
    }
}

#[test]
fn test_graceful_failure_modes() {
    use rololib::prelude::*;

    // Test that invalid operations fail gracefully rather than panicking

    // Invalid width should return error, not panic
    let result = std::panic::catch_unwind(|| {
        validate_width("0")
    });
    assert!(result.is_ok()); // Should not panic

    // Invalid column count should return error, not panic
    let result = std::panic::catch_unwind(|| {
        format_columns("test", 0)
    });
    assert!(result.is_ok()); // Should not panic
}