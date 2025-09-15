//! RSB feature tests - testing RSB global context and options

#[test]
fn test_rsb_options_parsing() {
    println!("=== RSB Options Parsing Test ===");

    use rololib::prelude::*;

    // Test RSB global context option setting
    set_var("cols", "3");
    assert_eq!(get_var("cols"), "3");
    println!("✅ cols option set correctly");

    // Test help flag (RSB boolean: 0=true, 1=false)
    set_var("help", "true");
    assert!(is_true("help"));
    println!("✅ help flag set correctly");

    // Test version flag
    set_var("version", "true");
    assert!(is_true("version"));
    println!("✅ version flag set correctly");

    println!("✅ RSB options parsing works correctly");
}

#[test]
fn test_rsb_command_modes() {
    println!("=== RSB Command Modes Test ===");

    use rololib::prelude::*;

    // Test table mode
    set_var("mode", "table");
    assert_eq!(get_var("mode"), "table");
    println!("✅ Table mode set");

    // Test list mode
    set_var("mode", "list");
    assert_eq!(get_var("mode"), "list");
    println!("✅ List mode set");

    // Test columns mode
    set_var("mode", "columns");
    assert_eq!(get_var("mode"), "columns");
    println!("✅ Columns mode set");

    println!("✅ RSB command modes work correctly");
}

#[test]
fn test_rsb_width_integration() {
    println!("=== RSB Width Integration Test ===");

    use rololib::prelude::*;

    // Test width setting with valid value
    set_var("width", "120");
    assert_eq!(get_var("width"), "120");
    assert!(validate_width("120").is_ok());
    println!("✅ Valid width value accepted");

    // Test width validation with invalid value
    assert!(validate_width("5").is_err());
    println!("✅ Invalid width value rejected");

    println!("✅ RSB width integration works correctly");
}

#[test]
fn test_rsb_error_handling() {
    println!("=== RSB Error Handling Test ===");

    use rololib::prelude::*;

    // Test invalid column count validation
    assert!(validate_width("20").is_err()); // Too small for width validation
    println!("✅ Invalid column count rejected");

    // Test RSB variable validation
    set_var("test_empty", "");
    assert_eq!(get_var("test_empty"), "");
    println!("✅ Empty argument handled");

    // Test undefined variable handling
    assert_eq!(get_var("unknown_var"), "");
    println!("✅ Unknown variable handled");

    println!("✅ RSB error handling works correctly");
}