//! CLI internal helpers per MODULE_SPEC

use crate::cli::error::CliError;

/// Parse column count from argument
pub(crate) fn parse_column_count(arg: &str) -> Result<usize, CliError> {
    match arg.parse::<usize>() {
        Ok(n) if n > 0 && n <= 10 => Ok(n),
        Ok(n) => Err(CliError::InvalidArgument(format!("Column count {} out of range (1-10)", n))),
        Err(_) => Err(CliError::InvalidArgument(format!("Column count must be a number: {}", arg))),
    }
}

/// Show version information
pub(crate) fn show_version() {
    println!("rolo v{}", env!("CARGO_PKG_VERSION"));
    println!("The spiritual love child of pr, paste, and col");
    println!("Built with RSB Framework");
}

/// Show help message
pub(crate) fn show_help() {
    println!("rolo v{} - Text layout tool for Unix pipelines", env!("CARGO_PKG_VERSION"));
    println!("The spiritual love child of pr, paste, and col");
    println!();
    println!("USAGE:");
    println!("    rolo [OPTIONS] [SUBCOMMAND]");
    println!();
    println!("OPTIONS:");
    println!("    --cols N        Format text in N columns (1-10)");
    println!("    --width N       Set terminal width (10-200)");
    println!("    --help, -h      Show this help message");
    println!("    --version, -V   Show version information");
    println!();
    println!("SUBCOMMANDS:");
    println!("    table           Format as table with headers");
    println!("    list            Format as bulleted list");
    println!("    columns         Format in columns (default)");
    println!();
    println!("EXAMPLES:");
    println!("    printf '%s\\n' $LIST | rolo --cols 4");
    println!("    cat data.tsv | rolo table");
    println!("    ls -la | rolo --cols 3 --width 120");
    println!();
    println!("PIPELINE INTEGRATION:");
    println!("    echo \"text\" | jynx | rolo --cols 2 | boxy");
}