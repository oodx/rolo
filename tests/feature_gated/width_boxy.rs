//! Width calculation tests with boxy adapter
//!
//! Tests width functionality that requires the width-boxy feature

#[test]
fn test_boxy_adapter_available() {
    // Test that boxy adapter functions are available when feature is enabled
    use rolo::prelude::*;

    // Test basic width calculation
    let width = get_display_width("hello world").expect("Width calculation should succeed");
    assert_eq!(width, 11);

    // Test ANSI sequence handling (if strip-ansi-escapes is available)
    let width = get_display_width("\x1b[31mred text\x1b[0m").expect("ANSI width calculation should succeed");
    assert_eq!(width, 8); // Should strip ANSI and count actual characters
}

#[test]
fn test_unicode_width_calculation() {
    use rolo::prelude::*;

    // Test Unicode width calculation with boxy feature
    let width = get_display_width("hello 世界").expect("Unicode width calculation should succeed");
    assert_eq!(width, 10); // 6 ASCII + 4 for two wide characters

    // Test combining characters
    let width = get_display_width("é").expect("Combining character width calculation should succeed"); // e + combining accent
    assert_eq!(width, 1);

    // Test zero-width characters
    let width = get_display_width("hello\u{200B}world").expect("Zero-width character calculation should succeed"); // Zero-width space
    assert_eq!(width, 10);
}

#[test]
fn test_ansi_stripping() {
    use rolo::prelude::*;

    // Test various ANSI escape sequences
    let test_cases = vec![
        ("\x1b[31mred\x1b[0m", 3, "Basic color"),
        ("\x1b[1;32mbold green\x1b[0m", 10, "Bold color"),
        ("\x1b[38;5;196mhigh color\x1b[0m", 10, "256-color"),
        ("\x1b[38;2;255;0;0mrgb color\x1b[0m", 9, "RGB color"),
        ("normal\x1b[Ktext", 10, "Clear line"),
        ("\x1b[2J\x1b[Hclear", 5, "Clear screen"),
    ];

    for (input, expected_width, description) in test_cases {
        let width = get_display_width(input).expect("ANSI width calculation should succeed");
        assert_eq!(width, expected_width, "Failed for case: {}", description);
    }
}

#[test]
fn test_mixed_content_width() {
    use rolo::prelude::*;

    // Test complex mixed content
    let mixed = "\x1b[31m世界\x1b[0m hello \x1b[1mworld\x1b[0m";
    let width = get_display_width(mixed).expect("Mixed content width calculation should succeed");
    assert_eq!(width, 16); // 4 (世界) + 7 ( hello ) + 5 (world) = 16, ANSI codes stripped

    // Test empty string
    let width = get_display_width("").expect("Empty string width calculation should succeed");
    assert_eq!(width, 0);

    // Test only ANSI codes
    let width = get_display_width("\x1b[31m\x1b[0m").expect("ANSI-only width calculation should succeed");
    assert_eq!(width, 0);
}

#[test]
fn test_width_validation_with_boxy() {
    use rolo::prelude::*;

    // Test that width validation works properly with boxy feature
    assert!(validate_width("10").is_ok());
    assert!(validate_width("80").is_ok());
    assert!(validate_width("200").is_ok());

    assert!(validate_width("0").is_err());
    assert!(validate_width("5").is_err());
    assert!(validate_width("300").is_err());
}

#[test]
fn test_terminal_width_detection_boxy() {
    use rolo::prelude::*;

    // Test terminal width detection with boxy features
    let width = get_terminal_width();

    // Should return a reasonable width
    assert!(width >= 10);
    assert!(width <= 300);

    // Width should be consistent between calls
    let width2 = get_terminal_width();
    assert_eq!(width, width2);
}

#[test]
fn test_boxy_integration_edge_cases() {
    use rolo::prelude::*;

    // Test edge cases that might occur with boxy integration

    // Very long strings
    let long_string = "a".repeat(1000);
    let width = get_display_width(&long_string).expect("Long string width calculation should succeed");
    assert_eq!(width, 1000);

    // Mixed long content with ANSI
    let long_ansi = format!("\x1b[31m{}\x1b[0m", "a".repeat(500));
    let width = get_display_width(&long_ansi).expect("Long ANSI string width calculation should succeed");
    assert_eq!(width, 500);

    // Malformed ANSI sequences (should be handled gracefully)
    let malformed = "\x1b[31mtext\x1b["; // Incomplete escape
    let width = get_display_width(malformed).expect("Malformed ANSI width calculation should succeed");
    assert!(width > 0); // Should not crash, should count something
}

#[test]
fn test_width_consistency() {
    use rolo::prelude::*;

    // Test that width calculations are consistent
    let text = "hello world";
    let width1 = get_display_width(text).expect("First width calculation should succeed");
    let width2 = get_display_width(text).expect("Second width calculation should succeed");
    assert_eq!(width1, width2);

    // Test that concatenated strings have additive width
    let part1 = "hello";
    let part2 = " world";
    let combined = format!("{}{}", part1, part2);

    let width_part1 = get_display_width(part1).expect("Part1 width calculation should succeed");
    let width_part2 = get_display_width(part2).expect("Part2 width calculation should succeed");
    let width_combined = get_display_width(&combined).expect("Combined width calculation should succeed");

    assert_eq!(width_combined, width_part1 + width_part2);
}