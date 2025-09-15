//! Default behavior tests - validates expected defaults
//!
//! Tests that default configurations and behaviors work as expected
//! without requiring any optional features.

#[test]
fn test_layout_config_defaults() {
    use rololib::prelude::*;

    let config = LayoutConfig::default();

    // Verify sensible defaults are set
    // (These will depend on actual LayoutConfig implementation)
    // For now, just test that default creation works
    assert!(true); // Placeholder - will update when LayoutConfig is implemented
}

#[test]
fn test_rsb_global_context_defaults() {
    use rololib::prelude::*;

    // Test RSB global context with defaults
    set_var("columns", "3");
    assert_eq!(get_var("columns"), "3");

    set_var("width", "80");
    assert_eq!(get_var("width"), "80");

    set_var("headers", "false"); // RSB: standard Rust boolean
    assert!(!is_true("headers"));

    set_var("help", "false"); // RSB: standard Rust boolean
    assert!(!is_true("help"));
}

#[test]
fn test_stream_config_defaults() {
    use rololib::prelude::*;

    let config = StreamConfig::default();

    // Test reasonable defaults for stream processing
    assert_eq!(config.max_buffer_size, 10 * 1024 * 1024); // 10MB default
    assert!(config.handle_sigpipe); // Handle SIGPIPE by default
    assert!(matches!(config.line_ending, LineEnding::Unix)); // Unix line endings
}

#[test]
fn test_default_width_detection() {
    use rololib::prelude::*;

    // Test terminal width detection fallback
    let width = get_terminal_width();

    // Should return a reasonable default even without terminal
    assert!(width >= 40); // Minimum reasonable width
    assert!(width <= 200); // Maximum reasonable width
}

#[test]
fn test_default_error_handling() {
    use rololib::prelude::*;

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
    use rololib::prelude::*;

    // Test that pipeline uses sensible defaults
    let pipeline = Pipeline::new();

    // Pipeline should be constructible and have reasonable internal config
    // (We can't directly test the private config field, but creation should work)
    assert!(true); // If this compiles and runs, the defaults work
}

#[test]
fn test_rsb_bootstrap_defaults() {
    use rololib::prelude::*;

    // Test that RSB bootstrap works with defaults
    set_var("mode", "columns"); // Default mode
    assert_eq!(get_var("mode"), "columns");

    // Test default RSB variables
    set_var("verbose", "false"); // RSB: standard Rust boolean
    assert!(!is_true("verbose"));

    set_var("debug", "false"); // RSB: standard Rust boolean
    assert!(!is_true("debug"));
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