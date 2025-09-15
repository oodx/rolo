//! CLI feature tests - testing argument parsing and execution

#[test]
fn test_cli_argument_parsing() {
    println!("=== CLI Argument Parsing Test ===");

    use rolo::prelude::*;

    // Test basic argument parsing
    let args = vec!["rolo".to_string(), "--cols".to_string(), "3".to_string()];
    let config = parse_args(&args).unwrap();

    match config.mode {
        CliMode::Columns => println!("✅ Default mode is columns"),
        _ => panic!("Expected columns mode"),
    }
    assert_eq!(config.columns, Some(3));
    println!("✅ --cols argument parsed correctly");

    // Test help flag
    let args = vec!["rolo".to_string(), "--help".to_string()];
    let config = parse_args(&args).unwrap();
    assert!(config.help);
    println!("✅ --help flag parsed correctly");

    // Test version flag
    let args = vec!["rolo".to_string(), "--version".to_string()];
    let config = parse_args(&args).unwrap();
    assert!(config.version);
    println!("✅ --version flag parsed correctly");

    println!("✅ CLI argument parsing works correctly");
}

#[test]
fn test_cli_subcommands() {
    println!("=== CLI Subcommands Test ===");

    use rolo::prelude::*;

    // Test table subcommand
    let args = vec!["rolo".to_string(), "table".to_string()];
    let config = parse_args(&args).unwrap();
    match config.mode {
        CliMode::Table => println!("✅ Table subcommand parsed"),
        _ => panic!("Expected table mode"),
    }

    // Test list subcommand
    let args = vec!["rolo".to_string(), "list".to_string()];
    let config = parse_args(&args).unwrap();
    match config.mode {
        CliMode::List => println!("✅ List subcommand parsed"),
        _ => panic!("Expected list mode"),
    }

    // Test columns subcommand
    let args = vec!["rolo".to_string(), "columns".to_string()];
    let config = parse_args(&args).unwrap();
    match config.mode {
        CliMode::Columns => println!("✅ Columns subcommand parsed"),
        _ => panic!("Expected columns mode"),
    }

    println!("✅ CLI subcommands work correctly");
}

#[test]
fn test_cli_width_integration() {
    println!("=== CLI Width Integration Test ===");

    use rolo::prelude::*;

    // Test width argument with valid value
    let args = vec!["rolo".to_string(), "--width".to_string(), "120".to_string()];
    let config = parse_args(&args).unwrap();
    assert_eq!(config.width, Some(120));
    println!("✅ Valid width argument accepted");

    // Test width argument with invalid value
    let args = vec!["rolo".to_string(), "--width".to_string(), "5".to_string()];
    let result = parse_args(&args);
    assert!(result.is_err());
    println!("✅ Invalid width argument rejected");

    println!("✅ CLI width integration works correctly");
}

#[test]
fn test_cli_error_handling() {
    println!("=== CLI Error Handling Test ===");

    use rolo::prelude::*;

    // Test invalid column count
    let args = vec!["rolo".to_string(), "--cols".to_string(), "20".to_string()];
    let result = parse_args(&args);
    assert!(result.is_err());
    println!("✅ Invalid column count rejected");

    // Test missing argument value
    let args = vec!["rolo".to_string(), "--cols".to_string()];
    let result = parse_args(&args);
    assert!(result.is_err());
    println!("✅ Missing argument value handled");

    // Test unknown option
    let args = vec!["rolo".to_string(), "--unknown".to_string()];
    let result = parse_args(&args);
    assert!(result.is_err());
    println!("✅ Unknown option rejected");

    println!("✅ CLI error handling works correctly");
}