//! Baseline sanity tests - visible demos per RSB HOWTO_TEST

#[test]
fn test_project_compiles() {
    println!("=== Rolo Project Structure Test ===");
    println!("Testing that the project compiles and modules are accessible...");

    // Test that we can access the core modules
    let _layout_config = rololib::layout::utils::LayoutConfig::default();
    println!("✅ Layout module accessible");

    let _width = rololib::width::get_terminal_width();
    println!("✅ Width module accessible");

    // Test prelude imports
    use rololib::prelude::*;
    let _config = LayoutConfig::default();
    println!("✅ Prelude imports work");

    println!("✅ All basic structure tests passed!");
}

#[test]
fn test_module_structure_compliance() {
    println!("=== MODULE_SPEC Compliance Test ===");
    println!("Verifying MODULE_SPEC structure is followed...");

    // Test that error types exist
    let _layout_error = rololib::layout::error::LayoutError::InvalidColumnCount(0);
    println!("✅ Layout error types defined");

    let _width_error = rololib::width::error::WidthError::InvalidInput("test".to_string());
    println!("✅ Width error types defined");

    // Test placeholder functions exist
    let _result = rololib::layout::utils::format_columns("test", 2);
    println!("✅ Layout utilities accessible");

    let _result = rololib::width::utils::get_display_width("test");
    println!("✅ Width utilities accessible");

    // Test RSB global context
    rsb::prelude::set_var("test", "value");
    assert_eq!(rsb::prelude::get_var("test"), "value");
    println!("✅ RSB global context accessible");

    println!("✅ MODULE_SPEC compliance verified!");
}

#[test]
fn test_feature_flags_default() {
    println!("=== Default Feature Flags Test ===");
    println!("Testing that default build works without optional features...");

    // This should work without any features enabled
    use rololib::prelude::*;

    let config = LayoutConfig::default();
    println!("Default layout config: width={}, gap={}, padding={}",
             config.width, config.gap, config.padding);

    let term_width = get_terminal_width();
    println!("Terminal width: {}", term_width);

    // Test RSB integration
    let _stream = rsb::prelude::Stream::from_string("test");
    println!("✅ RSB integration works");

    println!("✅ Default feature flags test passed!");
}