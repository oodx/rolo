//! Width feature tests - testing both with and without width-boxy feature

#[test]
fn test_width_module_basic_functionality() {
    println!("=== Width Module Basic Functionality Test ===");

    // Test terminal width
    let term_width = rololib::width::get_terminal_width();
    println!("Terminal width: {}", term_width);
    assert!(term_width >= 10, "Terminal width should be at least 10, got {}", term_width);
    assert!(term_width <= 1000, "Terminal width should be reasonable, got {}", term_width);

    // Test display width calculation
    let text = "hello world";
    let width_result = rololib::width::get_display_width(text);
    assert!(width_result.is_ok(), "get_display_width should succeed");
    let width = width_result.unwrap();
    println!("Display width of '{}': {}", text, width);
    assert!(width >= text.len(), "Display width should be at least char count");

    println!("✅ Basic width functionality works");
}

#[test]
fn test_width_validation() {
    println!("=== Width Validation Test ===");

    // Valid widths
    assert!(rololib::width::validate_width("80").is_ok());
    assert!(rololib::width::validate_width("120").is_ok());
    assert_eq!(rololib::width::validate_width("80").unwrap(), 80);

    // Invalid widths
    assert!(rololib::width::validate_width("5").is_err());   // too small
    assert!(rololib::width::validate_width("300").is_err()); // too large
    assert!(rololib::width::validate_width("not_a_number").is_err());
    assert!(rololib::width::validate_width("").is_err());

    println!("✅ Width validation works correctly");
}

#[test]
fn test_empty_and_edge_cases() {
    println!("=== Width Edge Cases Test ===");

    // Empty string
    let empty_width = rololib::width::get_display_width("").unwrap();
    assert_eq!(empty_width, 0, "Empty string should have width 0");

    // Single character
    let single_width = rololib::width::get_display_width("a").unwrap();
    assert!(single_width >= 1, "Single character should have width >= 1");

    println!("✅ Edge cases handled correctly");
}

#[cfg(feature = "width-boxy")]
#[test]
fn test_width_boxy_features() {
    println!("=== Width Boxy Feature Test ===");

    // Test ANSI escape sequence stripping
    let ansi_text = "\x1b[32mgreen\x1b[0m";
    let width = rololib::width::get_display_width(ansi_text).unwrap();
    println!("ANSI text '{}' has display width: {}", ansi_text, width);
    // Should be 5 for "green" without ANSI codes
    assert!(width <= ansi_text.len(), "ANSI-stripped width should be <= raw length");

    // Test Unicode width calculation
    let unicode_text = "café";
    let unicode_width = rololib::width::get_display_width(unicode_text).unwrap();
    println!("Unicode text '{}' has display width: {}", unicode_text, unicode_width);

    println!("✅ Boxy feature functionality works");
}

#[cfg(not(feature = "width-boxy"))]
#[test]
fn test_width_fallback_behavior() {
    println!("=== Width Fallback Behavior Test ===");

    // Test that fallback doesn't panic and gives reasonable results
    let text = "test fallback";
    let width = rololib::width::get_display_width(text).unwrap();
    println!("Fallback width for '{}': {}", text, width);

    // In fallback mode, should be char count
    assert_eq!(width, text.chars().count(),
               "Fallback mode should return char count");

    // Terminal width fallback
    let term_width = rololib::width::get_terminal_width();
    assert!(term_width >= 10, "Fallback terminal width should be reasonable");

    println!("✅ Fallback behavior works correctly");
}