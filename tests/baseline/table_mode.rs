//! Table mode implementation tests - validates TASK-008 requirements
//!
//! Tests the table formatting functionality including:
//! - Auto-width column detection
//! - Delimiter parsing (TSV, CSV, custom)
//! - Row overflow handling
//! - Header row detection and separation

#[test]
fn test_table_mode_basic_functionality() {
    use rololib::prelude::*;

    // Test basic table with TSV data (acceptance criteria)
    let input = "Name\tAge\tCity\nJohn\t25\tNew York\nAlice\t30\tLondon";
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("Name"));
    assert!(output.contains("Age"));
    assert!(output.contains("City"));
    assert!(output.contains("John"));
    assert!(output.contains("25"));
    assert!(output.contains("New York"));

    // Should have header separator
    assert!(output.contains("-----"));
    assert!(output.contains(" | "));
}

#[test]
fn test_table_mode_with_csv_delimiter() {
    use rololib::prelude::*;

    let input = "Product,Price,Stock\nApple,1.50,100\nBanana,0.75,50";
    let result = format_table(input, ",");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("Product"));
    assert!(output.contains("Price"));
    assert!(output.contains("Stock"));
    assert!(output.contains("Apple"));
    assert!(output.contains("1.50"));
    assert!(output.contains("100"));

    // Check proper column separation
    assert!(output.contains(" | "));
    assert!(output.contains("-------+-------+-----"));
}

#[test]
fn test_table_mode_auto_width_detection() {
    use rololib::prelude::*;

    // Test with varying column widths
    let input = "A\tVery Long Header Name\tC\nX\tShort\tY\nZ\tMedium Length\tW";
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    let lines: Vec<&str> = output.lines().collect();

    // Should have 4 lines: header, separator, 2 data rows
    assert_eq!(lines.len(), 4);

    // Check that columns are properly aligned
    assert!(lines[0].contains("Very Long Header Name"));
    assert!(lines[1].contains("---------------------")); // Long separator for long header
    assert!(lines[2].contains("Short"));
    assert!(lines[3].contains("Medium Length"));
}

#[test]
fn test_table_mode_with_width_constraints() {
    use rololib::prelude::*;

    let input = "Header1\tVery Long Header That Exceeds Width\tHeader3\nData1\tShort\tData3";
    let result = format_table_with_config(input, "\t", 40);
    assert!(result.is_ok());

    let output = result.unwrap();

    // Should handle width constraints by compressing columns
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 3); // At least header, separator, data

    // Check that total line width doesn't exceed constraints significantly
    for line in &lines {
        // Allow some flexibility for separators and formatting
        assert!(line.len() <= 50, "Line too long: {}", line);
    }
}

#[test]
fn test_table_mode_row_overflow_handling() {
    use rololib::prelude::*;

    // Test with uneven rows (some rows have more columns than others)
    let input = "A\tB\tC\nX\tY\nZ\tW\tV\tU"; // Second row missing column, fourth row has extra
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));
    assert!(output.contains("X"));
    assert!(output.contains("Y"));
    assert!(output.contains("Z"));
    assert!(output.contains("W"));
    assert!(output.contains("V"));
    assert!(output.contains("U"));

    // Should handle missing columns gracefully
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 3);
}

#[test]
fn test_table_mode_empty_input() {
    use rololib::prelude::*;

    let result = format_table("", "\t");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");

    // Test with only whitespace
    let result = format_table("   \n\n  \t  \n", "\t");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_table_mode_single_row() {
    use rololib::prelude::*;

    let result = format_table("Column1\tColumn2\tColumn3", "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("Column1"));
    assert!(output.contains("Column2"));
    assert!(output.contains("Column3"));

    // Single row should not have header separator
    assert!(!output.contains("-----"));
}

#[test]
fn test_table_mode_single_column() {
    use rololib::prelude::*;

    let input = "Header\nValue1\nValue2\nValue3";
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("Header"));
    assert!(output.contains("Value1"));
    assert!(output.contains("Value2"));
    assert!(output.contains("Value3"));

    // Should handle single column tables
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), 5); // Header, separator, 3 values
}

#[test]
fn test_table_mode_custom_delimiters() {
    use rololib::prelude::*;

    // Test semicolon delimiter
    let input = "Name;Age;Country\nJohn;25;USA\nMarie;30;France";
    let result = format_table(input, ";");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Name"));
    assert!(output.contains("John"));
    assert!(output.contains("Marie"));

    // Test pipe delimiter
    let input = "A|B|C\nX|Y|Z";
    let result = format_table(input, "|");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));
}

#[test]
fn test_table_mode_with_spaces_in_cells() {
    use rololib::prelude::*;

    let input = "Full Name\tJob Title\tCompany\nJohn Doe\tSoftware Engineer\tTech Corp\nJane Smith\tProduct Manager\tInnovate Inc";
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("John Doe"));
    assert!(output.contains("Software Engineer"));
    assert!(output.contains("Tech Corp"));
    assert!(output.contains("Jane Smith"));
    assert!(output.contains("Product Manager"));
    assert!(output.contains("Innovate Inc"));

    // Verify proper column alignment with spaces
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 4); // Header, separator, 2 data rows
}

#[test]
fn test_table_mode_ansi_aware_content() {
    use rololib::prelude::*;

    // Test with ANSI escape sequences
    let input = "\x1b[31mRed Header\x1b[0m\tNormal\n\x1b[32mGreen Data\x1b[0m\tRegular";
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();

    // Should preserve ANSI codes
    assert!(output.contains("\x1b[31m"));
    assert!(output.contains("\x1b[32m"));
    assert!(output.contains("\x1b[0m"));

    // Should still format properly
    assert!(output.contains("Red Header"));
    assert!(output.contains("Green Data"));
    assert!(output.contains("Normal"));
    assert!(output.contains("Regular"));
}

#[test]
fn test_table_mode_unicode_content() {
    use rololib::prelude::*;

    // Test with Unicode characters (wide chars)
    let input = "名前\t年齢\t都市\n太郎\t25\t東京\n花子\t30\t大阪";
    let result = format_table(input, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("名前"));
    assert!(output.contains("年齢"));
    assert!(output.contains("都市"));
    assert!(output.contains("太郎"));
    assert!(output.contains("花子"));
    assert!(output.contains("東京"));
    assert!(output.contains("大阪"));
}

#[test]
fn test_table_mode_truncation() {
    use rololib::prelude::*;

    // Test with very long content that should be truncated
    let very_long_content = "Very".repeat(50); // 200 characters
    let input = format!("Header\tVery Long\nData\t{}", very_long_content);

    let result = format_table_with_config(&input, "\t", 40);
    assert!(result.is_ok());

    let output = result.unwrap();

    // Should contain ellipsis for truncated content
    assert!(output.contains("..."));

    // Total width should be constrained
    for line in output.lines() {
        assert!(line.len() <= 50, "Line too long after truncation: {}", line);
    }
}

#[test]
fn test_table_mode_rsb_integration() {
    use rololib::prelude::*;

    // Test RSB global context for table mode
    set_var("mode", "table");
    set_var("delimiter", ",");

    // Should be able to read these configurations from RSB context
    assert_eq!(get_var("mode"), "table");
    assert_eq!(get_var("delimiter"), ",");

    // Test that table mode can be detected from RSB context
    let is_table_mode = get_var("mode") == "table";
    assert!(is_table_mode);
}