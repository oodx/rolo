# FEATURES_COLUMN_MODE.md

## Overview

Column Mode is one of rolo's core layout modes that transforms line-based input into multi-column formatted output. Implemented as **TASK-007**, this feature provides a powerful alternative to traditional Unix `pr` command with enhanced ANSI-awareness, Unicode support, and flexible configuration options.

Column Mode distributes input items across N columns using a **column-wise distribution algorithm** (items flow down columns, not across rows), making it ideal for displaying file listings, command outputs, and structured data in a compact, readable format.

## Usage Examples

### Basic Column Formatting
```bash
# Convert line-based input to 2 columns
echo -e "apple\nbanana\ncherry\ndate\nelderberry\nfig" | rolo --cols 2
```
**Output:**
```
apple      date
banana     elderberry
cherry     fig
```

### Custom Delimiter Support
```bash
# Use comma-separated input with custom delimiter
echo "red,green,blue,yellow,orange,purple" | rolo --cols 3 --delim ","
```
**Output:**
```
red     yellow
green   orange
blue    purple
```

### Gap Control
```bash
# Adjust spacing between columns
ls | rolo --cols 4 --gap 5
```

### Common Workflows
```bash
# Format directory listings
ls -1 | rolo --cols 3

# Display environment variables in columns
env | rolo --cols 2 --delim "="

# Format semicolon-separated data
echo "item1;item2;item3;item4;item5;item6" | rolo --cols 2 --delim ";"
```

## Technical Details

### Implementation Highlights

**Column Layout Algorithm:**
- **Column-wise distribution**: Items fill down first column, then second, etc.
- **ANSI-aware width calculation**: Uses `width_boxy_adapter.rs` for accurate display width
- **Unicode-safe**: Properly handles wide characters (CJK, emoji)
- **Proportional sizing**: Columns automatically sized based on content

**Configuration Options:**
- `--cols N`: Number of columns (required, must be > 0)
- `--gap N`: Space between columns (default: 2)
- `--delim "X"`: Input delimiter (default: newline)
  - Supports: comma (`,`), semicolon (`;`), space (` `), tab (`\t`), custom
- Width constraints and terminal adaptation

**Core Functions:**
```rust
// Basic column formatting
format_columns(input: &str, columns: usize) -> Result<String, LayoutError>

// With configuration
format_columns_with_config(input: &str, columns: usize, config: &LayoutConfig) -> Result<String, LayoutError>

// With custom delimiter
format_columns_with_delimiter(input: &str, columns: usize, config: &LayoutConfig, delimiter: Option<&str>) -> Result<String, LayoutError>
```

**Error Handling:**
- `InvalidColumnCount(0)`: Zero columns not allowed
- `WidthTooSmall(width, required)`: Terminal too narrow for requested layout
- `ColumnTooNarrow(width)`: Individual columns too narrow for content

### Key Implementation Features

1. **ANSI Preservation**: Color codes and escape sequences pass through unchanged
2. **Memory Efficient**: Streaming implementation for large inputs
3. **Width Adaptation**: Automatically adapts to terminal width or `--width` override
4. **Robust Parsing**: Handles edge cases like empty input, single items, uneven distributions

## Testing Coverage

**Comprehensive Test Suite (12 tests in `tests/baseline/column_mode.rs`):**

### Core Functionality Tests
- ✅ `test_column_mode_basic_functionality`: Validates 2-column layout acceptance criteria
- ✅ `test_column_mode_layout_algorithm`: Verifies column-wise vs row-wise distribution
- ✅ `test_column_mode_with_custom_config`: Tests gap and width configuration

### Edge Case Coverage
- ✅ `test_column_mode_empty_input`: Empty string handling
- ✅ `test_column_mode_single_item`: Single item distribution
- ✅ `test_column_mode_error_conditions`: Zero columns and width validation

### Advanced Features
- ✅ `test_column_mode_ansi_aware_width`: ANSI escape sequence preservation
- ✅ `test_column_mode_unicode_content`: Wide character support (CJK)
- ✅ `test_column_mode_gap_spacing`: Gap configuration testing
- ✅ `test_column_mode_width_constraints`: Width limitation handling

### Integration Testing
- ✅ `test_column_mode_cli_integration`: CLI configuration validation
- ✅ `test_column_mode_delimiter_support`: Comprehensive delimiter testing

**Test Data Integration:**
Uses `tests/data/` fixtures including real-world scenarios with ANSI colors, Unicode content, and various input formats.

## Integration Points

### CLI Integration
**Argument Parsing:**
```rust
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub columns: Option<usize>,    // --cols N
    pub gap: Option<usize>,        // --gap N
    pub delimiter: Option<String>, // --delim "X"
    pub width: Option<usize>,      // --width N
    // ... other fields
}
```

**Command Dispatch:**
Column mode activates when `--cols N` argument is provided. Integrates with RSB dispatch patterns for error handling and validation.

### Stream Processing Integration
**Pipeline Compatibility:**
```bash
# Input sources
cat file.txt | rolo --cols 3
find . -name "*.rs" | rolo --cols 4 --gap 3
env | cut -d= -f1 | rolo --cols 5

# Output chaining
rolo --cols 2 < data.txt | boxy --padding 2
ls | rolo --cols 3 | tee formatted.txt
```

### Width Module Integration
**ANSI-Aware Calculations:**
- Uses `width::get_display_width()` for accurate column sizing
- Leverages `width_boxy_adapter.rs` when `width-boxy` feature enabled
- Fallback width calculation for minimal dependencies

### Layout System Integration
**Shared Infrastructure:**
- Common `LayoutConfig` struct across all layout modes
- Shared error types via `layout::error::LayoutError`
- Consistent ANSI handling and Unicode support patterns

### Feature Flags
**Compilation Variants:**
- **Baseline**: Core functionality without external dependencies
- **Feature-gated**: Enhanced width calculation with `width-boxy` adapter
- **Visual**: Additional formatting options for rich terminal output

## Performance Characteristics

**Benchmarks (from TASK-015 planning):**
- **Memory**: O(n) for input buffering, minimal column state
- **CPU**: Linear time complexity for single-pass processing
- **Streaming**: Handles large inputs via buffered processing
- **Terminal Width**: Cached terminal detection for performance

**Optimization Features:**
- Zero-copy string slicing where possible
- Minimal allocations for column width calculations
- Efficient ANSI sequence detection and preservation

## Future Enhancements

**Planned Improvements (from ROADMAP):**
- **TASK-017**: Per-column alignment options (`--align L,C,R`)
- **TASK-019**: Smart truncation with word boundary detection
- **TASK-025**: Markdown table output mode integration
- **Theme System**: Configurable color schemes and styling

**Integration Roadmap:**
- **TokenStream Support** (TASK-013): Process RSB token streams directly
- **Plugin Architecture** (TASK-022): Extensible column formatting plugins
- **Configuration System** (TASK-026): User-defined column presets