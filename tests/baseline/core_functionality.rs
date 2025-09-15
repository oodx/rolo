//! Core functionality tests - no features required
//!
//! Tests the basic functionality that should work in the default build
//! without any optional features enabled.

#[test]
fn test_basic_imports() {
    // Test that basic imports work without features
    use rolo::prelude::*;

    // Should be able to create default configurations
    let _layout_config = LayoutConfig::default();
    let _cli_config = CliConfig::default();
    let _stream_config = StreamConfig::default();
}

#[test]
fn test_cli_modes() {
    use rolo::prelude::*;

    // Test CLI mode enum works
    let mode = CliMode::Columns;
    assert!(matches!(mode, CliMode::Columns));

    let mode = CliMode::Table;
    assert!(matches!(mode, CliMode::Table));

    let mode = CliMode::List;
    assert!(matches!(mode, CliMode::List));
}

#[test]
fn test_basic_width_functionality() {
    use rolo::prelude::*;

    // Test basic width validation (should work without boxy feature)
    assert!(validate_width("80").is_ok());
    assert!(validate_width("5").is_err()); // Too small
    assert!(validate_width("300").is_err()); // Too large
}

#[test]
fn test_layout_placeholders() {
    use rolo::prelude::*;

    // Test that layout functions exist (even if they return placeholders)
    let result = format_columns("test\ndata", 2);
    assert!(result.is_ok());

    let result = format_table("test\tdata", "\t");
    assert!(result.is_ok());

    let result = format_list("test\ndata");
    assert!(result.is_ok());
}

#[test]
fn test_stream_operations_basic() {
    use rolo::prelude::*;

    // Test basic stream config and pipeline creation
    let config = StreamConfig::default();
    assert_eq!(config.max_buffer_size, 10 * 1024 * 1024);

    let _pipeline = Pipeline::new();
    // Pipeline should be creatable without features
}

#[test]
fn test_error_types() {
    use rolo::prelude::*;

    // Test that all error types are accessible
    let _layout_error = LayoutError::InvalidColumnCount(0);
    let _width_error = WidthError::InvalidRange(0, 10, 200);
    let _cli_error = CliError::InvalidArgument("test".to_string());
    let _stream_error = StreamError::UnexpectedEof;

    // Test error display
    assert!(format!("{}", _layout_error).contains("Invalid") || format!("{}", _layout_error).contains("column"));
    assert!(format!("{}", _width_error).contains("range") || format!("{}", _width_error).contains("Invalid"));
    assert!(format!("{}", _cli_error).contains("Invalid"));
    assert!(format!("{}", _stream_error).contains("Unexpected"));
}

#[test]
fn test_rsb_integration_basic() {
    // Test that RSB integration works in baseline
    let stream = rsb::prelude::Stream::from_string("test\ndata\n");
    let content = stream.to_string();
    assert!(content.contains("test"));
    assert!(content.contains("data"));
}

#[test]
fn test_line_endings() {
    use rolo::prelude::*;

    // Test line ending enum
    assert_eq!(LineEnding::Unix.as_str(), "\n");
    assert_eq!(LineEnding::Windows.as_str(), "\r\n");
    assert_eq!(LineEnding::Mac.as_str(), "\r");
}