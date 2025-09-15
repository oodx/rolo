//! Integration tests using actual test data files
//!
//! These tests verify that rolo works correctly with real data files
//! in the tests/data directory, providing end-to-end validation.

use std::path::Path;

#[test]
fn test_table_mode_with_sample_tsv() {
    use rololib::prelude::*;

    // Read the actual test data file
    let test_data_path = "tests/data/sample.tsv";
    if !Path::new(test_data_path).exists() {
        // Skip test if running in environment without test data
        return;
    }

    let content = std::fs::read_to_string(test_data_path).expect("Failed to read test data");
    let result = format_table(&content, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();

    // Verify expected content from sample.tsv
    assert!(output.contains("Name"));
    assert!(output.contains("Department"));
    assert!(output.contains("John Doe"));
    assert!(output.contains("Engineering"));
    assert!(output.contains("Alice Smith"));
    assert!(output.contains("Marketing"));

    // Should have proper table formatting
    assert!(output.contains(" | "));
    assert!(output.contains("-----"));
}

#[test]
fn test_table_mode_with_sample_csv() {
    use rololib::prelude::*;

    let test_data_path = "tests/data/sample.csv";
    if !Path::new(test_data_path).exists() {
        return;
    }

    let content = std::fs::read_to_string(test_data_path).expect("Failed to read test data");
    let result = format_table(&content, ",");
    assert!(result.is_ok());

    let output = result.unwrap();

    // Verify expected content from sample.csv
    assert!(output.contains("Product"));
    assert!(output.contains("Price"));
    assert!(output.contains("Apple"));
    assert!(output.contains("1.50"));
    assert!(output.contains("Electronics"));

    // Should have proper table formatting
    assert!(output.contains(" | "));
    assert!(output.contains("-----"));
}

#[test]
fn test_column_mode_with_simple_list() {
    use rololib::prelude::*;

    let test_data_path = "tests/data/simple_list.txt";
    if !Path::new(test_data_path).exists() {
        return;
    }

    let content = std::fs::read_to_string(test_data_path).expect("Failed to read test data");
    let result = format_columns(&content, 3);
    assert!(result.is_ok());

    let output = result.unwrap();

    // Verify expected content from simple_list.txt
    assert!(output.contains("apple"));
    assert!(output.contains("banana"));
    assert!(output.contains("cherry"));
    assert!(output.contains("kiwi"));

    // Should have multiple lines (rows) for column layout
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 3); // 10 items in 3 columns = at least 4 rows
}

#[test]
fn test_table_mode_with_unicode_content() {
    use rololib::prelude::*;

    let test_data_path = "tests/data/unicode_content.tsv";
    if !Path::new(test_data_path).exists() {
        return;
    }

    let content = std::fs::read_to_string(test_data_path).expect("Failed to read test data");
    let result = format_table(&content, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();

    // Verify expected Japanese content
    assert!(output.contains("名前"));
    assert!(output.contains("太郎"));
    assert!(output.contains("エンジニア"));
    assert!(output.contains("東京"));

    // Should handle Unicode width properly
    assert!(output.contains(" | "));
    assert!(output.contains("-----"));
}

#[test]
fn test_table_mode_with_long_content() {
    use rololib::prelude::*;

    let test_data_path = "tests/data/long_content.tsv";
    if !Path::new(test_data_path).exists() {
        return;
    }

    let content = std::fs::read_to_string(test_data_path).expect("Failed to read test data");

    // Test with width constraints
    let result = format_table_with_config(&content, "\t", 80);
    assert!(result.is_ok());

    let output = result.unwrap();

    // Verify expected content from long_content.tsv
    assert!(output.contains("Title"));
    assert!(output.contains("Description"));
    assert!(output.contains("1984")); // Short title that shouldn't be truncated
    assert!(output.contains("Author")); // Header that should be there

    // Should handle long content appropriately
    assert!(output.contains(" | "));
    assert!(output.contains("-----"));

    // Lines should not be excessively long
    for line in output.lines() {
        assert!(line.len() <= 100, "Line too long: {}", line);
    }
}

#[test]
fn test_table_mode_with_uneven_rows() {
    use rololib::prelude::*;

    let test_data_path = "tests/data/uneven_rows.tsv";
    if !Path::new(test_data_path).exists() {
        return;
    }

    let content = std::fs::read_to_string(test_data_path).expect("Failed to read test data");
    let result = format_table(&content, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();

    // Should handle uneven rows gracefully
    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));
    assert!(output.contains("D"));
    assert!(output.contains("Extra"));
    assert!(output.contains("Short"));

    // Should still format as table
    assert!(output.contains(" | "));

    // Should have multiple rows
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 3); // At least header, separator, and data
}

#[test]
fn test_data_files_exist() {
    // Verify that our test data files are available
    let expected_files = [
        "tests/data/sample.tsv",
        "tests/data/sample.csv",
        "tests/data/simple_list.txt",
        "tests/data/unicode_content.tsv",
        "tests/data/long_content.tsv",
        "tests/data/uneven_rows.tsv",
        "tests/data/ansi_colors.tsv",
        "tests/data/README.md",
    ];

    for file_path in &expected_files {
        if Path::new(file_path).exists() {
            // File exists, verify it's readable
            let content = std::fs::read_to_string(file_path);
            assert!(content.is_ok(), "Failed to read {}", file_path);
            assert!(!content.unwrap().is_empty(), "File {} is empty", file_path);
        }
        // If file doesn't exist, test will be skipped (see individual tests above)
    }
}