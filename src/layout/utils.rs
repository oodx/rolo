//! Layout utilities - curated low-level helpers per MODULE_SPEC

use crate::layout::error::LayoutError;
use crate::width::get_display_width;

/// Basic layout configuration
pub struct LayoutConfig {
    pub width: usize,
    pub gap: usize,
    pub padding: usize,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            width: 80,
            gap: 2,
            padding: 1,
        }
    }
}

/// Format text into columns with proper ANSI-aware width handling
pub fn format_columns(text: &str, cols: usize) -> Result<String, LayoutError> {
    if cols == 0 {
        return Err(LayoutError::InvalidColumnCount(0));
    }

    format_columns_with_config(text, cols, &LayoutConfig::default())
}

/// Format text into columns with custom configuration
pub fn format_columns_with_config(text: &str, cols: usize, config: &LayoutConfig) -> Result<String, LayoutError> {
    if cols == 0 {
        return Err(LayoutError::InvalidColumnCount(0));
    }

    // Split input into items (lines or words)
    let items: Vec<&str> = text.lines().collect();
    if items.is_empty() {
        return Ok(String::new());
    }

    // Calculate column width
    let total_gap_space = (cols - 1) * config.gap;
    if config.width <= total_gap_space {
        return Err(LayoutError::WidthTooSmall(config.width, total_gap_space));
    }

    let available_width = config.width - total_gap_space;
    let col_width = available_width / cols;

    if col_width < 3 {
        return Err(LayoutError::ColumnTooNarrow(col_width));
    }

    // Calculate number of rows needed
    let rows = (items.len() + cols - 1) / cols;
    let mut result = Vec::new();

    for row in 0..rows {
        let mut line = String::new();

        for col in 0..cols {
            let item_index = row + col * rows;

            if item_index < items.len() {
                let item = items[item_index];
                let display_width = get_display_width(item).unwrap_or(item.len());

                // Add the item
                line.push_str(item);

                // Add padding to reach column width (except for last column)
                if col < cols - 1 {
                    let padding_needed = if display_width < col_width {
                        col_width - display_width
                    } else {
                        0
                    };

                    // Add padding spaces
                    for _ in 0..padding_needed {
                        line.push(' ');
                    }

                    // Add gap between columns
                    for _ in 0..config.gap {
                        line.push(' ');
                    }
                }
            }
        }

        // Remove trailing whitespace
        let trimmed_line = line.trim_end();
        result.push(trimmed_line.to_string());
    }

    Ok(result.join("\n"))
}

/// Format text into table (placeholder implementation)
pub fn format_table(_text: &str, _delimiter: &str) -> Result<String, LayoutError> {
    // TODO: Implement in TASK-008
    Ok("Table formatting not yet implemented".to_string())
}

/// Format text as list (placeholder implementation)
pub fn format_list(_text: &str) -> Result<String, LayoutError> {
    // TODO: Implement in TASK-009
    Ok("List formatting not yet implemented".to_string())
}