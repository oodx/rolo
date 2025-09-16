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
    options(&args);

    // Main command dispatch with proper RSB routing
    dispatch!(&args, {
        "list" => cmd_list,
        "table" => cmd_table,
        "columns" => cmd_columns
    });
}

// RSB options parser
fn options(args: &Args) {
    // Set default values
    set_var("opt_cols", "2");
    set_var("opt_width", "80");
    set_var("opt_gap", "2");
    set_var("opt_fit", "true");
    set_var("opt_delim", "");
    set_var("opt_sep", "");
    set_var("opt_line_numbers", "false");
    set_var("opt_list_style", "");
    set_var("opt_align", "left");

    // Parse options from args
    let mut i = 1; // Start at 1 to skip command name
    while i <= args.len() {
        let arg = args.get(i);

        // Column count
        if arg == "--cols" && i < args.len() {
            let val = args.get(i + 1);
            if !val.is_empty() {
                set_var("opt_cols", &val);
                i += 1;
            }
        }
        // Width
        else if arg == "--width" && i < args.len() {
            let val = args.get(i + 1);
            if !val.is_empty() {
                set_var("opt_width", &val);
                set_var("opt_fit", "false"); // Explicit width disables fit
                i += 1;
            }
        }
        // Gap between columns
        else if arg == "--gap" && i < args.len() {
            let val = args.get(i + 1);
            if !val.is_empty() {
                set_var("opt_gap", &val);
                i += 1;
            }
        }
        // Delimiter/separator for input splitting
        else if (arg == "--delim" || arg == "--delimiter" || arg == "--sep") && i < args.len() {
            let val = args.get(i + 1);
            if !val.is_empty() {
                set_var("opt_delim", &val);
                set_var("opt_sep", &val); // Store in both for compatibility
                i += 1;
            }
        }
        // Table mode
        else if arg == "--table" {
            set_var("opt_mode", "table");
        }
        // List mode
        else if arg == "--list" {
            set_var("opt_mode", "list");
        }
        // Fit mode
        else if arg == "--fit" {
            set_var("opt_fit", "true");
        }
        else if arg == "--no-fit" {
            set_var("opt_fit", "false");
        }
        // List options
        else if arg == "--line-numbers" || arg == "-n" {
            set_var("opt_line_numbers", "true");
        }
        else if arg == "--list-style" && i < args.len() {
            let val = args.get(i + 1);
            if !val.is_empty() {
                set_var("opt_list_style", &val);
                i += 1;
            }
        }
        else if arg == "--align" && i < args.len() {
            let val = args.get(i + 1);
            if !val.is_empty() {
                set_var("opt_align", &val);
                i += 1;
            }
        }
        // Handle flag=value format
        else if arg.contains('=') {
            let parts: Vec<&str> = arg.splitn(2, '=').collect();
            if parts.len() == 2 {
                let flag = parts[0];
                let value = parts[1];

                match flag {
                    "--cols" => set_var("opt_cols", value),
                    "--width" => {
                        set_var("opt_width", value);
                        set_var("opt_fit", "false");
                    },
                    "--gap" => set_var("opt_gap", value),
                    "--delim" | "--delimiter" | "--sep" => {
                        set_var("opt_delim", value);
                        set_var("opt_sep", value);
                    },
                    "--list-style" => set_var("opt_list_style", value),
                    "--align" => set_var("opt_align", value),
                    _ => {}
                }
            }
        }

        i += 1;
    }
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

    // Handle separator for list mode
    let separator = {
        let sep = get_var("opt_sep");
        if !sep.is_empty() {
            sep
        } else {
            get_var("opt_delim")
        }
    };

    let processed_input = if !separator.is_empty() {
        // Split input by separator and create one item per line
        let mut items = Vec::new();
        for line in input.lines() {
            for item in line.split(&separator) {
                let trimmed = item.trim();
                if !trimmed.is_empty() {
                    items.push(trimmed.to_string());
                }
            }
        }
        items.join("\n")
    } else {
        input
    };

    // Format as list
    let output = format_list_with_config(&processed_input, &list_config)?;

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

    // Get separator/delimiter for input splitting
    let separator = {
        let sep = get_var("opt_sep");
        if sep.is_empty() {
            let delim = get_var("opt_delim");
            if delim.is_empty() { None } else { Some(delim) }
        } else {
            Some(sep)
        }
    };

    let output = format_columns_with_delimiter(&input, cols, &layout_config, separator.as_deref())?;

    echo!("{}", output);
    Ok(())
}