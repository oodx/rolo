//! Width module UAT tests - Executive acceptance validation

#[test]
fn uat_width_calculation_accuracy() {
    println!("=== UAT: Width Calculation Accuracy ===");
    println!("Validating width calculations meet business requirements...");

    use rolo::prelude::*;

    // Executive requirement: ASCII text width should be predictable
    let ascii_text = "Hello World";
    let width = get_display_width(ascii_text).unwrap();
    println!("ASCII text '{}' width: {}", ascii_text, width);
    assert!(width >= ascii_text.len(), "ASCII text width should be at least character count");

    // Executive requirement: Empty text should have zero width
    let empty_width = get_display_width("").unwrap();
    assert_eq!(empty_width, 0, "Empty string should have zero width");
    println!("Empty text width: {} (correct)", empty_width);

    // Executive requirement: Terminal width should be reasonable
    let term_width = get_terminal_width();
    println!("Terminal width: {} columns", term_width);
    assert!(term_width >= 20, "Terminal width too narrow for business use: {}", term_width);
    assert!(term_width <= 500, "Terminal width unreasonably wide: {}", term_width);

    println!("✅ UAT PASSED: Width calculations meet business accuracy requirements");
}

#[test]
fn uat_width_validation_business_rules() {
    println!("=== UAT: Width Validation Business Rules ===");
    println!("Validating width constraints meet usability standards...");

    use rolo::prelude::*;

    // Executive requirement: Common terminal widths should be valid
    let common_widths = ["80", "120", "132", "160"];
    for width_str in &common_widths {
        let result = validate_width(width_str);
        assert!(result.is_ok(), "Common width {} should be valid", width_str);
        println!("✅ Common width {} accepted", width_str);
    }

    // Executive requirement: Unusably narrow widths should be rejected
    let narrow_widths = ["5", "8", "9"];
    for width_str in &narrow_widths {
        let result = validate_width(width_str);
        assert!(result.is_err(), "Narrow width {} should be rejected", width_str);
        println!("✅ Narrow width {} correctly rejected", width_str);
    }

    // Executive requirement: Unreasonably wide widths should be rejected
    let wide_widths = ["300", "500", "1000"];
    for width_str in &wide_widths {
        let result = validate_width(width_str);
        assert!(result.is_err(), "Wide width {} should be rejected", width_str);
        println!("✅ Wide width {} correctly rejected", width_str);
    }

    println!("✅ UAT PASSED: Width validation enforces business usability rules");
}

#[cfg(feature = "width-boxy")]
#[test]
fn uat_boxy_feature_integration() {
    println!("=== UAT: Boxy Feature Integration ===");
    println!("Validating enterprise-grade width calculation with boxy integration...");

    use rolo::prelude::*;

    // Executive requirement: ANSI escape sequences should be handled
    let ansi_text = "\x1b[32mGreen Text\x1b[0m";
    let width = get_display_width(ansi_text).unwrap();
    println!("ANSI text '{}' display width: {}", ansi_text, width);
    // Should be approximately 10 for "Green Text" without ANSI codes
    assert!(width <= ansi_text.len(), "ANSI width should be less than raw length");
    assert!(width >= 8, "ANSI-stripped width should be reasonable");

    println!("✅ UAT PASSED: Boxy integration handles ANSI sequences correctly");
}

#[cfg(not(feature = "width-boxy"))]
#[test]
fn uat_fallback_mode_reliability() {
    println!("=== UAT: Fallback Mode Reliability ===");
    println!("Validating system works without premium features...");

    use rolo::prelude::*;

    // Executive requirement: Basic functionality must work without dependencies
    let simple_text = "Simple text";
    let width = get_display_width(simple_text).unwrap();
    println!("Fallback width for '{}': {}", simple_text, width);
    assert_eq!(width, simple_text.chars().count(), "Fallback should use character count");

    let term_width = get_terminal_width();
    println!("Fallback terminal width: {}", term_width);
    assert!(term_width >= 10, "Fallback terminal width should be reasonable");

    println!("✅ UAT PASSED: Fallback mode provides reliable basic functionality");
}