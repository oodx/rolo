//! Rolo Terminal Layout Engine
//!
//! A render tree-based terminal multiplexer library inspired by yoga/ink but written in Rust.
//! Provides screen-based differential rendering with stateful components.

#![allow(unused_imports)]

use std::collections::HashMap;
use std::io::{self, Write};
use std::cell::RefCell;
use unicode_width::UnicodeWidthChar;
use boxy::{BoxyConfig, render_box_to_string};
use rsb::visual::glyphs::{glyph, glyph_enable};
use std::time::{Duration, Instant};
use rand::Rng;

/// Focus manager for handling multiple input panes
#[derive(Debug)]
pub struct FocusManager {
    pub pane_ids: Vec<usize>,
    pub active_index: usize,
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            pane_ids: Vec::new(),
            active_index: 0,
        }
    }

    pub fn add_pane(&mut self, pane_id: usize) {
        self.pane_ids.push(pane_id);
    }

    pub fn next_pane(&mut self) {
        if !self.pane_ids.is_empty() {
            self.active_index = (self.active_index + 1) % self.pane_ids.len();
        }
    }

    pub fn get_active_pane(&self) -> Option<usize> {
        self.pane_ids.get(self.active_index).copied()
    }

    pub fn is_active(&self, pane_id: usize) -> bool {
        self.get_active_pane() == Some(pane_id)
    }
}

/// Create an input pane with proper layering (container -> boxy background + input foreground)
pub fn create_input_pane(bounds: Rect, pane_id: usize, title: &str, focus_manager: &FocusManager) -> RenderNode {
    let is_focused = focus_manager.is_active(pane_id);

    // Capture dimensions before moving bounds
    let pane_width = bounds.width;
    let pane_height = bounds.height;

    // Create container that holds everything
    let mut container = RenderNode::new(bounds, NodeContent::Container);

    // Add boxy frame as BACKGROUND child (renders first/behind)
    let frame_color = if is_focused { "cyan" } else { "white" };
    let boxy_config = boxy::BoxyConfig {
        text: "".to_string(), // Empty content
        title: Some(format!("{} {}", if is_focused { glyph("target") } else { glyph("dot") }, title)),
        colors: boxy::BoxColors {
            box_color: frame_color.to_string(),
            text_color: "auto".to_string(),
            title_color: Some(if is_focused { "yellow" } else { "white" }.to_string()),
            ..Default::default()
        },
        width: boxy::WidthConfig {
            fixed_width: Some(pane_width),
            ..Default::default()
        },
        ..Default::default()
    };

    let background_frame = RenderNode::new(
        Rect::new(0, 0, pane_width, pane_height), // Full size at container origin
        NodeContent::BoxyContent {
            boxy_object: BoxyObject::new(boxy_config),
        }
    );
    container.add_child(background_frame);

    // Add input area as FOREGROUND child (renders on top)
    // Position it inside the boxy frame area
    let input_bounds = Rect::new(2, 2, pane_width.saturating_sub(4), 1);
    let mut input_state = TextInputState::new("$ ");
    input_state.focused = is_focused;

    let input_node = RenderNode::new(input_bounds, NodeContent::TextInput {
        input_state: RefCell::new(input_state),
        color: if is_focused { Color::Yellow } else { Color::White },
    });
    container.add_child(input_node);

    container
}

/// Stateful boxy object that can be updated and re-rendered
#[derive(Debug)]
pub struct BoxyObject {
    config: RefCell<BoxyConfig>,
    cached_render: RefCell<Option<String>>,
    dirty: RefCell<bool>,
}

impl Clone for BoxyObject {
    fn clone(&self) -> Self {
        Self {
            config: RefCell::new(self.config.borrow().clone()),
            cached_render: RefCell::new(self.cached_render.borrow().clone()),
            dirty: RefCell::new(*self.dirty.borrow()),
        }
    }
}

impl BoxyObject {
    pub fn new(config: BoxyConfig) -> Self {
        Self {
            config: RefCell::new(config),
            cached_render: RefCell::new(None),
            dirty: RefCell::new(true),
        }
    }

    /// Update the main text content
    pub fn update_text(&self, new_text: String) {
        self.config.borrow_mut().text = new_text;
        self.mark_dirty();
    }

    /// Update the title
    pub fn update_title(&self, new_title: Option<String>) {
        self.config.borrow_mut().title = new_title;
        self.mark_dirty();
    }

    /// Append a line to the existing text
    pub fn append_line(&self, line: &str) {
        let mut config = self.config.borrow_mut();
        if !config.text.is_empty() {
            config.text.push('\n');
        }
        config.text.push_str(line);
        drop(config);
        self.mark_dirty();
    }

    /// Update the box border color
    pub fn update_box_color(&self, new_color: String) {
        self.config.borrow_mut().colors.box_color = new_color;
        self.mark_dirty();
    }

    /// Mark this object as dirty (needs re-rendering)
    fn mark_dirty(&self) {
        *self.dirty.borrow_mut() = true;
        *self.cached_render.borrow_mut() = None;
    }

    /// Render this box, using cache if not dirty
    pub fn render(&self) -> String {
        let is_dirty = *self.dirty.borrow();

        if !is_dirty {
            if let Some(cached) = &*self.cached_render.borrow() {
                return cached.clone();
            }
        }

        // Re-render the box
        let config = self.config.borrow().clone();
        let rendered = render_box_to_string(config);

        // Cache the result
        *self.cached_render.borrow_mut() = Some(rendered.clone());
        *self.dirty.borrow_mut() = false;

        rendered
    }
}

/// ANSI color codes for terminal styling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Red2,
    Orange,
    Grey,
}

impl Color {
    pub fn to_ansi(&self) -> &'static str {
        match self {
            Color::Black => "\x1B[30m",
            Color::Red => "\x1B[31m",
            Color::Green => "\x1B[32m",
            Color::Yellow => "\x1B[33m",
            Color::Blue => "\x1B[34m",
            Color::Purple => "\x1B[35m",
            Color::Cyan => "\x1B[36m",
            Color::White => "\x1B[37m",
            Color::Red2 => "\x1B[38;5;9m",
            Color::Orange => "\x1B[38;5;214m",
            Color::Grey => "\x1B[38;5;244m",
        }
    }
}

/// Rectangle bounds for positioning nodes
#[derive(Debug, Clone)]
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

/// Text input state for interactive panes
#[derive(Debug, Clone)]
pub struct TextInputState {
    pub buffer: String,
    pub cursor_pos: usize,
    pub focused: bool,
    pub prompt: String,
}

impl TextInputState {
    pub fn new(prompt: &str) -> Self {
        Self {
            buffer: String::new(),
            cursor_pos: 0,
            focused: false,
            prompt: prompt.to_string(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.buffer.remove(self.cursor_pos);
        }
    }

    pub fn get_display_line(&self) -> String {
        if self.focused {
            // Show cursor with blinking effect (simplified)
            let mut display = format!("{}{}", self.prompt, self.buffer);
            if self.cursor_pos <= self.buffer.len() {
                display.insert(self.prompt.len() + self.cursor_pos, '█');
            }
            display
        } else {
            format!("{}{}", self.prompt, self.buffer)
        }
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
    BoxyContent {
        boxy_object: BoxyObject,
    },
    TextInput {
        input_state: RefCell<TextInputState>,
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

    /// Move this node to a new position
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.bounds.x = x;
        self.bounds.y = y;
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

/// Screen buffer that manages the entire terminal state
#[derive(Debug)]
pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec<char>>,
    pub color_buffer: Vec<Vec<String>>,
    pub dirty_lines: Vec<bool>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![' '; width]; height],
            color_buffer: vec![vec![String::new(); width]; height],
            dirty_lines: vec![false; height],
        }
    }

    /// Clear the screen buffer
    pub fn clear(&mut self) {
        for row in &mut self.buffer {
            for cell in row {
                *cell = ' ';
            }
        }
        for row in &mut self.color_buffer {
            for cell in row {
                cell.clear();
            }
        }
        // Mark all lines as dirty since we cleared
        for dirty in &mut self.dirty_lines {
            *dirty = true;
        }
    }

    /// Write text at specific position with optional color
    pub fn write_at(&mut self, x: usize, y: usize, text: &str, color: Option<&str>) {
        if y >= self.height { return; }

        // Parse ANSI sequences and extract plain text with colors
        let (plain_text, color_codes) = self.parse_ansi_text(text);

        let mut current_x = x;
        let mut color_index = 0;

        for ch in plain_text.chars() {
            if current_x >= self.width { break; }

            // Get color for this character position
            let char_color = if color_codes.len() > color_index {
                &color_codes[color_index]
            } else {
                color.unwrap_or("")
            };

            self.buffer[y][current_x] = ch;
            self.color_buffer[y][current_x] = char_color.to_string();
            self.dirty_lines[y] = true;

            // Handle Unicode width properly
            let char_width = ch.width().unwrap_or(1);
            current_x += char_width;
            color_index += 1;
        }
    }

    /// Parse ANSI escape sequences from text
    fn parse_ansi_text(&self, text: &str) -> (String, Vec<String>) {
        let mut plain_text = String::new();
        let mut color_codes = Vec::new();
        let mut chars = text.chars().peekable();
        let mut current_color = String::new();

        while let Some(ch) = chars.next() {
            if ch == '\x1B' && chars.peek() == Some(&'[') {
                // Start of ANSI sequence
                chars.next(); // consume '['
                let mut sequence = String::from("\x1B[");

                // Collect the ANSI sequence
                while let Some(seq_ch) = chars.next() {
                    sequence.push(seq_ch);
                    if seq_ch.is_ascii_alphabetic() {
                        break;
                    }
                }

                // This is a color code, save it for future characters
                if sequence.contains("38;5;") || sequence.contains("[3") {
                    current_color = sequence;
                } else if sequence == "\x1B[0m" {
                    current_color.clear(); // Reset
                }
            } else {
                // Regular character
                plain_text.push(ch);
                color_codes.push(current_color.clone());
            }
        }

        (plain_text, color_codes)
    }

    /// Get differential screen updates (only changed lines)
    pub fn render_differential(&mut self) -> String {
        let mut output = String::new();

        for (y, is_dirty) in self.dirty_lines.iter().enumerate() {
            if *is_dirty {
                // Position cursor at start of line
                output.push_str(&format!("\x1B[{};1H", y + 1));

                // Build line with grouped colors for efficiency
                let mut line_output = String::new();
                let mut current_color = String::new();

                for (x, &ch) in self.buffer[y].iter().enumerate() {
                    let char_color = &self.color_buffer[y][x];

                    // Only add color code if it changed
                    if char_color != &current_color {
                        if !char_color.is_empty() {
                            line_output.push_str(char_color);
                        } else if !current_color.is_empty() {
                            line_output.push_str("\x1B[0m"); // Reset color
                        }
                        current_color = char_color.clone();
                    }

                    line_output.push(ch);
                }

                // Reset color at end of line if we had any color
                if !current_color.is_empty() {
                    line_output.push_str("\x1B[0m");
                }

                output.push_str(&line_output);
            }
        }

        // Mark all lines as clean
        for dirty in &mut self.dirty_lines {
            *dirty = false;
        }

        output
    }

    /// Initialize terminal for first render
    pub fn init_terminal() -> String {
        "\x1B[2J\x1B[?25l\x1B[1;1H".to_string() // Clear screen, hide cursor, go to top
    }

    /// Restore terminal when done
    pub fn restore_terminal() -> String {
        "\x1B[?25h".to_string() // Show cursor
    }
}

/// Main render engine that manages the render tree and screen buffer
#[derive(Debug)]
pub struct RenderEngine {
    pub root: RenderNode,
    pub screen: Screen,
}

impl RenderEngine {
    pub fn new(terminal_size: (usize, usize)) -> Self {
        let root = RenderNode::new(
            Rect::new(0, 0, terminal_size.0, terminal_size.1),
            NodeContent::Container,
        );

        let screen = Screen::new(terminal_size.0, terminal_size.1);

        Self { root, screen }
    }

    /// Render the entire tree to the Screen buffer
    pub fn render_to_screen(&mut self) {
        // Clear the screen buffer first
        self.screen.clear();

        // Clone the root node to avoid borrowing issues
        let root_clone = self.root.clone();

        // Render the tree into the screen buffer
        self.render_node_to_screen(&root_clone, (0, 0));
    }

    /// Get differential screen updates (only changed lines)
    pub fn get_screen_updates(&mut self) -> String {
        self.screen.render_differential()
    }

    /// Initialize terminal for first render
    pub fn init_terminal(&self) -> String {
        Screen::init_terminal()
    }

    /// Restore terminal when done
    pub fn restore_terminal(&self) -> String {
        Screen::restore_terminal()
    }

    /// Recursively render a node and its children to screen buffer
    fn render_node_to_screen(&mut self, node: &RenderNode, parent_pos: (usize, usize)) {
        let abs_bounds = node.absolute_bounds(parent_pos);

        // Render this node's content to screen buffer
        match &node.content {
            NodeContent::Text { content, color } => {
                // Write text to screen buffer with color
                self.screen.write_at(abs_bounds.x, abs_bounds.y, content, Some(color.to_ansi()));
            }
            NodeContent::Block { character, color } => {
                // Write single character to screen buffer with color
                let char_str = character.to_string();
                self.screen.write_at(abs_bounds.x, abs_bounds.y, &char_str, Some(color.to_ansi()));
            }
            NodeContent::BoxyContent { boxy_object } => {
                // Get the boxy box rendered as a string (will re-render if dirty)
                let box_string = boxy_object.render();
                let box_lines: Vec<&str> = box_string.lines().collect();

                // Write each line of the box to screen buffer
                for (line_idx, line) in box_lines.iter().enumerate() {
                    let y_pos = abs_bounds.y + line_idx;
                    if y_pos < self.screen.height {
                        // Parse ANSI colors from the line if any and write to screen
                        self.screen.write_at(abs_bounds.x, y_pos, line, None);
                    }
                }
            }
            NodeContent::TextInput { input_state, color } => {
                // Get the current display line with cursor
                let display_line = input_state.borrow().get_display_line();
                self.screen.write_at(abs_bounds.x, abs_bounds.y, &display_line, Some(color.to_ansi()));
            }
            NodeContent::Container => {
                // Container nodes don't render themselves, just their children
            }
        }

        // Render all children
        for child in &node.children {
            self.render_node_to_screen(child, (abs_bounds.x, abs_bounds.y));
        }
    }

    /// Add a node to the root
    pub fn add_root_child(&mut self, child: RenderNode) {
        self.root.add_child(child);
    }

    /// Get mutable reference to child by index
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut RenderNode> {
        self.root.children.get_mut(index)
    }

    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.root.children.len()
    }

    /// Move a random child to a new random position
    pub fn move_random_child(&mut self, terminal_size: (usize, usize)) {
        let child_count = self.child_count();
        if child_count == 0 { return; }

        let mut rng = rand::thread_rng();
        let child_index = rng.gen_range(0..child_count);

        if let Some(child) = self.get_child_mut(child_index) {
            // Get constraints based on box size to keep it on screen
            let max_x = terminal_size.0.saturating_sub(child.bounds.width + 1);
            let max_y = terminal_size.1.saturating_sub(child.bounds.height + 1);

            if max_x > 0 && max_y > 0 {
                let new_x = rng.gen_range(1..=max_x);
                let new_y = rng.gen_range(1..=max_y);
                child.move_to(new_x, new_y);
            }
        }
    }
}