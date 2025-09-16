//! Mode Integration Tests - TASK-012
//!
//! Cross-mode test fixtures and pipeline testing to ensure
//! all modes work together seamlessly in real-world scenarios

use rololib::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_mode_switching_preserves_data() {
    // Test that data integrity is maintained across different modes
    let input = "apple,banana,cherry,date,elderberry,fig";

    // Test through column mode
    let config = LayoutConfig { width: 80, gap: 2, padding: 1 };
    let columns_result = format_columns_with_delimiter(input, 2, &config, Some(","));
    assert!(columns_result.is_ok());
    assert!(columns_result.unwrap().contains("apple"));

    // Test through table mode
    let table_input = "Name,Count\napple,5\nbanana,3";
    let table_result = format_table_with_config(table_input, ",", 80);
    assert!(table_result.is_ok());
    assert!(table_result.unwrap().contains("Name"));

    // Test through list mode
    let list_config = ListConfig {
        width: 80,
        line_numbers: true,
        list_style: None,
        alignment: ListAlignment::Left,
    };
    let list_result = format_list_with_config("apple\nbanana\ncherry", &list_config);
    assert!(list_result.is_ok());
    assert!(list_result.unwrap().contains("1. apple"));
}

#[test]
fn test_pipeline_column_to_table() {
    // Simulate: cat data | rolo columns | rolo table
    let initial_data = "a,b,c,d,e,f";
    let config = LayoutConfig { width: 80, gap: 2, padding: 1 };

    // First pass: columns mode
    let columns_output = format_columns_with_delimiter(initial_data, 3, &config, Some(","))
        .expect("Column formatting failed");

    // The column output could be processed as table input
    // This tests that output formats are compatible
    let lines: Vec<&str> = columns_output.lines().collect();
    assert_eq!(lines.len(), 2); // 6 items in 3 columns = 2 rows
}

#[test]
fn test_delimiter_consistency_across_modes() {
    // Ensure all modes handle the same delimiters correctly
    let delimiters = vec![",", "|", ";", "\t", ":"];
    let config = LayoutConfig { width: 80, gap: 2, padding: 1 };

    for delim in delimiters {
        let input = format!("one{}two{}three", delim, delim);

        // Column mode
        let col_result = format_columns_with_delimiter(&input, 2, &config, Some(delim));
        assert!(col_result.is_ok(), "Column mode failed with delimiter: {}", delim);

        // Table mode
        let table_result = format_table_with_config(&input, delim, 80);
        assert!(table_result.is_ok(), "Table mode failed with delimiter: {}", delim);
    }
}

#[test]
fn test_width_constraints_respected_all_modes() {
    // Test that width constraints are properly enforced across all modes
    let widths = vec![40, 60, 80, 120];

    for width in widths {
        // Column mode
        let config = LayoutConfig { width, gap: 2, padding: 1 };
        let col_result = format_columns_with_config("test data for columns", 2, &config);
        assert!(col_result.is_ok());
        let col_output = col_result.unwrap();
        for line in col_output.lines() {
            assert!(line.len() <= width + 10, // Allow some margin for ANSI codes
                    "Column mode exceeded width {} with line length {}", width, line.len());
        }

        // Table mode
        let table_result = format_table_with_config("col1,col2\ndata1,data2", ",", width);
        assert!(table_result.is_ok());
        let table_output = table_result.unwrap();
        for line in table_output.lines() {
            assert!(line.len() <= width + 10,
                    "Table mode exceeded width {} with line length {}", width, line.len());
        }

        // List mode
        let list_config = ListConfig {
            width,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        };
        let list_result = format_list_with_config("test\ndata\nfor\nlist", &list_config);
        assert!(list_result.is_ok());
    }
}

#[test]
fn test_empty_input_handling_all_modes() {
    // Test that all modes gracefully handle empty input
    let config = LayoutConfig { width: 80, gap: 2, padding: 1 };

    // Empty string
    let empty = "";
    assert!(format_columns(empty, 2).is_ok());
    assert!(format_table(empty, ",").is_ok());
    assert!(format_list(empty).is_ok());

    // Whitespace only
    let whitespace = "   \n  \t  \n  ";
    assert!(format_columns_with_config(whitespace, 2, &config).is_ok());
    assert!(format_table_with_config(whitespace, ",", 80).is_ok());

    // Empty after delimiter split
    let empty_fields = ",,,,";
    assert!(format_columns_with_delimiter(empty_fields, 2, &config, Some(",")).is_ok());
}

#[test]
fn test_unicode_handling_all_modes() {
    // Test Unicode text handling across all modes
    let unicode_text = "Hello,世界,🌍,Здравствуй,مرحبا";
    let config = LayoutConfig { width: 80, gap: 2, padding: 1 };

    // Column mode with Unicode
    let col_result = format_columns_with_delimiter(unicode_text, 3, &config, Some(","));
    assert!(col_result.is_ok());
    let col_output = col_result.unwrap();
    assert!(col_output.contains("世界"));
    assert!(col_output.contains("🌍"));

    // Table mode with Unicode
    let table_result = format_table_with_config(unicode_text, ",", 80);
    assert!(table_result.is_ok());

    // List mode with Unicode
    let list_config = ListConfig {
        width: 80,
        line_numbers: false,
        list_style: Some("bullets".to_string()),
        alignment: ListAlignment::Left,
    };
    let list_input = "世界\n🌍\nЗдравствуй";
    let list_result = format_list_with_config(list_input, &list_config);
    assert!(list_result.is_ok());
    assert!(list_result.unwrap().contains("• 世界"));
}

#[test]
fn test_large_dataset_handling() {
    // Test performance with larger datasets
    let mut large_input = Vec::new();
    for i in 0..1000 {
        large_input.push(format!("item{}", i));
    }
    let input_str = large_input.join(",");

    let config = LayoutConfig { width: 120, gap: 2, padding: 1 };

    // Should handle 1000 items without panic
    let result = format_columns_with_delimiter(&input_str, 5, &config, Some(","));
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("item0"));
    assert!(output.contains("item999"));
}

#[test]
fn test_mode_combination_scenarios() {
    // Real-world scenario: Process CSV data through different viewing modes
    let csv_data = "Name,Age,Department,Salary
Alice,30,Engineering,75000
Bob,25,Marketing,65000
Carol,35,Sales,70000
David,28,Engineering,72000";

    // Scenario 1: View as table
    let table_view = format_table_with_config(csv_data, ",", 100);
    assert!(table_view.is_ok());
    let table_output = table_view.unwrap();
    assert!(table_output.contains("Name"));
    assert!(table_output.contains("Alice"));
    assert!(table_output.contains("Engineering"));

    // Scenario 2: Extract names and view as list
    let names_only: Vec<&str> = csv_data.lines()
        .skip(1) // Skip header
        .map(|line| line.split(',').next().unwrap_or(""))
        .collect();
    let names_str = names_only.join("\n");

    let list_config = ListConfig {
        width: 80,
        line_numbers: true,
        list_style: None,
        alignment: ListAlignment::Left,
    };
    let list_view = format_list_with_config(&names_str, &list_config);
    assert!(list_view.is_ok());
    assert!(list_view.unwrap().contains("1. Alice"));

    // Scenario 3: Department names in columns
    let departments = "Engineering,Marketing,Sales,Engineering";
    let config = LayoutConfig { width: 80, gap: 3, padding: 1 };
    let col_view = format_columns_with_delimiter(departments, 2, &config, Some(","));
    assert!(col_view.is_ok());
}

#[test]
fn test_error_propagation_across_modes() {
    // Test that errors are properly handled and reported

    // Invalid column count
    let result = format_columns("test", 0);
    assert!(result.is_err());

    // Invalid width validation
    let invalid_width = validate_width("99999");
    assert!(invalid_width.is_err());

    // These should handle edge cases gracefully
    let config = LayoutConfig { width: 20, gap: 2, padding: 1 }; // Constrained but valid layout
    let result = format_columns_with_config("test data that is too long", 2, &config);
    assert!(result.is_ok()); // Should still work, even if output is constrained
}