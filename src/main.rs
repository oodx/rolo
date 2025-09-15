//! Rolo CLI entry point - RSB Framework integration

use rsb::prelude::*;

fn main() {
    // RSB Framework integration
    let args = bootstrap!();

    // Handle admin commands first (no full context needed)
    if pre_dispatch!(&args, {
        "init" => cmd_init,
        "version" => cmd_version,
        "help" => cmd_help
    }) {
        return; // Admin command handled, exit early
    }

    // Parse CLI options into global context
    options!(&args);

    // Main command dispatch with proper RSB routing
    dispatch!(&args, {
        "list" => cmd_list,
        "table" => cmd_table,
        "columns" => cmd_columns
    });
}

// RSB command handlers (Args) -> i32

fn cmd_init(_args: Args) -> i32 {
    echo!("🚀 Rolo initialized");
    0
}

fn cmd_version(_args: Args) -> i32 {
    echo!("rolo {}", env!("CARGO_PKG_VERSION"));
    0
}

fn cmd_help(_args: Args) -> i32 {
    echo!("Rolo - Text layout tool for Unix pipelines");
    echo!("");
    echo!("USAGE:");
    echo!("    rolo [COMMAND] [OPTIONS]");
    echo!("");
    echo!("COMMANDS:");
    echo!("    list       Format as list with optional line numbers");
    echo!("    table      Format as table with delimiter detection");
    echo!("    columns    Format as columns (default)");
    echo!("");
    echo!("OPTIONS:");
    echo!("    --cols=N           Number of columns");
    echo!("    --width=N          Terminal width");
    echo!("    --gap=N            Gap between columns");
    echo!("    --delim=STR        Delimiter for input parsing");
    echo!("    --line-numbers     Add line numbers to list mode");
    echo!("    --list-style=STYLE List style (bullets, stars, numbers, dash, dots)");
    echo!("    --align=ALIGN      Alignment (left, right, center)");
    echo!("    --fit              Fit to terminal width (default)");
    echo!("    --no-fit           Use fixed width");
    0
}

fn cmd_list(args: Args) -> i32 {
    if let Err(e) = execute_list_command(args) {
        stderr!("❌ List command failed: {}", e);
        return 1;
    }
    0
}

fn cmd_table(args: Args) -> i32 {
    if let Err(e) = execute_table_command(args) {
        stderr!("❌ Table command failed: {}", e);
        return 1;
    }
    0
}

fn cmd_columns(args: Args) -> i32 {
    if let Err(e) = execute_columns_command(args) {
        stderr!("❌ Columns command failed: {}", e);
        return 1;
    }
    0
}

// Command implementation functions

fn execute_list_command(_args: Args) -> Result<(), Box<dyn std::error::Error>> {
    use rololib::prelude::*;

    // Get options from global context (RSB pattern)
    let width = if is_true("opt_fit") {
        get_var("opt_width").parse().unwrap_or_else(|_| get_terminal_width())
    } else {
        get_var("opt_width").parse().unwrap_or(80)
    };

    let line_numbers = is_true("opt_line_numbers");
    let list_style = {
        let style_val = get_var("opt_list_style");
        if style_val.is_empty() { None } else { Some(style_val) }
    };
    let alignment = match get_var("opt_align").as_str() {
        "right" => ListAlignment::Right,
        "center" => ListAlignment::Center,
        _ => ListAlignment::Left,
    };

    let list_config = ListConfig {
        width,
        line_numbers,
        list_style,
        alignment,
    };

    // Read input from stdin
    let input = read_stdin()?;

    // Format as list
    let output = format_list_with_config(&input, &list_config)?;

    echo!("{}", output);
    Ok(())
}

fn execute_table_command(_args: Args) -> Result<(), Box<dyn std::error::Error>> {
    use rololib::prelude::*;

    // Get options from global context
    let delimiter = {
        let delim = get_var("opt_delim");
        if delim.is_empty() { "\t".to_string() } else { delim }
    };

    let width = if is_true("opt_fit") {
        get_var("opt_width").parse().unwrap_or_else(|_| get_terminal_width())
    } else {
        get_var("opt_width").parse().unwrap_or(80)
    };

    // Read input from stdin
    let input = read_stdin()?;

    // Format as table
    let output = format_table_with_config(&input, &delimiter, width)?;

    echo!("{}", output);
    Ok(())
}

fn execute_columns_command(_args: Args) -> Result<(), Box<dyn std::error::Error>> {
    use rololib::prelude::*;

    // Get options from global context
    let cols = get_var("opt_cols").parse().unwrap_or(2);
    let width = if is_true("opt_fit") {
        get_var("opt_width").parse().unwrap_or_else(|_| get_terminal_width())
    } else {
        get_var("opt_width").parse().unwrap_or(80)
    };
    let gap = get_var("opt_gap").parse().unwrap_or(2);

    let layout_config = LayoutConfig {
        width,
        gap,
        padding: 1,
    };

    // Read input from stdin
    let input = read_stdin()?;

    // Format into columns with optional delimiter
    let delimiter = {
        let delim = get_var("opt_delim");
        if delim.is_empty() { None } else { Some(delim) }
    };

    let output = format_columns_with_delimiter(&input, cols, &layout_config, delimiter.as_deref())?;

    echo!("{}", output);
    Ok(())
}