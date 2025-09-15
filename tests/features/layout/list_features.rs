//! Layout list mode feature tests - comprehensive coverage per RSB MODULE_SPEC

use rololib::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_LIST: &str = "apple\nbanana\ncherry\ndate\nfig\ngrape\nhoneydew\nkiwi\nlemon\nmango";

    #[test]
    fn test_basic_list_formatting() {
        let config = ListConfig {
            width: 80,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config(SAMPLE_LIST, &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], "apple");
        assert_eq!(lines[1], "banana");
        assert_eq!(lines[9], "mango");
    }

    #[test]
    fn test_list_with_line_numbers() {
        let config = ListConfig {
            width: 80,
            line_numbers: true,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config(SAMPLE_LIST, &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 10);
        assert!(lines[0].starts_with(" 1. "));
        assert!(lines[0].contains("apple"));
        assert!(lines[9].starts_with("10. "));
        assert!(lines[9].contains("mango"));
    }

    #[test]
    fn test_list_alignment_left() {
        let config = ListConfig {
            width: 20,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("short\nlongerword\nhi", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Left aligned should not add leading spaces for short content
        assert_eq!(lines[0], "short");
        assert_eq!(lines[1], "longerword");
        assert_eq!(lines[2], "hi");
    }

    #[test]
    fn test_list_alignment_right() {
        let config = ListConfig {
            width: 20,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Right,
        };

        let result = format_list_with_config("short\nlongerword\nhi", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Right aligned should add leading spaces for short content
        assert!(lines[0].ends_with("short"));
        assert!(lines[0].starts_with(" "));
        assert!(lines[2].ends_with("hi"));
        assert!(lines[2].starts_with(" "));
    }

    #[test]
    fn test_list_alignment_center() {
        let config = ListConfig {
            width: 20,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Center,
        };

        let result = format_list_with_config("short\nhi", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Center aligned should have roughly equal spaces on both sides
        let short_line = lines[0];
        let hi_line = lines[1];

        assert!(short_line.contains("short"));
        assert!(hi_line.contains("hi"));
        // Both should have leading spaces for centering
        assert!(short_line.starts_with(" "));
        assert!(hi_line.starts_with(" "));
    }

    #[test]
    fn test_list_width_constraints() {
        let config = ListConfig {
            width: 10,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("verylongwordthatwillbetruncated\nshort", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Long content should be truncated with ellipsis
        assert!(lines[0].contains("..."));
        assert!(lines[0].len() <= 10);
        assert_eq!(lines[1], "short");
    }

    #[test]
    fn test_list_with_line_numbers_and_alignment() {
        let config = ListConfig {
            width: 30,
            line_numbers: true,
            list_style: None,
            alignment: ListAlignment::Center,
        };

        let result = format_list_with_config("apple\nbanana\ncherry", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 3);
        // Each line should start with line number
        assert!(lines[0].starts_with("1. "));
        assert!(lines[1].starts_with("2. "));
        assert!(lines[2].starts_with("3. "));

        // Content should be centered within available width
        assert!(lines[0].contains("apple"));
        assert!(lines[1].contains("banana"));
        assert!(lines[2].contains("cherry"));
    }

    #[test]
    fn test_empty_input() {
        let config = ListConfig::default();
        let result = format_list_with_config("", &config).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_whitespace_only_input() {
        let config = ListConfig::default();
        let result = format_list_with_config("   \n\t\n  ", &config).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_mixed_content_with_empty_lines() {
        let config = ListConfig::default();
        let result = format_list_with_config("apple\n\nbanana\n\ncherry", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Empty lines should be filtered out
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "apple");
        assert_eq!(lines[1], "banana");
        assert_eq!(lines[2], "cherry");
    }

    #[test]
    fn test_very_narrow_width() {
        let config = ListConfig {
            width: 5,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("testing\nhi", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        // Should handle very narrow widths gracefully
        assert!(lines[0].contains("..."));
        assert!(lines[0].len() <= 5);
        assert_eq!(lines[1], "hi");
    }

    #[test]
    fn test_line_numbers_with_many_items() {
        let mut items = Vec::new();
        for i in 1..=100 {
            items.push(format!("item{}", i));
        }
        let input = items.join("\n");

        let config = ListConfig {
            width: 80,
            line_numbers: true,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config(&input, &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 100);

        // First few items should have single-digit line numbers with padding
        assert!(lines[0].starts_with("  1. "));
        assert!(lines[8].starts_with("  9. "));

        // Later items should have multi-digit line numbers with proper spacing
        assert!(lines[9].starts_with(" 10. "));
        assert!(lines[98].starts_with(" 99. "));
        assert!(lines[99].starts_with("100. "));
    }

    #[test]
    fn test_truncation_with_different_alignments() {
        let config_left = ListConfig {
            width: 8,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Left,
        };

        let config_right = ListConfig {
            width: 8,
            line_numbers: false,
            list_style: None,
            alignment: ListAlignment::Right,
        };

        let long_text = "verylongtext";

        let result_left = format_list_with_config(long_text, &config_left).unwrap();
        let result_right = format_list_with_config(long_text, &config_right).unwrap();

        // Both should be truncated but right alignment should handle it differently
        assert!(result_left.contains("..."));
        assert!(result_right.contains("..."));
        assert!(result_left.len() <= 8);
        assert!(result_right.len() <= 8);
    }

    #[test]
    fn test_list_style_bullets() {
        let config = ListConfig {
            width: 80,
            line_numbers: false,
            list_style: Some("bullets".to_string()),
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("apple\nbanana\ncherry", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("• "));
        assert!(lines[1].starts_with("• "));
        assert!(lines[2].starts_with("• "));
        assert!(lines[0].contains("apple"));
        assert!(lines[1].contains("banana"));
        assert!(lines[2].contains("cherry"));
    }

    #[test]
    fn test_list_style_stars() {
        let config = ListConfig {
            width: 80,
            line_numbers: false,
            list_style: Some("stars".to_string()),
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("apple\nbanana", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("* "));
        assert!(lines[1].starts_with("* "));
        assert!(lines[0].contains("apple"));
        assert!(lines[1].contains("banana"));
    }

    #[test]
    fn test_list_style_numbers() {
        let config = ListConfig {
            width: 80,
            line_numbers: false,
            list_style: Some("numbers".to_string()),
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("apple\nbanana\ncherry", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("1. "));
        assert!(lines[1].starts_with("2. "));
        assert!(lines[2].starts_with("3. "));
        assert!(lines[0].contains("apple"));
        assert!(lines[1].contains("banana"));
        assert!(lines[2].contains("cherry"));
    }

    #[test]
    fn test_list_style_dash() {
        let config = ListConfig {
            width: 80,
            line_numbers: false,
            list_style: Some("dash".to_string()),
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("apple\nbanana", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("- "));
        assert!(lines[1].starts_with("- "));
        assert!(lines[0].contains("apple"));
        assert!(lines[1].contains("banana"));
    }

    #[test]
    fn test_list_style_dots() {
        let config = ListConfig {
            width: 80,
            line_numbers: false,
            list_style: Some("dots".to_string()),
            alignment: ListAlignment::Left,
        };

        let result = format_list_with_config("apple\nbanana", &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("· "));
        assert!(lines[1].starts_with("· "));
        assert!(lines[0].contains("apple"));
        assert!(lines[1].contains("banana"));
    }
}