//! Prelude functionality tests - RSB MODULE_SPEC compliant
//!
//! Tests that all prelude exports work correctly

#[test]
fn test_prelude_layout_exports() {
    println!("=== Prelude Layout Exports Test ===");

    // Test that layout functions are available via prelude
    use rololib::prelude::*;

    let config = LayoutConfig::default();
    println!("Default layout config: width={}, gap={}, padding={}",
             config.width, config.gap, config.padding);

    // Test layout functions
    let _result = format_columns("test", 2);
    let _result = format_table("test", ",");
    let _result = format_list("test");

    println!("✅ Layout exports work via prelude");
}

#[test]
fn test_prelude_width_exports() {
    println!("=== Prelude Width Exports Test ===");

    use rololib::prelude::*;

    // Test width functions
    let term_width = get_terminal_width();
    assert!(term_width >= 10, "Terminal width should be reasonable");
    println!("Terminal width: {}", term_width);

    let display_width = get_display_width("test").unwrap();
    println!("Display width of 'test': {}", display_width);

    // Test width validation
    let validated = validate_width("80").unwrap();
    assert_eq!(validated, 80);
    println!("Width validation works: 80 -> {}", validated);

    println!("✅ Width exports work via prelude");
}

#[test]
fn test_prelude_error_exports() {
    println!("=== Prelude Error Exports Test ===");

    use rololib::prelude::*;

    // Test error types are available
    let _width_error = WidthError::InvalidInput("test".to_string());
    let _layout_error = LayoutError::InvalidColumnCount(0);

    println!("✅ Error types available via prelude");
}

#[test]
fn test_prelude_macro_exports() {
    println!("=== Prelude Macro Exports Test ===");

    use rololib::prelude::*;

    // Test layout_config macro
    let config1 = layout_config!(100);
    assert_eq!(config1.width, 100);
    assert_eq!(config1.gap, 2);
    assert_eq!(config1.padding, 1);

    let config2 = layout_config!(120, gap: 4);
    assert_eq!(config2.width, 120);
    assert_eq!(config2.gap, 4);
    assert_eq!(config2.padding, 1);

    let config3 = layout_config!(140, gap: 3, padding: 2);
    assert_eq!(config3.width, 140);
    assert_eq!(config3.gap, 3);
    assert_eq!(config3.padding, 2);

    println!("✅ layout_config! macro works via prelude");
}

#[test]
fn test_prelude_comprehensive() {
    println!("=== Comprehensive Prelude Test ===");

    // Test that everything works together
    use rololib::prelude::*;

    // Get terminal dimensions
    let term_width = get_terminal_width();

    // Create a layout config using the macro
    let config = layout_config!(term_width, gap: 2, padding: 1);

    // Validate a width string
    let user_width = validate_width("80").unwrap();

    // Calculate display width
    let text = "Hello, world!";
    let text_width = get_display_width(text).unwrap();

    println!("Terminal: {} cols, User: {} cols, Text: '{}' = {} cols",
             term_width, user_width, text, text_width);

    // Format some content (placeholder implementations)
    let _formatted = format_columns(text, 2);
    let _table = format_table(text, ",");
    let _list = format_list(text);

    println!("✅ All prelude functionality integrates correctly");
}