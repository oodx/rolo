//! CLI utilities - public API per MODULE_SPEC

use crate::cli::error::CliError;
use crate::cli::helpers::{parse_column_count, show_version, show_help};
use crate::width::validate_width;

/// CLI configuration structure
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub mode: CliMode,
    pub columns: Option<usize>,
    pub width: Option<usize>,
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

    // TODO: Integrate with actual layout processing
    match config.mode {
        CliMode::Columns => {
            let cols = config.columns.unwrap_or(2);
            println!("Processing input in {} columns mode", cols);
            if let Some(width) = config.width {
                println!("Using terminal width: {}", width);
            }
        }
        CliMode::Table => {
            println!("Processing input in table mode");
        }
        CliMode::List => {
            println!("Processing input in list mode");
        }
    }

    Ok(())
}