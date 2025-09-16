//! Visual demonstration UAT - Shows actual rolo output
//!
//! These tests demonstrate rolo's capabilities with real formatting examples

use rololib::prelude::*;

#[test]
fn uat_columns_visual_demo() {
    println!("\n🎯 ROLO VISUAL DEMONSTRATION: COLUMNS MODE");
    println!("=====================================\n");

    let sample_data = "apple\nbanana\ncherry\ndate\nfig\ngrape\nhoneydew\nkiwi\nlemon\nmango\norange\npear";

    println!("📝 INPUT DATA:");
    println!("{}\n", sample_data);

    let config = LayoutConfig { width: 80, gap: 4, padding: 1 };

    println!("🔧 CONFIGURATION: {} columns, {}px width, {}px gap\n", 3, config.width, config.gap);

    println!("📊 OUTPUT (3 columns):");
    println!("─────────────────────");
    match format_columns_with_config(sample_data, 3, &config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n📊 OUTPUT (2 columns):");
    println!("─────────────────────");
    match format_columns_with_config(sample_data, 2, &config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }
}

#[test]
fn uat_table_visual_demo() {
    println!("\n🎯 ROLO VISUAL DEMONSTRATION: TABLE MODE");
    println!("===================================\n");

    let csv_data = "Product,Price,Stock\nApple,1.50,100\nBanana,0.75,250\nCherry,3.00,50\nDate,2.25,75";

    println!("📝 INPUT DATA (CSV):");
    println!("{}\n", csv_data);

    println!("📊 OUTPUT (formatted table):");
    println!("───────────────────────────");
    match format_table_with_config(csv_data, ",", 80) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n📝 INPUT DATA (pipe-delimited):");
    let pipe_data = "Name|Department|Salary\nJohn|Engineering|75000\nSarah|Marketing|65000\nMike|Sales|60000";
    println!("{}\n", pipe_data);

    println!("📊 OUTPUT (pipe-delimited table):");
    println!("─────────────────────────────────");
    match format_table_with_config(pipe_data, "|", 80) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }
}

#[test]
fn uat_list_visual_demo() {
    println!("\n🎯 ROLO VISUAL DEMONSTRATION: LIST MODE");
    println!("================================\n");

    let items = "First item\nSecond item\nThird item\nFourth item\nFifth item";

    println!("📝 INPUT DATA:");
    println!("{}\n", items);

    let config = ListConfig {
        width: 80,
        line_numbers: false,
        list_style: Some("bullets".to_string()),
        alignment: ListAlignment::Left,
    };

    println!("📊 OUTPUT (bulleted list):");
    println!("─────────────────────────");
    match format_list_with_config(items, &config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }

    let numbered_config = ListConfig {
        width: 80,
        line_numbers: true,
        list_style: Some("numbers".to_string()),
        alignment: ListAlignment::Left,
    };

    println!("\n📊 OUTPUT (numbered list):");
    println!("─────────────────────────");
    match format_list_with_config(items, &numbered_config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }
}

#[test]
fn uat_separator_visual_demo() {
    println!("\n🎯 ROLO VISUAL DEMONSTRATION: SEPARATOR PROCESSING");
    println!("============================================\n");

    let comma_data = "red,green,blue,yellow,orange,purple,cyan,magenta";

    println!("📝 INPUT DATA (comma-separated):");
    println!("{}\n", comma_data);

    let config = LayoutConfig { width: 60, gap: 2, padding: 1 };

    println!("📊 OUTPUT (3 columns with comma separator):");
    println!("──────────────────────────────────────────");
    match format_columns_with_delimiter(comma_data, 3, &config, Some(",")) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n📊 OUTPUT (4 columns with comma separator):");
    println!("──────────────────────────────────────────");
    match format_columns_with_delimiter(comma_data, 4, &config, Some(",")) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }
}

#[test]
fn uat_width_adaptation_demo() {
    println!("\n🎯 ROLO VISUAL DEMONSTRATION: WIDTH ADAPTATION");
    println!("=========================================\n");

    let text = "Lorem\nipsum\ndolor\nsit\namet\nconsectetur\nadipiscing\nelit\nsed\ndo\neiusmod\ntempor";

    println!("📝 INPUT DATA:");
    println!("{}\n", text);

    println!("📊 OUTPUT (narrow: 40px width):");
    println!("─────────────────────────────────");
    let narrow_config = LayoutConfig { width: 40, gap: 2, padding: 1 };
    match format_columns_with_config(text, 2, &narrow_config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n📊 OUTPUT (wide: 100px width):");
    println!("──────────────────────────────");
    let wide_config = LayoutConfig { width: 100, gap: 4, padding: 1 };
    match format_columns_with_config(text, 4, &wide_config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("❌ Error: {}", e),
    }
}