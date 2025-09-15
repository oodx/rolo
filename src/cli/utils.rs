//! CLI utilities - public API per MODULE_SPEC

use crate::cli::error::CliError;
use crate::cli::helpers::{parse_column_count, show_version, show_help};
use crate::width::{validate_width, get_terminal_width};
use crate::layout::{format_columns_with_delimiter, format_list_with_config, LayoutConfig, ListConfig, ListAlignment};
use crate::stream::read_stdin;

/// CLI configuration structure
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub mode: CliMode,
    pub columns: Option<usize>,
    pub width: Option<usize>,
    pub gap: Option<usize>,
    pub delimiter: Option<String>,
    pub fit_mode: bool,
    pub line_numbers: bool,
    pub list_style: Option<String>,
    pub alignment: ListAlignment,
    pub headers: bool,
    pub help: bool,
    pub version: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            mode: CliMode::Columns,
            columns: None,
            width: None,
            gap: None,
            delimiter: None,
            fit_mode: true, // Default to fit mode
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
            headers: false,
            help: false,
            version: false,
        }
    }
}

/// CLI operating modes
#[derive(Debug, Clone)]
pub enum CliMode {
    Columns,
    Table,
    List,
}


/// Parse command line arguments into configuration
pub fn parse_args(args: &[String]) -> Result<CliConfig, CliError> {
    let mut config = CliConfig::default();
    let mut i = 1; // Skip program name

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                config.help = true;
                break; // Help takes precedence
            }
            "--version" | "-V" => {
                config.version = true;
                break; // Version takes precedence
            }
            "--cols" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::MissingArgument("--cols requires a value".to_string()));
                }
                config.columns = Some(parse_column_count(&args[i])?);
            }
            "--width" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::MissingArgument("--width requires a value".to_string()));
                }
                let width = validate_width(&args[i])
                    .map_err(|e| CliError::InvalidWidth(format!("{}", e)))?;
                config.width = Some(width);
            }
            "--gap" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::MissingArgument("--gap requires a value".to_string()));
                }
                match args[i].parse::<usize>() {
                    Ok(gap) if gap <= 20 => config.gap = Some(gap),
                    Ok(gap) => return Err(CliError::InvalidArgument(format!("Gap too large (max 20): {}", gap))),
                    Err(_) => return Err(CliError::InvalidArgument(format!("Invalid gap value: {}", args[i]))),
                }
            }
            "--delim" | "--delimiter" | "--sep" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::MissingArgument("--delim requires a value".to_string()));
                }
                config.delimiter = Some(args[i].clone());
            }
            "--table" => {
                config.mode = CliMode::Table;
            }
            "--fit" => {
                config.fit_mode = true;
            }
            "--no-fit" => {
                config.fit_mode = false;
            }
            "--list" => {
                config.mode = CliMode::List;
            }
            "--line-numbers" | "-n" => {
                config.line_numbers = true;
            }
            "--list-style" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::MissingArgument("--list-style requires a value (numbers, bullets, stars, dots, dash)".to_string()));
                }
                config.list_style = Some(args[i].clone());
            }
            "--align" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::MissingArgument("--align requires a value (left, right, center)".to_string()));
                }
                config.alignment = match args[i].to_lowercase().as_str() {
                    "left" | "l" => ListAlignment::Left,
                    "right" | "r" => ListAlignment::Right,
                    "center" | "c" => ListAlignment::Center,
                    _ => return Err(CliError::InvalidArgument(format!("Invalid alignment: {}. Use left, right, or center", args[i]))),
                };
            }
            "table" => config.mode = CliMode::Table,
            "list" => config.mode = CliMode::List,
            "columns" => config.mode = CliMode::Columns,
            arg if arg.starts_with("--") => {
                return Err(CliError::InvalidArgument(format!("Unknown option: {}", arg)));
            }
            _ => {
                // Positional arguments (subcommands without --)
                match args[i].as_str() {
                    "table" => config.mode = CliMode::Table,
                    "list" => config.mode = CliMode::List,
                    "columns" => config.mode = CliMode::Columns,
                    _ => return Err(CliError::UnsupportedCommand(args[i].clone())),
                }
            }
        }
        i += 1;
    }

    Ok(config)
}

/// Execute CLI action based on configuration
pub fn execute_cli(config: &CliConfig) -> Result<(), CliError> {
    if config.help {
        show_help();
        return Ok(());
    }

    if config.version {
        show_version();
        return Ok(());
    }

    // Process input through layout system
    match config.mode {
        CliMode::Columns => {
            let cols = config.columns.unwrap_or(2);
            let width = if config.fit_mode {
                config.width.unwrap_or_else(|| get_terminal_width())
            } else {
                config.width.unwrap_or(80) // Fixed width when not in fit mode
            };
            let gap = config.gap.unwrap_or(2);

            let layout_config = LayoutConfig {
                width,
                gap,
                padding: 1,
            };

            // Read input from stdin
            let input = read_stdin()
                .map_err(|e| CliError::ProcessingError(format!("Failed to read input: {}", e)))?;

            // Format into columns with optional delimiter
            let output = format_columns_with_delimiter(&input, cols, &layout_config, config.delimiter.as_deref())
                .map_err(|e| CliError::ProcessingError(format!("Column formatting failed: {}", e)))?;

            println!("{}", output);
        }
        CliMode::Table => {
            let delimiter = config.delimiter.as_deref().unwrap_or("\t");
            let width = if config.fit_mode {
                config.width.unwrap_or_else(|| get_terminal_width())
            } else {
                config.width.unwrap_or(80) // Fixed width when not in fit mode
            };

            // Read input from stdin
            let input = read_stdin()
                .map_err(|e| CliError::ProcessingError(format!("Failed to read input: {}", e)))?;

            // Format as table
            let output = crate::layout::format_table_with_config(&input, delimiter, width)
                .map_err(|e| CliError::ProcessingError(format!("Table formatting failed: {}", e)))?;

            println!("{}", output);
        }
        CliMode::List => {
            let width = if config.fit_mode {
                config.width.unwrap_or_else(|| get_terminal_width())
            } else {
                config.width.unwrap_or(80) // Fixed width when not in fit mode
            };

            let list_config = ListConfig {
                width,
                line_numbers: config.line_numbers,
                list_style: config.list_style.clone(),
                alignment: config.alignment.clone(),
            };

            // Read input from stdin
            let input = read_stdin()
                .map_err(|e| CliError::ProcessingError(format!("Failed to read input: {}", e)))?;

            // Format as list
            let output = format_list_with_config(&input, &list_config)
                .map_err(|e| CliError::ProcessingError(format!("List formatting failed: {}", e)))?;

            println!("{}", output);
        }
    }

    Ok(())
}