//! CLI utilities - public API per MODULE_SPEC

use crate::cli::error::CliError;
use crate::cli::helpers::{parse_column_count, show_version, show_help};
use crate::width::{validate_width, get_terminal_width};
use crate::layout::{format_columns_with_delimiter, LayoutConfig};
use crate::stream::read_stdin;

/// CLI configuration structure
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub mode: CliMode,
    pub columns: Option<usize>,
    pub width: Option<usize>,
    pub gap: Option<usize>,
    pub delimiter: Option<String>,
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
            let width = config.width.unwrap_or_else(|| get_terminal_width());
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
            let width = config.width.unwrap_or_else(|| get_terminal_width());

            // Read input from stdin
            let input = read_stdin()
                .map_err(|e| CliError::ProcessingError(format!("Failed to read input: {}", e)))?;

            // Format as table
            let output = crate::layout::format_table_with_config(&input, delimiter, width)
                .map_err(|e| CliError::ProcessingError(format!("Table formatting failed: {}", e)))?;

            println!("{}", output);
        }
        CliMode::List => {
            return Err(CliError::UnsupportedCommand("List mode not yet implemented".to_string()));
        }
    }

    Ok(())
}