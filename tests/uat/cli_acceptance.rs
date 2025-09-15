//! CLI UAT tests - Executive acceptance validation

#[test]
fn uat_cli_help_system_usability() {
    println!("=== UAT: CLI Help System Usability ===");
    println!("Validating help system meets executive usability standards...");

    use rolo::prelude::*;

    // Executive requirement: Help should be accessible and informative
    let args = vec!["rolo".to_string(), "--help".to_string()];
    let config = parse_args(&args).unwrap();
    assert!(config.help, "Help flag should be recognized");
    println!("✅ Help system accessible via --help");

    // Executive requirement: Short help flag should work
    let args = vec!["rolo".to_string(), "-h".to_string()];
    let config = parse_args(&args).unwrap();
    assert!(config.help, "Short help flag should be recognized");
    println!("✅ Help system accessible via -h");

    // Executive requirement: Version information should be accessible
    let args = vec!["rolo".to_string(), "--version".to_string()];
    let config = parse_args(&args).unwrap();
    assert!(config.version, "Version flag should be recognized");
    println!("✅ Version information accessible");

    println!("✅ UAT PASSED: CLI help system meets executive usability standards");
}

#[test]
fn uat_cli_common_workflows() {
    println!("=== UAT: CLI Common Workflows ===");
    println!("Validating common user workflows work intuitively...");

    use rolo::prelude::*;

    // Executive workflow: Simple column formatting
    let args = vec!["rolo".to_string(), "--cols".to_string(), "3".to_string()];
    let config = parse_args(&args).unwrap();
    assert_eq!(config.columns, Some(3));
    match config.mode {
        CliMode::Columns => println!("✅ Column workflow: --cols 3 works"),
        _ => panic!("Expected columns mode"),
    }

    // Executive workflow: Table formatting
    let args = vec!["rolo".to_string(), "table".to_string()];
    let config = parse_args(&args).unwrap();
    match config.mode {
        CliMode::Table => println!("✅ Table workflow: table subcommand works"),
        _ => panic!("Expected table mode"),
    }

    // Executive workflow: Custom width specification
    let args = vec![
        "rolo".to_string(),
        "--cols".to_string(),
        "2".to_string(),
        "--width".to_string(),
        "120".to_string()
    ];
    let config = parse_args(&args).unwrap();
    assert_eq!(config.columns, Some(2));
    assert_eq!(config.width, Some(120));
    println!("✅ Combined workflow: --cols 2 --width 120 works");

    println!("✅ UAT PASSED: Common workflows are intuitive and functional");
}

#[test]
fn uat_cli_error_prevention() {
    println!("=== UAT: CLI Error Prevention ===");
    println!("Validating system prevents common user errors gracefully...");

    use rolo::prelude::*;

    // Executive requirement: Invalid column counts should be rejected
    let invalid_columns = ["0", "15", "abc", ""];
    for invalid_col in &invalid_columns {
        let args = vec!["rolo".to_string(), "--cols".to_string(), invalid_col.to_string()];
        let result = parse_args(&args);
        assert!(result.is_err(), "Invalid column count '{}' should be rejected", invalid_col);
        println!("✅ Invalid column count '{}' correctly rejected", invalid_col);
    }

    // Executive requirement: Invalid widths should be rejected
    let invalid_widths = ["5", "300", "not_a_number"];
    for invalid_width in &invalid_widths {
        let args = vec!["rolo".to_string(), "--width".to_string(), invalid_width.to_string()];
        let result = parse_args(&args);
        assert!(result.is_err(), "Invalid width '{}' should be rejected", invalid_width);
        println!("✅ Invalid width '{}' correctly rejected", invalid_width);
    }

    // Executive requirement: Missing required values should be handled
    let incomplete_args = [
        vec!["rolo".to_string(), "--cols".to_string()],
        vec!["rolo".to_string(), "--width".to_string()],
    ];
    for args in &incomplete_args {
        let result = parse_args(args);
        assert!(result.is_err(), "Incomplete arguments should be rejected");
        println!("✅ Missing argument value correctly handled");
    }

    println!("✅ UAT PASSED: System prevents user errors with clear feedback");
}

#[test]
fn uat_cli_business_compliance() {
    println!("=== UAT: CLI Business Rule Compliance ===");
    println!("Validating CLI enforces business rules and constraints...");

    use rolo::prelude::*;

    // Executive requirement: Column limits should enforce business rules
    let business_column_limits = ["1", "5", "10"]; // Valid business range
    for valid_cols in &business_column_limits {
        let args = vec!["rolo".to_string(), "--cols".to_string(), valid_cols.to_string()];
        let config = parse_args(&args).unwrap();
        let cols = config.columns.unwrap();
        assert!(cols >= 1 && cols <= 10, "Column count should be in business range");
        println!("✅ Business-valid column count {} accepted", cols);
    }

    // Executive requirement: Width constraints should match terminal standards
    let business_widths = ["80", "120", "132", "160"]; // Standard terminal widths
    for valid_width in &business_widths {
        let args = vec!["rolo".to_string(), "--width".to_string(), valid_width.to_string()];
        let config = parse_args(&args).unwrap();
        let width = config.width.unwrap();
        assert!(width >= 10 && width <= 200, "Width should be in business range");
        println!("✅ Business-valid width {} accepted", width);
    }

    // Executive requirement: Default behavior should be sensible
    let args = vec!["rolo".to_string()];
    let config = parse_args(&args).unwrap();
    match config.mode {
        CliMode::Columns => println!("✅ Default mode is columns (business appropriate)"),
        _ => panic!("Default mode should be columns for business use"),
    }
    assert_eq!(config.columns, None, "Default should allow auto-detection");
    assert_eq!(config.width, None, "Default should use terminal width");

    println!("✅ UAT PASSED: CLI enforces business rules and provides sensible defaults");
}