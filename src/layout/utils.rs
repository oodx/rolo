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
    format_columns_with_delimiter(text, cols, config, None)
}

/// Format text into columns with custom configuration and delimiter
pub fn format_columns_with_delimiter(text: &str, cols: usize, config: &LayoutConfig, delimiter: Option<&str>) -> Result<String, LayoutError> {
    if cols == 0 {
        return Err(LayoutError::InvalidColumnCount(0));
    }

    // Split input into items based on delimiter
    let items: Vec<&str> = match delimiter {
        Some(delim) if !delim.is_empty() => {
            // First split by lines, then by custom delimiter
            let mut all_items = Vec::new();
            for line in text.lines() {
                if line.trim().is_empty() {
                    continue; // Skip empty lines
                }
                for item in line.split(delim) {
                    let trimmed = item.trim();
                    if !trimmed.is_empty() {
                        all_items.push(trimmed);
                    }
                }
            }
            all_items
        }
        _ => {
            // Default: split by lines only
            text.lines().filter(|line| !line.trim().is_empty()).collect()
        }
    };

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

/// Format text into table with delimiter and width constraints
pub fn format_table_with_config(text: &str, delimiter: &str, width: usize) -> Result<String, LayoutError> {
    if text.trim().is_empty() {
        return Ok(String::new());
    }

    // Parse input into rows and columns
    let rows: Vec<Vec<&str>> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split(delimiter).collect())
        .collect();

    if rows.is_empty() {
        return Ok(String::new());
    }

    // Determine maximum number of columns
    let max_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    if max_cols == 0 {
        return Ok(String::new());
    }

    // Calculate column widths based on content
    let mut col_widths = vec![0; max_cols];
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if i < max_cols {
                let cell_width = get_display_width(cell.trim()).unwrap_or(cell.trim().len());
                col_widths[i] = col_widths[i].max(cell_width);
            }
        }
    }

    // Calculate total required width including separators
    let separator_width = 3; // " | " between columns
    let total_separator_width = (max_cols - 1) * separator_width;
    let total_content_width: usize = col_widths.iter().sum();
    let required_width = total_content_width + total_separator_width;

    // Handle width constraints - compress columns if necessary
    if required_width > width && width > total_separator_width {
        let available_content_width = width - total_separator_width;
        let scale_factor = available_content_width as f64 / total_content_width as f64;

        // Scale down column widths proportionally
        for col_width in &mut col_widths {
            *col_width = ((*col_width as f64 * scale_factor).max(3.0)) as usize;
        }
    }

    // Format the table
    let mut result = Vec::new();

    for (row_idx, row) in rows.iter().enumerate() {
        let mut line = String::new();

        for (col_idx, &cell) in row.iter().enumerate() {
            if col_idx >= max_cols {
                break;
            }

            let cell_content = cell.trim();
            let cell_width = get_display_width(cell_content).unwrap_or(cell_content.len());
            let target_width = col_widths[col_idx];

            // Handle overflow by truncating with ellipsis
            let formatted_cell = if cell_width > target_width {
                if target_width >= 3 {
                    format!("{}...", &cell_content[..target_width.saturating_sub(3)])
                } else {
                    "...".to_string()
                }
            } else {
                format!("{:width$}", cell_content, width = target_width)
            };

            line.push_str(&formatted_cell);

            // Add separator except for last column
            if col_idx < max_cols - 1 {
                line.push_str(" | ");
            }
        }

        result.push(line.trim_end().to_string());

        // Add header separator after first row (if it looks like a header)
        if row_idx == 0 && rows.len() > 1 {
            let mut separator = String::new();
            for (col_idx, &width) in col_widths.iter().enumerate() {
                separator.push_str(&"-".repeat(width));
                if col_idx < max_cols - 1 {
                    separator.push_str("-+-");
                }
            }
            result.push(separator);
        }
    }

    Ok(result.join("\n"))
}

/// Format text into table (convenience function)
pub fn format_table(text: &str, delimiter: &str) -> Result<String, LayoutError> {
    format_table_with_config(text, delimiter, 80)
}

/// List formatting configuration
pub struct ListConfig {
    pub width: usize,
    pub line_numbers: bool,
    pub list_style: Option<String>,
    pub alignment: ListAlignment,
}

impl Default for ListConfig {
    fn default() -> Self {
        Self {
            width: 80,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        }
    }
}

/// List alignment options
#[derive(Debug, Clone)]
pub enum ListAlignment {
    Left,
    Right,
    Center,
}

/// Format text as list with line numbers and alignment
pub fn format_list_with_config(text: &str, config: &ListConfig) -> Result<String, LayoutError> {
    if text.trim().is_empty() {
        return Ok(String::new());
    }

    let lines: Vec<&str> = text.lines().filter(|line| !line.trim().is_empty()).collect();

    if lines.is_empty() {
        return Ok(String::new());
    }

    let mut result = Vec::new();
    let line_count = lines.len();

    // Calculate number/style width for line markers
    let (marker_width, use_line_numbers) = if config.line_numbers {
        (line_count.to_string().len(), true)
    } else if config.list_style.is_some() {
        (get_list_style_width(&config.list_style), false)
    } else {
        (0, false)
    };

    // Calculate available content width
    let separator_width = if use_line_numbers || config.list_style.is_some() { 1 } else { 0 }; // " " after marker
    let available_width = if config.width > marker_width + separator_width {
        config.width - marker_width - separator_width
    } else {
        config.width.max(10) // Minimum usable width
    };

    for (i, line) in lines.iter().enumerate() {
        let line_content = line.trim();
        let line_number = i + 1;

        // Format the line content with alignment
        let aligned_content = match config.alignment {
            ListAlignment::Left => {
                if line_content.len() <= available_width {
                    format!("{:<width$}", line_content, width = available_width)
                } else {
                    // Truncate with ellipsis if too long
                    if available_width >= 3 {
                        format!("{}...", &line_content[..available_width.saturating_sub(3)])
                    } else {
                        "...".to_string()
                    }
                }
            }
            ListAlignment::Right => {
                if line_content.len() <= available_width {
                    format!("{:>width$}", line_content, width = available_width)
                } else {
                    // Truncate with ellipsis if too long
                    if available_width >= 3 {
                        format!("...{}", &line_content[line_content.len().saturating_sub(available_width - 3)..])
                    } else {
                        "...".to_string()
                    }
                }
            }
            ListAlignment::Center => {
                if line_content.len() <= available_width {
                    format!("{:^width$}", line_content, width = available_width)
                } else {
                    // Truncate with ellipsis if too long
                    if available_width >= 3 {
                        format!("{}...", &line_content[..available_width.saturating_sub(3)])
                    } else {
                        "...".to_string()
                    }
                }
            }
        };

        // Add line marker if requested
        let formatted_line = if use_line_numbers {
            format!("{:width$}. {}", line_number, aligned_content.trim_end(), width = marker_width)
        } else if let Some(style) = &config.list_style {
            let marker = get_list_style_marker(style, line_number);
            format!("{} {}", marker, aligned_content.trim_end())
        } else {
            aligned_content.trim_end().to_string()
        };

        result.push(formatted_line);
    }

    Ok(result.join("\n"))
}

/// Format text as list (convenience function)
pub fn format_list(text: &str) -> Result<String, LayoutError> {
    format_list_with_config(text, &ListConfig::default())
}

/// Get the width needed for a list style marker
fn get_list_style_width(style: &Option<String>) -> usize {
    match style.as_deref() {
        Some("numbers") => 2, // e.g., "1."
        Some("bullets") => 1, // •
        Some("stars") => 1,   // *
        Some("dots") => 1,    // ·
        Some("dash") => 1,    // -
        _ => 1, // Default to single character
    }
}

/// Get the marker string for a given list style and line number
fn get_list_style_marker(style: &str, line_number: usize) -> String {
    match style {
        "numbers" => format!("{}.", line_number),
        "bullets" => "•".to_string(),
        "stars" => "*".to_string(),
        "dots" => "·".to_string(),
        "dash" => "-".to_string(),
        _ => "•".to_string(), // Default to bullet
    }
}