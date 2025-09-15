# Test Data Files

This directory contains test data files for rolo functionality testing.

## Files

- **sample.tsv** - Basic TSV data for table mode testing
- **sample.csv** - CSV data for delimiter testing
- **long_content.tsv** - Table with long text content for width constraint testing
- **unicode_content.tsv** - Japanese text for Unicode width testing
- **simple_list.txt** - Simple newline-separated list for column mode testing
- **uneven_rows.tsv** - TSV with inconsistent column counts for overflow testing
- **ansi_colors.tsv** - Content with ANSI escape sequences for color preservation testing

## Usage

These files are used by both unit tests and integration tests:

```bash
# Table mode testing
cat tests/data/sample.tsv | cargo run -- --table

# Column mode testing
cat tests/data/simple_list.txt | cargo run -- --cols 3

# CSV delimiter testing
cat tests/data/sample.csv | cargo run -- --table --delim ","

# Unicode testing
cat tests/data/unicode_content.tsv | cargo run -- --table
```

## Test Coverage

Each file targets specific functionality:
- Basic formatting and alignment
- Delimiter parsing (tabs, commas)
- Width constraints and truncation
- Unicode character width handling
- ANSI escape sequence preservation
- Edge cases (uneven rows, overflow)