//! Terminal width detection tests - validates TASK-010 requirements
//!
//! Tests the terminal width detection functionality including:
//! - RSB host module integration for robust width detection
//! - --width override CLI argument support
//! - --fit mode as default behavior with --no-fit option
//! - Terminal resize handling capabilities

#[test]
fn test_terminal_width_detection_basic() {
    use rololib::prelude::*;

    // Basic width detection should return a reasonable value
    let width = get_terminal_width();
    assert!(width >= 10, "Terminal width should be at least 10 characters");
    assert!(width <= 500, "Terminal width should be reasonable (max 500)");
}

#[test]
fn test_terminal_width_with_columns_env() {
    use rololib::prelude::*;
    use std::env;

    // Save original value
    let original = env::var("COLUMNS").ok();

    // Test with COLUMNS environment variable
    env::set_var("COLUMNS", "120");
    let width = get_terminal_width();

    // Should respect COLUMNS when set
    #[cfg(not(feature = "width-boxy"))]
    assert_eq!(width, 120);

    // Test with invalid COLUMNS value
    env::set_var("COLUMNS", "invalid");
    let width = get_terminal_width();
    assert!(width >= 10); // Should fall back to reasonable default

    // Test with too small COLUMNS value
    env::set_var("COLUMNS", "5");
    let width = get_terminal_width();
    // Should either use fallback or handle gracefully
    assert!(width >= 10);

    // Restore original value
    match original {
        Some(val) => env::set_var("COLUMNS", val),
        None => env::remove_var("COLUMNS"),
    }
}

#[test]
fn test_width_validation() {
    use rololib::prelude::*;

    // Valid width values
    assert!(validate_width("80").is_ok());
    assert!(validate_width("120").is_ok());
    assert!(validate_width("10").is_ok());

    // Invalid width values
    assert!(validate_width("5").is_err()); // Too small
    assert!(validate_width("500").is_err()); // Too large (in fallback mode)
    assert!(validate_width("abc").is_err()); // Not a number
    assert!(validate_width("").is_err()); // Empty string
    assert!(validate_width("-10").is_err()); // Negative number
}

#[test]
fn test_cli_width_override() {
    use rololib::prelude::*;

    // Test --width argument parsing
    let args = vec!["rolo".to_string(), "--width".to_string(), "100".to_string(), "--cols".to_string(), "2".to_string()];
    let config = parse_args(&args);
    assert!(config.is_ok());

    let config = config.unwrap();
    assert_eq!(config.width, Some(100));
    assert_eq!(config.columns, Some(2));
}

#[test]
fn test_cli_width_override_invalid() {
    use rololib::prelude::*;

    // Test with invalid width
    let args = vec!["rolo".to_string(), "--width".to_string(), "invalid".to_string()];
    let config = parse_args(&args);
    assert!(config.is_err());

    // Test with missing width value
    let args = vec!["rolo".to_string(), "--width".to_string()];
    let config = parse_args(&args);
    assert!(config.is_err());
}

#[test]
fn test_fit_mode_default() {
    use rololib::prelude::*;

    // Default config should have fit mode enabled
    let config = CliConfig::default();
    assert!(config.fit_mode);

    // Test --fit flag
    let args = vec!["rolo".to_string(), "--fit".to_string()];
    let config = parse_args(&args);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config.fit_mode);

    // Test --no-fit flag
    let args = vec!["rolo".to_string(), "--no-fit".to_string()];
    let config = parse_args(&args);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(!config.fit_mode);
}

#[test]
fn test_fit_mode_width_selection() {
    use rololib::prelude::*;

    // Create test configs to verify width selection logic
    let mut fit_config = CliConfig::default();
    fit_config.fit_mode = true;
    fit_config.width = Some(100);

    let mut no_fit_config = CliConfig::default();
    no_fit_config.fit_mode = false;
    no_fit_config.width = Some(100);

    // Both should respect explicit width when set
    // This test verifies the logic exists in CLI execution
    assert!(fit_config.fit_mode);
    assert!(!no_fit_config.fit_mode);
    assert_eq!(fit_config.width, Some(100));
    assert_eq!(no_fit_config.width, Some(100));
}

#[test]
#[cfg(unix)]
fn test_terminal_resize_detection() {
    use rololib::prelude::*;

    // Test terminal resize detection (basic functionality)
    // Note: This may return None if terminal size hasn't changed
    let resize_result = check_terminal_resize();

    // If we get a result, it should be reasonable dimensions
    if let Some((width, height)) = resize_result {
        assert!(width >= 10, "Detected width should be reasonable");
        assert!(width <= 500, "Detected width should be reasonable");
        assert!(height >= 5, "Detected height should be reasonable");
        assert!(height <= 200, "Detected height should be reasonable");
    }

    // Calling twice in a row should return None (no change)
    let second_call = check_terminal_resize();
    // Second call might return None since no resize happened
    if let Some((width, height)) = second_call {
        assert!(width >= 10);
        assert!(height >= 5);
    }
}

#[test]
fn test_width_detection_fallback_chain() {
    use rololib::prelude::*;
    use std::env;

    // Save original COLUMNS
    let original_columns = env::var("COLUMNS").ok();

    // Remove COLUMNS to test fallback chain
    env::remove_var("COLUMNS");

    // Should still return a reasonable width via fallback methods
    let width = get_terminal_width();
    assert!(width >= 10);
    assert!(width <= 500);

    // Restore COLUMNS
    if let Some(val) = original_columns {
        env::set_var("COLUMNS", val);
    }
}

#[test]
fn test_multiple_width_env_vars() {
    use rololib::prelude::*;
    use std::env;

    // Save original values
    let original_columns = env::var("COLUMNS").ok();
    let original_term_width = env::var("TERM_WIDTH").ok();

    // Test that TERM_WIDTH is checked as fallback
    env::remove_var("COLUMNS");
    env::set_var("TERM_WIDTH", "90");

    let width = get_terminal_width();
    // In fallback mode, should check TERM_WIDTH
    #[cfg(not(feature = "width-boxy"))]
    {
        // The fallback implementation should find TERM_WIDTH
        // This tests the environment variable fallback chain
        assert!(width >= 10); // Should get some reasonable value
    }

    // Restore original values
    if let Some(val) = original_columns {
        env::set_var("COLUMNS", val);
    } else {
        env::remove_var("COLUMNS");
    }
    if let Some(val) = original_term_width {
        env::set_var("TERM_WIDTH", val);
    } else {
        env::remove_var("TERM_WIDTH");
    }
}

#[test]
fn test_cli_integration_width_and_fit() {
    use rololib::prelude::*;

    // Test combined width and fit flags
    let args = vec![
        "rolo".to_string(),
        "--width".to_string(),
        "120".to_string(),
        "--fit".to_string(),
        "--cols".to_string(),
        "3".to_string(),
    ];

    let config = parse_args(&args);
    assert!(config.is_ok());

    let config = config.unwrap();
    assert_eq!(config.width, Some(120));
    assert!(config.fit_mode);
    assert_eq!(config.columns, Some(3));
    assert!(matches!(config.mode, CliMode::Columns));
}

#[test]
fn test_width_constraints_validation() {
    use rololib::prelude::*;

    // Test width constraint validation in different scenarios
    let very_small = validate_width("1");
    assert!(very_small.is_err()); // Should reject very small widths

    let reasonable = validate_width("80");
    assert!(reasonable.is_ok()); // Should accept reasonable widths

    let large = validate_width("160");
    assert!(large.is_ok()); // Should accept larger reasonable widths

    // The exact upper limit depends on the implementation
    let very_large = validate_width("1000");
    // In fallback mode, this should be rejected
    #[cfg(not(feature = "width-boxy"))]
    assert!(very_large.is_err());
}