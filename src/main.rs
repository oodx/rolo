use std::collections::HashMap;
use std::io::{self, Write};

/// Color system based on your gorgeous 256-color palette!
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red2,    // 197
    Red,     // 31
    Orange,  // 214
    Yellow,  // 33
    Green,   // 32
    Blue,    // 36
    Blue2,   // 39
    Cyan,    // 14
    Magenta, // 35
    Purple,  // 213
    Purple2, // 141
    White,   // 248
    White2,  // 15
    Grey,    // 244
    Grey2,   // 240
    Reset,
}

impl Color {
    fn to_ansi(&self) -> &'static str {
        match self {
            Color::Red2 => "\x1B[38;5;197m",
            Color::Red => "\x1B[31m",
            Color::Orange => "\x1B[38;5;214m",
            Color::Yellow => "\x1B[33m",
            Color::Green => "\x1B[32m",
            Color::Blue => "\x1B[36m",
            Color::Blue2 => "\x1B[38;5;39m",
            Color::Cyan => "\x1B[38;5;14m",
            Color::Magenta => "\x1B[35m",
            Color::Purple => "\x1B[38;5;213m",
            Color::Purple2 => "\x1B[38;5;141m",
            Color::White => "\x1B[38;5;248m",
            Color::White2 => "\x1B[38;5;15m",
            Color::Grey => "\x1B[38;5;244m",
            Color::Grey2 => "\x1B[38;5;240m",
            Color::Reset => "\x1B[0m",
        }
    }
}

/// Position and dimensions
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}

/// Different types of renderable content
#[derive(Debug, Clone)]
pub enum NodeContent {
    Text {
        content: String,
        color: Color,
    },
    Block {
        character: char,
        color: Color,
    },
    Container,  // Just holds children
}

/// A node in the render tree
#[derive(Debug, Clone)]
pub struct RenderNode {
    pub bounds: Rect,
    pub content: NodeContent,
    pub children: Vec<RenderNode>,
}

impl RenderNode {
    pub fn new(bounds: Rect, content: NodeContent) -> Self {
        Self {
            bounds,
            content,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: RenderNode) {
        self.children.push(child);
    }

    /// Calculate absolute position from parent position
    pub fn absolute_bounds(&self, parent_pos: (usize, usize)) -> Rect {
        Rect::new(
            parent_pos.0 + self.bounds.x,
            parent_pos.1 + self.bounds.y,
            self.bounds.width,
            self.bounds.height,
        )
    }
}

/// The main rendering engine
pub struct RenderEngine {
    pub root: RenderNode,
    pub terminal_size: (usize, usize),
}

impl RenderEngine {
    pub fn new(terminal_size: (usize, usize)) -> Self {
        let root = RenderNode::new(
            Rect::new(0, 0, terminal_size.0, terminal_size.1),
            NodeContent::Container,
        );

        Self { root, terminal_size }
    }

    /// Render the entire tree to ANSI output
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Clear screen and hide cursor
        output.push_str("\x1B[2J\x1B[?25l");

        // Render the tree starting from root
        self.render_node(&self.root, (0, 0), &mut output);

        output
    }

    /// Recursively render a node and its children
    fn render_node(&self, node: &RenderNode, parent_pos: (usize, usize), output: &mut String) {
        let abs_bounds = node.absolute_bounds(parent_pos);

        // Render this node's content
        match &node.content {
            NodeContent::Text { content, color } => {
                // Position cursor and render text
                output.push_str(&format!("\x1B[{};{}H", abs_bounds.y + 1, abs_bounds.x + 1));
                output.push_str(color.to_ansi());
                output.push_str(content);
                output.push_str(Color::Reset.to_ansi());
            }
            NodeContent::Block { character, color } => {
                // Position cursor and render block character
                output.push_str(&format!("\x1B[{};{}H", abs_bounds.y + 1, abs_bounds.x + 1));
                output.push_str(color.to_ansi());
                output.push(*character);
                output.push_str(Color::Reset.to_ansi());
            }
            NodeContent::Container => {
                // Container nodes don't render themselves, just their children
            }
        }

        // Render all children
        for child in &node.children {
            self.render_node(child, (abs_bounds.x, abs_bounds.y), output);
        }
    }

    /// Add a node to the root
    pub fn add_root_child(&mut self, child: RenderNode) {
        self.root.add_child(child);
    }
}

fn main() {
    // Get terminal size (simplified - in real implementation we'd use termios)
    let terminal_size = (80, 24);

    // Create render engine
    let mut engine = RenderEngine::new(terminal_size);

    // Create some test nodes using the patterns from our bash demos

    // Title at top
    let title = RenderNode::new(
        Rect::new(1, 1, 50, 1),
        NodeContent::Text {
            content: "🎨 Rolo Tree Rendering Demo!".to_string(),
            color: Color::Cyan,
        },
    );
    engine.add_root_child(title);

    // Some colored blocks like our coordinate demo
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
            content: "Tree-based rendering with your beautiful colors! Press Ctrl+C to exit.".to_string(),
            color: Color::White,
        },
    );
    engine.add_root_child(instructions);

    // Render and display
    let output = engine.render();
    print!("{}", output);
    io::stdout().flush().unwrap();

    // Keep it running so we can see the output
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Cleanup
    print!("\x1B[?25h\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
