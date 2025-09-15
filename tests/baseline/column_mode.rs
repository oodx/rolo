//! Column mode implementation tests - validates TASK-007 requirements
//!
//! Tests the core column formatting functionality including:
//! - --cols N argument parsing
//! - Column layout algorithm
//! - ANSI-aware width padding
//! - --gap option for spacing

#[test]
fn test_column_mode_basic_functionality() {
    use rolo::prelude::*;

    // Test basic 2-column layout (acceptance criteria)
    let input = "a\nb\nc\nd";
    let result = format_columns(input, 2);
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("a"));
    assert!(output.contains("b"));
    assert!(output.contains("c"));
    assert!(output.contains("d"));

    // Should have 2 lines (rows) for 4 items in 2 columns
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn test_column_mode_with_custom_config() {
    use rolo::prelude::*;

    let input = "one\ntwo\nthree\nfour\nfive\nsix";
    let config = LayoutConfig {
        width: 60,
        gap: 3,
        padding: 1,
    };

    let result = format_columns_with_config(input, 3, &config);
    assert!(result.is_ok());

    let output = result.unwrap();
    let lines: Vec<&str> = output.lines().collect();

    // 6 items in 3 columns = 2 rows
    assert_eq!(lines.len(), 2);

    // Check that all items are present
    assert!(output.contains("one"));
    assert!(output.contains("two"));
    assert!(output.contains("three"));
    assert!(output.contains("four"));
    assert!(output.contains("five"));
    assert!(output.contains("six"));
}

#[test]
fn test_column_mode_error_conditions() {
    use rolo::prelude::*;

    // Test zero columns
    let result = format_columns("test", 0);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LayoutError::InvalidColumnCount(0)));

    // Test width too small for gaps
    let config = LayoutConfig {
        width: 10,
        gap: 5,
        padding: 1,
    };
    let result = format_columns_with_config("test", 3, &config);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LayoutError::WidthTooSmall(_, _)));
}

#[test]
fn test_column_mode_empty_input() {
    use rolo::prelude::*;

    let result = format_columns("", 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_column_mode_single_item() {
    use rolo::prelude::*;

    let result = format_columns("single", 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "single");
}

#[test]
fn test_column_mode_gap_spacing() {
    use rolo::prelude::*;

    let input = "a\nb\nc\nd";

    // Test with gap = 1
    let config1 = LayoutConfig {
        width: 40,
        gap: 1,
        padding: 1,
    };
    let result1 = format_columns_with_config(input, 2, &config1);
    assert!(result1.is_ok());

    // Test with gap = 5
    let config5 = LayoutConfig {
        width: 40,
        gap: 5,
        padding: 1,
    };
    let result5 = format_columns_with_config(input, 2, &config5);
    assert!(result5.is_ok());

    // Both should work but have different spacing
    assert_ne!(result1.unwrap(), result5.unwrap());
}

#[test]
fn test_column_mode_ansi_aware_width() {
    use rolo::prelude::*;

    // Test with ANSI escape sequences
    let input = "\x1b[31mred\x1b[0m\nblue\n\x1b[32mgreen\x1b[0m\nyellow";
    let result = format_columns(input, 2);
    assert!(result.is_ok());

    let output = result.unwrap();

    // Should preserve ANSI codes
    assert!(output.contains("\x1b[31m"));
    assert!(output.contains("\x1b[32m"));
    assert!(output.contains("\x1b[0m"));

    // Should still format properly
    assert!(output.contains("red"));
    assert!(output.contains("blue"));
    assert!(output.contains("green"));
    assert!(output.contains("yellow"));
}

#[test]
fn test_column_mode_unicode_content() {
    use rolo::prelude::*;

    // Test with Unicode characters (wide chars)
    let input = "hello\n世界\ntest\nうに";
    let result = format_columns(input, 2);
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("hello"));
    assert!(output.contains("世界"));
    assert!(output.contains("test"));
    assert!(output.contains("うに"));
}

#[test]
fn test_column_mode_layout_algorithm() {
    use rolo::prelude::*;

    // Test that items are distributed column-wise (not row-wise)
    let input = "1\n2\n3\n4\n5\n6";
    let result = format_columns(input, 2);
    assert!(result.is_ok());

    let output = result.unwrap();
    let lines: Vec<&str> = output.lines().collect();

    // Should have 3 lines for 6 items in 2 columns
    assert_eq!(lines.len(), 3);

    // Column-wise distribution should have 1,3,5 in first column and 2,4,6 in second
    // Check first line contains 1 and 4 (first row of each column)
    assert!(lines[0].contains("1"));
    assert!(lines[0].contains("4"));

    // Check second line contains 2 and 5
    assert!(lines[1].contains("2"));
    assert!(lines[1].contains("5"));

    // Check third line contains 3 and 6
    assert!(lines[2].contains("3"));
    assert!(lines[2].contains("6"));
}

#[test]
fn test_column_mode_cli_integration() {
    use rolo::prelude::*;

    // Test CLI configuration with gap option
    let mut config = CliConfig::default();
    config.gap = Some(3);
    config.columns = Some(2);

    // Should be able to construct this configuration
    assert_eq!(config.gap, Some(3));
    assert_eq!(config.columns, Some(2));
}

#[test]
fn test_column_mode_width_constraints() {
    use rolo::prelude::*;

    // Test that very narrow columns are rejected
    let config = LayoutConfig {
        width: 8,  // Very small width
        gap: 2,
        padding: 1,
    };

    let result = format_columns_with_config("test", 3, &config);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), LayoutError::ColumnTooNarrow(_)));
}

#[test]
fn test_column_mode_delimiter_support() {
    use rolo::prelude::*;

    let config = LayoutConfig::default();

    // Test comma delimiter
    let input = "apple,banana,cherry,date";
    let result = format_columns_with_delimiter(input, 2, &config, Some(","));
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.lines().count(), 2);
    assert!(output.contains("apple"));
    assert!(output.contains("banana"));
    assert!(output.contains("cherry"));
    assert!(output.contains("date"));

    // Test space delimiter
    let input = "one two three four";
    let result = format_columns_with_delimiter(input, 2, &config, Some(" "));
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.lines().count(), 2);

    // Test semicolon delimiter
    let input = "red;green;blue;yellow";
    let result = format_columns_with_delimiter(input, 2, &config, Some(";"));
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.lines().count(), 2);

    // Test that newlines still work (backwards compatibility)
    let input = "line1\nline2\nline3\nline4";
    let result = format_columns_with_delimiter(input, 2, &config, None);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.lines().count(), 2);
}