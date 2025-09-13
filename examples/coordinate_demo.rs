//! Coordinate System Demo
//!
//! Shows how to position colored blocks and text labels using the render tree.
//! Demonstrates basic positioning and coordinate system.

use std::io::{self, Write};
use rolo::*;
use rsb::visual::glyphs::{glyph, glyph_enable};

fn main() {
    // Enable RSB glyphs
    glyph_enable();

    let terminal_size = (80, 24);
    let mut engine = RenderEngine::new(terminal_size);

    // Title
    let title = RenderNode::new(
        Rect::new(1, 1, 50, 1),
        NodeContent::Text {
            content: format!("{} Coordinate System Demo", glyph("star")),
            color: Color::Cyan,
        },
    );
    engine.add_root_child(title);

    // Colored blocks at specific coordinates
    let block1 = RenderNode::new(
        Rect::new(10, 5, 1, 1),
        NodeContent::Block {
            character: '█',
            color: Color::Red2,
        },
    );
    engine.add_root_child(block1);

    let block2 = RenderNode::new(
        Rect::new(15, 8, 1, 1),
        NodeContent::Block {
            character: '●',
            color: Color::Purple,
        },
    );
    engine.add_root_child(block2);

    let block3 = RenderNode::new(
        Rect::new(25, 12, 1, 1),
        NodeContent::Block {
            character: '▲',
            color: Color::Orange,
        },
    );
    engine.add_root_child(block3);

    // Coordinate labels
    let coord1 = RenderNode::new(
        Rect::new(12, 5, 10, 1),
        NodeContent::Text {
            content: "(10,5)".to_string(),
            color: Color::Grey,
        },
    );
    engine.add_root_child(coord1);

    let coord2 = RenderNode::new(
        Rect::new(17, 8, 10, 1),
        NodeContent::Text {
            content: "(15,8)".to_string(),
            color: Color::Grey,
        },
    );
    engine.add_root_child(coord2);

    let coord3 = RenderNode::new(
        Rect::new(27, 12, 10, 1),
        NodeContent::Text {
            content: "(25,12)".to_string(),
            color: Color::Grey,
        },
    );
    engine.add_root_child(coord3);

    // Instructions
    let instructions = RenderNode::new(
        Rect::new(1, 20, 60, 1),
        NodeContent::Text {
            content: "Each block shows its (x,y) coordinate. Press Enter to exit.".to_string(),
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