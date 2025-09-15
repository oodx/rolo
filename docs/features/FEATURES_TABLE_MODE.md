# FEATURES_TABLE_MODE.md

## Overview

Table Mode is rolo's premier tabular data formatting system, implemented as **TASK-008**. It transforms delimited input data into properly aligned, visually appealing tables with automatic column width detection, header separation, and robust handling of irregular data structures.

Unlike simple column alignment tools, Table Mode provides intelligent parsing of CSV, TSV, and custom-delimited data with features like proportional column compression, content truncation, overflow handling, and full ANSI/Unicode support.

## Usage Examples

### Basic Table Formatting
```bash
# Format TSV data with automatic header detection
cat tests/data/sample.tsv | rolo --table
```
**Output:**
```
Name         | Age | City     | Department
-------------+-----+----------+-------------
John Doe     | 25  | New York | Engineering
Alice Smith  | 30  | London   | Marketing
Bob Johnson  | 35  | Tokyo    | Sales
Carol Brown  | 28  | Paris    | Design
David Wilson | 32  | Berlin   | Engineering
Eva Garcia   | 27  | Madrid   | Marketing
```

### CSV Data Processing
```bash
# Process CSV with custom delimiter
echo "Product,Price,Stock\nApple,1.50,100\nBanana,0.75,50" | rolo --table --delim ","
```
**Output:**
```
Product | Price | Stock
--------+-------+------
Apple   | 1.50  | 100
Banana  | 0.75  | 50
```

### Custom Delimiters
```bash
# Semicolon-separated European data format
echo "Name;Age;Country\nJohn;25;USA\nMarie;30;France" | rolo --table --delim ";"

# Pipe-separated data
echo "A|B|C\nX|Y|Z" | rolo --table --delim "|"
```

### Width-Constrained Tables
```bash
# Limit table width for narrow terminals
cat large_data.tsv | rolo --table --width 60
```

### Real-World Workflows
```bash
# Format system information
ps aux | head -10 | rolo --table --delim " "

# Process database exports
mysql -e "SELECT * FROM users LIMIT 5" | rolo --table

# Format configuration files
cat /etc/passwd | cut -d: -f1,3,5 | rolo --table --delim ":"
```

## Technical Details

### Implementation Highlights

**Auto-Width Detection Algorithm:**
- **Dynamic column sizing**: Calculates optimal width for each column based on content
- **Proportional compression**: When width-constrained, compresses columns proportionally
- **Content-aware padding**: Maintains readability while maximizing space utilization
- **Header detection**: Automatically identifies and separates header rows

**Delimiter Support:**
- **TSV** (Tab-separated): Default mode, handles `\t` delimiters
- **CSV** (Comma-separated): Standard RFC 4180 compliance
- **Custom delimiters**: Any single or multi-character separator
- **Whitespace handling**: Intelligent trimming and preservation

**Core Functions:**
```rust
// Basic table formatting with TSV default
format_table(input: &str, delimiter: &str) -> Result<String, LayoutError>

// With width constraints
format_table_with_config(input: &str, delimiter: &str, max_width: usize) -> Result<String, LayoutError>
```

**Advanced Features:**
1. **Row Overflow Handling**: Gracefully handles uneven row structures
2. **Content Truncation**: Ellipsis-based truncation for oversized cells
3. **ANSI Preservation**: Color codes maintained through formatting
4. **Unicode Support**: Proper handling of wide characters and emojis

### Key Implementation Components

**Table Structure Generation:**
```rust
// Table cell representation
struct TableCell {
    content: String,
    display_width: usize,
    ansi_stripped: String,
}

// Column metadata
struct ColumnInfo {
    max_width: usize,
    min_width: usize,
    content_examples: Vec<String>,
}
```

**Width Calculation Strategy:**
1. **First Pass**: Scan all rows to determine natural column widths
2. **Constraint Check**: Validate against terminal/specified width limits
3. **Compression**: Apply proportional reduction when necessary
4. **Truncation**: Apply ellipsis to cells exceeding compressed width

**Error Handling:**
- `EmptyInput`: Graceful handling of empty or whitespace-only input
- `InvalidDelimiter`: Validation of delimiter characters
- `RowParseError`: Recovery from malformed rows
- `WidthConstraintError`: Handling of impossible width requirements

## Testing Coverage

**Comprehensive Test Suite (14 tests in `tests/baseline/table_mode.rs`):**

### Core Functionality Tests
- ✅ `test_table_mode_basic_functionality`: TSV acceptance criteria validation
- ✅ `test_table_mode_with_csv_delimiter`: CSV parsing and formatting
- ✅ `test_table_mode_auto_width_detection`: Dynamic column sizing verification

### Data Structure Handling
- ✅ `test_table_mode_row_overflow_handling`: Uneven row structure support
- ✅ `test_table_mode_single_row`: Header-only table handling
- ✅ `test_table_mode_single_column`: Single-column table formatting
- ✅ `test_table_mode_empty_input`: Empty and whitespace-only input

### Advanced Features Testing
- ✅ `test_table_mode_with_width_constraints`: Width limitation compliance
- ✅ `test_table_mode_truncation`: Content truncation with ellipsis
- ✅ `test_table_mode_with_spaces_in_cells`: Multi-word cell content

### Character Encoding Support
- ✅ `test_table_mode_ansi_aware_content`: ANSI escape sequence preservation
- ✅ `test_table_mode_unicode_content`: Unicode character handling (CJK)

### Delimiter and Integration Testing
- ✅ `test_table_mode_custom_delimiters`: Multiple delimiter formats
- ✅ `test_table_mode_cli_integration`: CLI configuration validation

**Integration Test Data:**
Utilizes comprehensive test fixtures in `tests/data/`:
- `sample.tsv`: Real-world employee data with mixed content types
- `sample.csv`: Standard CSV format with pricing data
- `unicode_content.tsv`: CJK characters and emoji testing
- `ansi_colors.tsv`: ANSI escape sequence preservation
- `uneven_rows.tsv`: Irregular data structure handling
- `long_content.tsv`: Content truncation scenarios

## Integration Points

### CLI Integration
**Table Mode Activation:**
```rust
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub mode: CliMode,             // CliMode::Table
    pub delimiter: Option<String>, // --delim "X"
    pub width: Option<usize>,      // --width N
    pub table: bool,               // --table flag
    // ... other configuration
}
```

**Command Line Usage:**
```bash
rolo --table                    # TSV mode (default)
rolo --table --delim ","        # CSV mode
rolo --table --delim ";" --width 80  # Custom delimiter with width limit
```

### Stream Processing Integration
**Pipeline Compatibility:**
```bash
# Database exports
mysql -B -e "QUERY" | rolo --table

# Log file processing
grep ERROR /var/log/app.log | cut -d' ' -f1,3,5 | rolo --table

# System monitoring
top -b -n1 | tail -n+8 | rolo --table --delim " "

# Configuration analysis
cat /etc/hosts | rolo --table --delim " "
```

### Width Module Integration
**ANSI-Aware Width Calculations:**
- Leverages `width::get_display_width()` for accurate cell sizing
- Integrates with `width_boxy_adapter.rs` for enhanced precision
- Fallback implementations for minimal dependency builds

**Terminal Adaptation:**
- Automatic terminal width detection via `get_terminal_width()`
- Respects `--width` override for fixed-size outputs
- Responsive design for terminal resize events

### Layout System Integration
**Shared Components:**
- Common `LayoutConfig` infrastructure with column and list modes
- Unified error handling via `layout::error::LayoutError`
- Consistent ANSI processing pipeline
- Shared Unicode width calculation utilities

### File Format Support
**Current Implementations:**
- **TSV**: Tab-separated values (primary target)
- **CSV**: Comma-separated values with quote handling
- **SSV**: Space-separated values with whitespace normalization
- **Custom**: User-defined single or multi-character delimiters

**Future Format Support (Roadmap):**
- **JSON flattening** (TASK-024): Nested object table representation
- **XML parsing**: Element-to-row transformation
- **Fixed-width**: Legacy mainframe data format support

## Performance Characteristics

**Scalability Metrics:**
- **Memory Usage**: O(n) for row buffering, O(cols) for width calculation
- **Processing Time**: Linear scan with minimal overhead
- **Large File Support**: Streaming implementation for GB-scale inputs
- **Terminal Responsiveness**: Cached width calculations

**Optimization Features:**
- **Lazy Row Processing**: On-demand parsing for memory efficiency
- **Width Caching**: Column width calculations cached per delimiter type
- **ANSI Sequence Optimization**: Efficient escape code detection and preservation
- **Unicode Width Caching**: Pre-computed width tables for common characters

**Benchmark Targets (TASK-015):**
- Process 10K+ rows/second on standard hardware
- Handle files up to 1GB without memory pressure
- Sub-100ms formatting for typical terminal-sized outputs

## Advanced Features

### Header Detection Algorithm
**Automatic Header Identification:**
1. **Content Analysis**: Examines first row for non-numeric patterns
2. **Delimiter Consistency**: Validates column count consistency
3. **Separator Generation**: Creates appropriate ASCII art separators
4. **Alignment Inference**: Detects optimal column alignment patterns

### Row Overflow Handling
**Strategies for Irregular Data:**
- **Column Extension**: Dynamically adds columns for oversized rows
- **Missing Cell Handling**: Empty cells for undersized rows
- **Data Recovery**: Preserves all content despite structural inconsistencies
- **Error Reporting**: Optional warnings for data quality issues

### Content Truncation System
**Intelligent Content Management:**
```rust
// Truncation configuration
struct TruncationConfig {
    max_cell_width: usize,
    ellipsis_style: EllipsisStyle,  // "...", "…", custom
    word_boundary_aware: bool,
    preserve_ansi: bool,
}
```

**Truncation Strategies:**
- **Character-based**: Hard truncation at width limit
- **Word-boundary**: Intelligent breaking at word boundaries
- **ANSI-aware**: Preserves color formatting through truncation
- **Progressive**: Graceful degradation as width decreases

## Future Enhancements

**Immediate Roadmap (v0.3-0.4):**
- **TASK-017**: Per-column alignment control (`--align L,C,R`)
- **TASK-018**: Border style options (`--border unicode|ascii|none`)
- **TASK-019**: Enhanced truncation policies with word-wrap
- **TASK-020**: Improved error messages and malformed data recovery

**Advanced Features (v0.5+):**
- **TASK-024**: JSON table flattening plugin
- **TASK-025**: Markdown table output format
- **TASK-026**: Configuration file support for table presets
- **Plugin Architecture**: Custom formatter plugins for specialized data types

**Integration Enhancements:**
- **Database Connectors**: Direct database query result formatting
- **Spreadsheet Export**: Excel/LibreOffice compatible output
- **API Integration**: REST API response table formatting
- **Configuration Management**: User-defined table templates and styles