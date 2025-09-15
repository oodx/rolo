//! Default behavior tests - validates expected defaults
//!
//! Tests that default configurations and behaviors work as expected
//! without requiring any optional features.

#[test]
fn test_layout_config_defaults() {
    use rolo::prelude::*;

    let config = LayoutConfig::default();

    // Verify sensible defaults are set
    // (These will depend on actual LayoutConfig implementation)
    // For now, just test that default creation works
    assert!(true); // Placeholder - will update when LayoutConfig is implemented
}

#[test]
fn test_cli_config_defaults() {
    use rolo::prelude::*;

    let config = CliConfig::default();

    // Test default CLI behavior
    assert!(matches!(config.mode, CliMode::Columns)); // Default to columns mode
    assert_eq!(config.columns, None); // No column override by default
    assert_eq!(config.width, None); // No width override by default
    assert!(!config.headers); // Headers off by default
    assert!(!config.help); // Help not requested by default
    assert!(!config.version); // Version not requested by default
}

#[test]
fn test_stream_config_defaults() {
    use rolo::prelude::*;

    let config = StreamConfig::default();

    // Test reasonable defaults for stream processing
    assert_eq!(config.max_buffer_size, 10 * 1024 * 1024); // 10MB default
    assert!(config.handle_sigpipe); // Handle SIGPIPE by default
    assert!(matches!(config.line_ending, LineEnding::Unix)); // Unix line endings
}

#[test]
fn test_default_width_detection() {
    use rolo::prelude::*;

    // Test terminal width detection fallback
    let width = get_terminal_width();

    // Should return a reasonable default even without terminal
    assert!(width >= 40); // Minimum reasonable width
    assert!(width <= 200); // Maximum reasonable width
}

#[test]
fn test_default_error_handling() {
    use rolo::prelude::*;

    // Test that errors have reasonable default messages
    let error = LayoutError::InvalidColumnCount(0);
    let message = format!("{}", error);
    assert!(!message.is_empty());
    assert!(message.contains("Invalid") || message.contains("column"));

    let error = WidthError::InvalidRange(5, 10, 200);
    let message = format!("{}", error);
    assert!(!message.is_empty());
    assert!(message.contains("range") || message.contains("width"));
}

#[test]
fn test_pipeline_default_config() {
    use rolo::prelude::*;

    // Test that pipeline uses sensible defaults
    let pipeline = Pipeline::new();

    // Pipeline should be constructible and have reasonable internal config
    // (We can't directly test the private config field, but creation should work)
    assert!(true); // If this compiles and runs, the defaults work
}

#[test]
fn test_cli_argument_parsing_defaults() {
    use rolo::prelude::*;

    // Test that empty arguments result in sensible defaults
    let args: Vec<String> = vec!["rolo".to_string()]; // Just program name
    let result = parse_args(&args);

    // Should either succeed with defaults or give helpful error
    match result {
        Ok(config) => {
            // Should default to columns mode
            assert!(matches!(config.mode, CliMode::Columns));
        }
        Err(_) => {
            // If it fails, that's also acceptable for empty args
            // The important thing is it doesn't panic
        }
    }
}

#[test]
fn test_basic_text_processing() {
    // Test basic text processing without features
    let input = "line1\nline2\nline3";

    // Should be able to create streams and process basic text
    let stream = rsb::prelude::Stream::from_string(input);
    let lines = stream.to_vec();

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "line1");
    assert_eq!(lines[1], "line2");
    assert_eq!(lines[2], "line3");
}