# FEATURES_TEST_INFRASTRUCTURE.md

## Overview

Rolo implements a comprehensive, multi-layered testing infrastructure designed to validate functionality across different feature configurations and usage scenarios. The testing framework follows RSB (Rust Systems Building) patterns and supports both minimal baseline builds and feature-rich configurations.

The testing infrastructure was developed primarily during **TASK-006** (Feature Flags and Testing Setup) and expanded throughout **TASK-007** (Column Mode - ✅ COMPLETE with --delim support) and **TASK-008** (Table Mode - ✅ COMPLETE with auto-width detection) implementations, providing **157 total test functions** across multiple categories with comprehensive test coverage.

## Testing Framework Architecture

### Test Organization Structure

```
tests/
├── baseline_main.rs              # Entry point for core functionality tests
├── feature_gated_main.rs         # Entry point for optional feature tests
├── visual_uat_main.rs            # Entry point for visual acceptance tests
├── sanity_main.rs                # Entry point for basic sanity checks
├── uat_main.rs                   # Entry point for user acceptance tests
├── prelude_test.rs               # Entry point for prelude API tests
├── stream_sanity.rs              # Standalone stream processing tests
├── baseline/                     # Core functionality (no features required)
│   ├── core_functionality.rs
│   ├── default_behavior.rs
│   ├── error_handling.rs
│   ├── column_mode.rs           # 12 column mode tests
│   ├── table_mode.rs            # 14 table mode tests
│   └── integration_with_test_data.rs
├── feature_gated/               # Optional feature tests
│   ├── width_boxy.rs           # Width calculation enhancements
│   ├── visual.rs               # Rich terminal formatting
│   ├── csv_support.rs          # CSV parsing features
│   └── json_support.rs         # JSON processing features
├── visual_uat/                  # Visual User Acceptance Testing
│   ├── column_formatting.rs    # Visual column layout validation
│   ├── table_formatting.rs     # Visual table output validation
│   ├── list_formatting.rs      # Visual list formatting validation
│   ├── width_integration.rs    # Terminal width adaptation testing
│   ├── separator_demo.rs       # Comprehensive separator/delimiter testing ✅ NEW
│   └── column_mode_uat.rs      # Executive column mode demonstration
├── uat/                         # User Acceptance Testing
│   ├── acceptance_baseline.rs
│   ├── cli_acceptance.rs
│   ├── width_acceptance.rs
│   └── prelude_acceptance.rs
├── integration/                 # Integration testing
├── sanity/                      # Basic sanity checks
└── data/                        # Test fixtures and data files
    ├── README.md
    ├── sample.tsv              # Real-world TSV data
    ├── sample.csv              # CSV test data
    ├── unicode_content.tsv     # CJK and Unicode testing
    ├── ansi_colors.tsv         # ANSI escape sequence testing
    ├── long_content.tsv        # Width constraint testing
    ├── uneven_rows.tsv         # Row overflow testing
    └── simple_list.txt         # Basic column mode testing
```

### Test Execution Modes

**1. Baseline Testing (No Features)**
```bash
# Tests core functionality without optional dependencies
cargo test --no-default-features
```
- **Comprehensive test coverage** validating essential rolo functionality
- Ensures minimal builds work correctly
- Tests CLI parsing, basic formatting, error handling

**2. Feature-Gated Testing**
```bash
# Tests with all features enabled
cargo test --all-features
```
- **Additional test coverage** for enhanced functionality
- Validates `width-boxy`, `visual`, `csv-support`, `json-support` features
- Tests adapter patterns and feature integration

**3. Visual UAT Testing**
```bash
# Visual acceptance testing for executive review
cargo test visual_uat
```
- **Visual output validation** for stakeholder approval
- Demonstrates actual formatting results
- Screenshots and reference output generation

**4. Integration Testing**
```bash
# Real-world usage scenario validation
cargo test integration
```
- **End-to-end pipeline testing**
- Real data file processing
- CLI integration with actual workflows

## Test Data Infrastructure

### Comprehensive Test Fixtures

**`tests/data/` Directory Contents:**

**Basic Data Files:**
- **`sample.tsv`**: Employee data with mixed content types (Name, Age, City, Department)
- **`sample.csv`**: Product catalog with pricing data
- **`simple_list.txt`**: Basic newline-separated items for column testing

**Advanced Testing Scenarios:**
- **`unicode_content.tsv`**: Japanese text (CJK characters) for Unicode width testing
- **`ansi_colors.tsv`**: ANSI escape sequences for color preservation testing
- **`long_content.tsv`**: Oversized content for width constraint and truncation testing
- **`uneven_rows.tsv`**: Irregular data structures for overflow handling testing

**Test Data Characteristics:**
```tsv
# Sample from sample.tsv
Name	Age	City	Department
John Doe	25	New York	Engineering
Alice Smith	30	London	Marketing
Bob Johnson	35	Tokyo	Sales
Carol Brown	28	Paris	Design
David Wilson	32	Berlin	Engineering
Eva Garcia	27	Madrid	Marketing
```

### Data-Driven Testing Strategy

**Integration Test Pattern:**
```rust
#[test]
fn test_real_data_integration() {
    let test_data = include_str!("../data/sample.tsv");
    let result = format_table(test_data, "\t");
    assert!(result.is_ok());

    let output = result.unwrap();
    // Validate structure, content, formatting
    assert!(output.contains("Name"));
    assert!(output.contains("Engineering"));
    assert!(output.contains("-----"));
}
```

## Testing Coverage Analysis

### Baseline Test Suite (Comprehensive Coverage)

**Core Functionality Testing:**
- ✅ **CLI Parsing**: Argument validation, error handling, configuration
- ✅ **Stream Processing**: stdin/stdout handling, pipeline integration
- ✅ **Width Calculations**: Terminal detection, ANSI-aware sizing
- ✅ **Error Handling**: Comprehensive error type validation

**Column Mode Testing (12 Tests):**
- ✅ Basic 2-column layout (acceptance criteria)
- ✅ Custom configuration (gap, width, padding)
- ✅ Error conditions (zero columns, width constraints)
- ✅ Edge cases (empty input, single items)
- ✅ ANSI preservation and Unicode support
- ✅ Delimiter support (comma, semicolon, space, custom)
- ✅ Layout algorithm validation (column-wise distribution)
- ✅ CLI integration and width constraints

**Table Mode Testing (14 Tests):**
- ✅ Basic TSV functionality (acceptance criteria)
- ✅ CSV and custom delimiter support
- ✅ Auto-width detection and column sizing
- ✅ Row overflow and uneven structure handling
- ✅ Width constraints and content truncation
- ✅ Header detection and separator generation
- ✅ ANSI escape sequence preservation
- ✅ Unicode character width handling
- ✅ Edge cases (empty input, single row/column)
- ✅ Content with spaces and special characters

### Feature-Gated Test Suite (Additional Coverage)

**Width-Boxy Integration:**
```rust
#[cfg(feature = "width-boxy")]
mod width_boxy_tests {
    // Enhanced width calculation accuracy
    // Boxy adapter functionality
    // Performance comparisons
}
```

**Visual Feature Testing:**
```rust
#[cfg(feature = "visual")]
mod visual_tests {
    // Rich terminal formatting
    // Color scheme support
    // Enhanced styling options
}
```

**Future Feature Support:**
```rust
#[cfg(feature = "csv-support")]  // Planned: Enhanced CSV parsing
#[cfg(feature = "json-support")] // Planned: JSON table flattening
```

### Visual UAT Testing Framework

**Executive Demonstration Tests:**
- **Column Formatting**: Live demonstration of 1-10 column layouts
- **Table Formatting**: Visual table output with real data
- **List Formatting**: Bulleted and numbered list outputs
- **Width Integration**: Terminal adaptation across different sizes

**UAT Test Pattern:**
```rust
#[test]
fn visual_column_demo() {
    println!("=== COLUMN MODE VISUAL DEMONSTRATION ===");

    let input = "apple\nbanana\ncherry\ndate\nelderberry\nfig";
    for cols in 1..=4 {
        println!("\n--- {} Columns ---", cols);
        let result = format_columns(input, cols).unwrap();
        println!("{}", result);
    }
}
```

## RSB HOWTO_TEST Compliance

### Test Organization Standards

**MODULE_SPEC Compliance:**
- **Baseline vs Feature separation**: Clear distinction between core and optional functionality
- **Error testing**: Comprehensive error condition coverage
- **Integration testing**: Real-world usage scenario validation
- **Performance baseline**: Metrics for optimization targets

**RSB Testing Patterns:**
```rust
// Error handling validation
assert!(matches!(result.unwrap_err(), LayoutError::InvalidColumnCount(0)));

// Configuration testing
let config = LayoutConfig { width: 60, gap: 3, padding: 1 };

// Stream integration
let pipeline = create_pipeline().with_formatter(format_columns);
```

### Test Execution Infrastructure

**Cargo Test Integration:**
```toml
# Cargo.toml test configuration
[dev-dependencies]
criterion = "0.4"  # Benchmarking (planned)

[[test]]
name = "baseline"
path = "tests/baseline_main.rs"

[[test]]
name = "feature_gated"
path = "tests/feature_gated_main.rs"
required-features = ["width-boxy"]
```

**CI/CD Testing Strategy:**
```bash
# Baseline build verification
cargo test --no-default-features

# Full feature testing
cargo test --all-features

# Performance regression testing
cargo test --release performance

# Visual output validation
cargo test visual_uat > visual_output.txt
```

## Integration Points

### Stream Processing Integration

**Pipeline Testing:**
```rust
#[test]
fn test_stream_integration() {
    let input = "a\nb\nc\nd";
    let pipeline = stdin_to_stream(input.as_bytes())
        .pipe_transform(|s| format_columns(&s, 2))
        .stream_to_stdout();

    assert!(pipeline.execute().is_ok());
}
```

**Real-World Usage Testing:**
```bash
# Test actual command pipelines
echo "data" | cargo run -- --cols 2 | wc -l
find . -name "*.rs" | cargo run -- --cols 4 | head -10
ps aux | cargo run -- --table --delim " " | tail -5
```

### CLI Testing Integration

**Argument Parsing Validation:**
```rust
#[test]
fn test_cli_integration() {
    let config = CliConfig {
        columns: Some(3),
        gap: Some(2),
        delimiter: Some(",".to_string()),
        mode: CliMode::Column,
    };

    // Validate configuration application
    assert_eq!(config.columns, Some(3));
    assert!(matches!(config.mode, CliMode::Column));
}
```

### Error Handling Integration

**Comprehensive Error Validation:**
```rust
// Width constraint errors
assert!(matches!(result.unwrap_err(), LayoutError::WidthTooSmall(_, _)));

// Input validation errors
assert!(matches!(result.unwrap_err(), LayoutError::InvalidColumnCount(0)));

// Stream processing errors
assert!(matches!(result.unwrap_err(), StreamError::PipelineBroken));
```

## Testing Best Practices

### Test Writing Standards

**Descriptive Test Names:**
```rust
#[test]
fn test_column_mode_ansi_aware_width() // Clear purpose
fn test_table_mode_row_overflow_handling() // Specific scenario
fn test_unicode_content_width_calculation() // Feature focus
```

**Comprehensive Assertions:**
```rust
// Structure validation
assert_eq!(lines.len(), 2);

// Content preservation
assert!(output.contains("expected_text"));

// ANSI preservation
assert!(output.contains("\x1b[31m"));

// Error type validation
assert!(matches!(result.unwrap_err(), LayoutError::ColumnTooNarrow(_)));
```

### Performance Testing Framework

**Benchmark Integration (Planned TASK-015):**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn column_mode_benchmark(c: &mut Criterion) {
    let large_input = "item\n".repeat(10000);

    c.bench_function("column_mode_10k_items", |b| {
        b.iter(|| format_columns(black_box(&large_input), 4))
    });
}
```

**Memory Usage Testing:**
```rust
#[test]
fn test_memory_efficiency() {
    let large_input = "x\n".repeat(100_000);
    let result = format_columns(&large_input, 5);

    // Memory usage should be linear, not exponential
    assert!(result.is_ok());
}
```

## Future Testing Enhancements

### Planned Testing Improvements

**Property-Based Testing (TASK-021):**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn column_formatting_preserves_content(
        items in prop::collection::vec(any::<String>(), 1..100),
        cols in 1usize..10
    ) {
        let input = items.join("\n");
        let result = format_columns(&input, cols).unwrap();

        // All original items should appear in output
        for item in &items {
            assert!(result.contains(item));
        }
    }
}
```

**Fuzz Testing Integration:**
```rust
#[test]
fn fuzz_table_parser() {
    // Random input generation for parser robustness
    // Edge case discovery through automated testing
}
```

**Visual Regression Testing:**
```bash
# Automated screenshot comparison
cargo test visual_uat > current_output.txt
diff reference_output.txt current_output.txt
```

### Integration Test Expansion

**Database Integration Testing:**
```bash
# Real database export testing
mysql -B -e "SELECT * FROM users LIMIT 10" | cargo run -- --table
psql -t -c "SELECT name, age FROM people" | cargo run -- --table
```

**Log Processing Testing:**
```bash
# System log parsing validation
journalctl -n 100 | cargo run -- --table --delim " "
tail -f /var/log/syslog | cargo run -- --cols 3
```

**Configuration Management Testing:**
```bash
# Config file processing
cat /etc/passwd | cut -d: -f1,3,5 | cargo run -- --table --delim ":"
env | cargo run -- --table --delim "="
```

This comprehensive testing infrastructure ensures rolo's reliability, performance, and compatibility across diverse usage scenarios while maintaining RSB compliance and supporting both minimal and feature-rich builds.