//! Dynamic Updates Demo
//!
//! Shows how stateful boxy objects can update their content dynamically.
//! Demonstrates differential rendering with changing content.

use std::io::{self, Write};
use std::time::{Duration, Instant};
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
            content: format!("{} Dynamic Content Updates", glyph("bolt")),
            color: Color::Cyan,
        },
    );
    engine.add_root_child(title);

    // Dynamic status box
    let status_box_object = BoxyObject::new(boxy::BoxyConfig {
        text: "Status: STARTING\nCPU: --\nRAM: --".to_string(),
        title: Some("System Monitor".to_string()),
        colors: boxy::BoxColors {
            box_color: "blue".to_string(),
            text_color: "auto".to_string(),
            title_color: Some("white".to_string()),
            ..Default::default()
        },
        width: boxy::WidthConfig {
            fixed_width: Some(25),
            ..Default::default()
        },
        ..Default::default()
    });

    let status_box = RenderNode::new(
        Rect::new(5, 5, 25, 6),
        NodeContent::BoxyContent {
            boxy_object: status_box_object.clone()
        },
    );
    engine.add_root_child(status_box);

    // Dynamic terminal box
    let terminal_box_object = BoxyObject::new(boxy::BoxyConfig {
        text: "$ system-monitor".to_string(),
        title: Some("Terminal".to_string()),
        colors: boxy::BoxColors {
            box_color: "white".to_string(),
            text_color: "auto".to_string(),
            title_color: Some("blue".to_string()),
            ..Default::default()
        },
        width: boxy::WidthConfig {
            fixed_width: Some(40),
            ..Default::default()
        },
        ..Default::default()
    });

    let terminal_box = RenderNode::new(
        Rect::new(35, 5, 40, 8),
        NodeContent::BoxyContent {
            boxy_object: terminal_box_object.clone()
        },
    );
    engine.add_root_child(terminal_box);

    // Instructions
    let instructions = RenderNode::new(
        Rect::new(1, 18, 70, 1),
        NodeContent::Text {
            content: "Watch the boxes update with differential rendering! Press Ctrl+C to exit.".to_string(),
            color: Color::White,
        },
    );
    engine.add_root_child(instructions);

    // Initial render
    print!("{}", engine.init_terminal());
    engine.render_to_screen();
    print!("{}", engine.get_screen_updates());
    io::stdout().flush().unwrap();

    println!("\n{} Starting dynamic updates...", glyph("star"));

    let mut last_update = Instant::now();
    let mut update_counter = 0;

    loop {
        let now = Instant::now();

        // Update every 2 seconds
        if now.duration_since(last_update) >= Duration::from_secs(2) {
            update_counter += 1;

            match update_counter % 5 {
                1 => {
                    status_box_object.update_text("Status: ONLINE\nCPU: 42%\nRAM: 1.2GB".to_string());
                    status_box_object.update_box_color("green".to_string());
                    terminal_box_object.append_line("System started successfully");
                }
                2 => {
                    status_box_object.update_text("Status: BUSY\nCPU: 67%\nRAM: 1.8GB".to_string());
                    status_box_object.update_box_color("yellow".to_string());
                    terminal_box_object.append_line("Processing workload...");
                }
                3 => {
                    status_box_object.update_text("Status: HIGH\nCPU: 84%\nRAM: 2.1GB".to_string());
                    status_box_object.update_box_color("orange".to_string());
                    terminal_box_object.append_line("High resource usage detected");
                }
                4 => {
                    status_box_object.update_text("Status: ALERT!\nCPU: 91%\nRAM: 2.4GB".to_string());
                    status_box_object.update_box_color("red".to_string());
                    status_box_object.update_title(Some(format!("{} ALERT", glyph("fail"))));
                    terminal_box_object.append_line("WARNING: System overload!");
                }
                0 => {
                    status_box_object.update_text("Status: RECOVERED\nCPU: 35%\nRAM: 1.1GB".to_string());
                    status_box_object.update_box_color("blue".to_string());
                    status_box_object.update_title(Some("System Monitor".to_string()));
                    terminal_box_object.append_line("System stabilized");
                }
                _ => {}
            }

            last_update = now;
        }

        // Re-render with differential updates
        engine.render_to_screen();
        print!("{}", engine.get_screen_updates());
        io::stdout().flush().unwrap();

        std::thread::sleep(Duration::from_millis(100));
    }
}