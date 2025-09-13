//! Boxy Integration Demo
//!
//! Demonstrates how to integrate boxy boxes into the render tree.
//! Shows stateful boxy objects with dynamic content updates.

use std::io::{self, Write};
use std::time::Duration;
use rolo::*;
use rsb::visual::glyphs::{glyph, glyph_enable};

fn main() {
    // Enable RSB glyphs
    glyph_enable();

    let terminal_size = (80, 24);
    let mut engine = RenderEngine::new(terminal_size);

    // Title
    let title = RenderNode::new(
        Rect::new(1, 1, 60, 1),
        NodeContent::Text {
            content: format!("{} Boxy Integration Demo", glyph("gear")),
            color: Color::Cyan,
        },
    );
    engine.add_root_child(title);

    // Main boxy demo box
    let boxy_config = boxy::BoxyConfig {
        text: format!("{} Rolo + Boxy Integration!\nTree-based rendering with real box lines!", glyph("star")),
        title: Some("Boxy Demo".to_string()),
        colors: boxy::BoxColors {
            box_color: "purple".to_string(),
            text_color: "auto".to_string(),
            title_color: Some("cyan".to_string()),
            ..Default::default()
        },
        width: boxy::WidthConfig {
            fixed_width: Some(40),
            ..Default::default()
        },
        ..Default::default()
    };

    let boxy_demo = RenderNode::new(
        Rect::new(20, 5, 40, 6),
        NodeContent::BoxyContent {
            boxy_object: BoxyObject::new(boxy_config)
        },
    );
    engine.add_root_child(boxy_demo);

    // Features showcase box
    let info_box = RenderNode::new(
        Rect::new(10, 13, 60, 5),
        NodeContent::BoxyContent {
            boxy_object: BoxyObject::new(boxy::BoxyConfig {
                text: format!("{} Screen-based rendering\n{} Stateful box objects\n{} RSB glyph integration\n{} Differential updates",
                    glyph("bolt"), glyph("gear"), glyph("star"), glyph("pass")),
                title: Some("Features".to_string()),
                colors: boxy::BoxColors {
                    box_color: "cyan".to_string(),
                    text_color: "auto".to_string(),
                    title_color: Some("yellow".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            })
        },
    );
    engine.add_root_child(info_box);

    // Instructions
    let instructions = RenderNode::new(
        Rect::new(1, 20, 60, 1),
        NodeContent::Text {
            content: "Boxy boxes fully integrated into render tree! Press Enter to exit.".to_string(),
            color: Color::White,
        },
    );
    engine.add_root_child(instructions);

    // Render
    print!("{}", engine.init_terminal());
    engine.render_to_screen();
    print!("{}", engine.get_screen_updates());
    io::stdout().flush().unwrap();

    // Wait for input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Cleanup
    print!("{}", engine.restore_terminal());
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}