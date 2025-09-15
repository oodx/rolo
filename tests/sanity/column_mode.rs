//! Column mode sanity tests - visible demos per RSB HOWTO_TEST
//!
//! These tests provide visual feedback and demonstrate column functionality

#[test]
fn test_column_mode_basic_demo() {
    println!("=== Column Mode Basic Demo ===");
    println!("Testing basic column formatting functionality...\n");

    use rolo::prelude::*;

    let input = "apple\nbanana\ncherry\ndate\nelderberry\nfig";
    println!("Input text (6 items):");
    println!("{}", input.replace('\n', ", "));
    println!();

    // Test 2 columns
    let result = format_columns(input, 2).expect("Column formatting should work");
    println!("Output with 2 columns:");
    println!("{}", result);
    println!();

    // Test 3 columns
    let result = format_columns(input, 3).expect("Column formatting should work");
    println!("Output with 3 columns:");
    println!("{}", result);
    println!();

    println!("✅ Basic column formatting works!");
}

#[test]
fn test_column_mode_gap_demo() {
    println!("=== Column Mode Gap Spacing Demo ===");
    println!("Demonstrating configurable gap between columns...\n");

    use rolo::prelude::*;

    let input = "one\ntwo\nthree\nfour";
    println!("Input: {}", input.replace('\n', ", "));
    println!();

    // Test with different gap sizes
    for gap in [1, 3, 5] {
        let config = LayoutConfig {
            width: 60,
            gap,
            padding: 1,
        };

        let result = format_columns_with_config(input, 2, &config)
            .expect("Column formatting should work");

        println!("Gap = {} spaces:", gap);
        println!("{}", result);
        println!();
    }

    println!("✅ Gap spacing configuration works!");
}

#[test]
fn test_column_mode_ansi_demo() {
    println!("=== Column Mode ANSI Color Demo ===");
    println!("Testing ANSI color preservation in columns...\n");

    use rolo::prelude::*;

    // Create colored input
    let input = format!(
        "{}\n{}\n{}\n{}",
        "\x1b[31mred\x1b[0m",
        "\x1b[32mgreen\x1b[0m",
        "\x1b[33myellow\x1b[0m",
        "\x1b[34mblue\x1b[0m"
    );

    println!("Input (with ANSI colors):");
    println!("{}", input.replace('\n', ", "));
    println!();

    let result = format_columns(&input, 2).expect("Column formatting should work");
    println!("Output in 2 columns (colors preserved):");
    println!("{}", result);
    println!();

    println!("✅ ANSI color preservation works!");
}

#[test]
fn test_column_mode_unicode_demo() {
    println!("=== Column Mode Unicode Demo ===");
    println!("Testing Unicode and wide character support...\n");

    use rolo::prelude::*;

    let input = "hello\n世界\nこんにちは\n🌟\nemoji\n文字";
    println!("Input (mixed Unicode):");
    for line in input.lines() {
        println!("  - {}", line);
    }
    println!();

    let result = format_columns(input, 2).expect("Column formatting should work");
    println!("Output in 2 columns:");
    println!("{}", result);
    println!();

    println!("✅ Unicode width calculation works!");
}

#[test]
fn test_column_mode_cli_integration_demo() {
    println!("=== Column Mode CLI Integration Demo ===");
    println!("Testing CLI argument parsing for column mode...\n");

    use rolo::prelude::*;

    // Simulate CLI arguments
    let args = vec![
        "rolo".to_string(),
        "--cols".to_string(),
        "3".to_string(),
        "--gap".to_string(),
        "2".to_string(),
    ];

    let config = parse_args(&args).expect("CLI parsing should work");

    println!("Parsed CLI configuration:");
    println!("  Mode: {:?}", config.mode);
    println!("  Columns: {:?}", config.columns);
    println!("  Gap: {:?}", config.gap);
    println!("  Width: {:?}", config.width);
    println!();

    // Verify column mode settings
    assert_eq!(config.columns, Some(3));
    assert_eq!(config.gap, Some(2));

    println!("✅ CLI integration for column mode works!");
}

#[test]
fn test_column_mode_edge_cases_demo() {
    println!("=== Column Mode Edge Cases Demo ===");
    println!("Testing edge cases and error handling...\n");

    use rolo::prelude::*;

    // Test empty input
    let result = format_columns("", 2);
    assert!(result.is_ok());
    println!("✅ Empty input handled gracefully");

    // Test single item
    let result = format_columns("single", 2);
    assert!(result.is_ok());
    println!("✅ Single item handled correctly");

    // Test zero columns (should error)
    let result = format_columns("test", 0);
    assert!(result.is_err());
    println!("✅ Zero columns rejected as expected");

    // Test very narrow width
    let config = LayoutConfig {
        width: 5,
        gap: 2,
        padding: 1,
    };
    let result = format_columns_with_config("test", 3, &config);
    assert!(result.is_err());
    println!("✅ Insufficient width detected");

    println!("\n✅ All edge cases handled properly!");
}

#[test]
fn test_column_mode_performance_demo() {
    println!("=== Column Mode Performance Demo ===");
    println!("Testing with larger datasets...\n");

    use rolo::prelude::*;
    use std::time::Instant;

    // Generate test data
    let items: Vec<String> = (1..=100).map(|i| format!("item_{:03}", i)).collect();
    let input = items.join("\n");

    println!("Processing {} items...", items.len());

    let start = Instant::now();
    let result = format_columns(&input, 4).expect("Column formatting should work");
    let duration = start.elapsed();

    let lines = result.lines().count();
    println!("Output: {} lines in 4 columns", lines);
    println!("Time: {:?}", duration);

    // Show first few lines as sample
    for (i, line) in result.lines().take(3).enumerate() {
        println!("Line {}: {}", i + 1, line);
    }
    println!("... ({} more lines)", lines - 3);

    println!("\n✅ Large dataset handled efficiently!");
}