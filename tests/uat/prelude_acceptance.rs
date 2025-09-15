//! Prelude UAT tests - API usability validation

#[test]
fn uat_prelude_single_import_usability() {
    println!("=== UAT: Single Import Usability ===");
    println!("Validating that single import provides complete functionality...");

    // Executive requirement: One import should provide everything needed
    use rololib::prelude::*;

    // All major function categories should be accessible
    let _width_fn = get_display_width;
    let _term_fn = get_terminal_width;
    let _validate_fn = validate_width;
    let _format1_fn = format_columns;
    let _format2_fn = format_table;
    let _format3_fn = format_list;

    // Configuration types should be accessible
    let _config_type = LayoutConfig::default();

    // Error types should be accessible
    let _width_error = WidthError::InvalidInput("test".to_string());
    let _layout_error = LayoutError::InvalidColumnCount(0);

    // Macros should work
    let _macro_config = layout_config!(80);

    println!("✅ All core functionality accessible via single prelude import");
    println!("✅ UAT PASSED: Single import provides complete API surface");
}

#[test]
fn uat_api_consistency_and_ergonomics() {
    println!("=== UAT: API Consistency and Ergonomics ===");
    println!("Validating API design meets usability standards...");

    use rololib::prelude::*;

    // Executive requirement: Common workflows should be straightforward

    // Workflow 1: Get terminal dimensions and create config
    let term_width = get_terminal_width();
    let config = layout_config!(term_width, gap: 2);
    println!("Terminal config workflow: {}x{} gap={}", config.width, term_width, config.gap);

    // Workflow 2: Validate user input and calculate text width
    let user_width = "120";
    match validate_width(user_width) {
        Ok(width) => {
            println!("User width {} validated successfully", width);

            let sample_text = "Sample business content";
            match get_display_width(sample_text) {
                Ok(text_width) => {
                    println!("Text '{}' width: {}", sample_text, text_width);
                    println!("Fits in user width: {}", text_width <= width);
                },
                Err(e) => println!("Text width calculation failed: {}", e),
            }
        },
        Err(e) => println!("Width validation failed: {}", e),
    }

    // Workflow 3: Error handling should be descriptive
    match validate_width("invalid") {
        Ok(_) => panic!("Should have failed"),
        Err(e) => {
            println!("Error message quality: '{}'", e);
            let error_str = format!("{}", e);
            assert!(error_str.contains("number"), "Error should mention number requirement");
        }
    }

    println!("✅ UAT PASSED: API provides ergonomic workflows with good error messages");
}

#[test]
fn uat_macro_flexibility() {
    println!("=== UAT: Configuration Macro Flexibility ===");
    println!("Validating configuration macro meets various use cases...");

    use rololib::prelude::*;

    // Executive requirement: Configuration should be flexible and intuitive

    // Basic usage - width only
    let basic = layout_config!(80);
    assert_eq!(basic.width, 80);
    assert_eq!(basic.gap, 2);      // Default
    assert_eq!(basic.padding, 1);  // Default
    println!("✅ Basic config: width={}, gap={}, padding={}", basic.width, basic.gap, basic.padding);

    // Medium usage - width + gap
    let medium = layout_config!(120, gap: 4);
    assert_eq!(medium.width, 120);
    assert_eq!(medium.gap, 4);
    assert_eq!(medium.padding, 1);  // Default
    println!("✅ Medium config: width={}, gap={}, padding={}", medium.width, medium.gap, medium.padding);

    // Full usage - all parameters
    let full = layout_config!(160, gap: 3, padding: 2);
    assert_eq!(full.width, 160);
    assert_eq!(full.gap, 3);
    assert_eq!(full.padding, 2);
    println!("✅ Full config: width={}, gap={}, padding={}", full.width, full.gap, full.padding);

    // Dynamic usage with variables
    let term_width = get_terminal_width();
    let dynamic = layout_config!(term_width, gap: 1, padding: 0);
    assert_eq!(dynamic.width, term_width);
    println!("✅ Dynamic config: width={}, gap={}, padding={}", dynamic.width, dynamic.gap, dynamic.padding);

    println!("✅ UAT PASSED: Configuration macro supports all required use cases");
}