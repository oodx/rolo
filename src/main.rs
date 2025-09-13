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

/// Create an input pane with boxy frame and input child
pub fn create_input_pane(bounds: Rect, pane_id: usize, title: &str, focus_manager: &FocusManager) -> RenderNode {
    let is_focused = focus_manager.is_active(pane_id);

    // Create the boxy frame
    let frame_color = if is_focused { "cyan" } else { "white" };
    let boxy_config = boxy::BoxyConfig {
        text: "".to_string(), // Empty content, input goes on top
        title: Some(format!("{} {}", if is_focused { glyph("target") } else { glyph("dot") }, title)),
        colors: boxy::BoxColors {
            box_color: frame_color.to_string(),
            text_color: "auto".to_string(),
            title_color: Some(if is_focused { "yellow" } else { "white" }.to_string()),
            ..Default::default()
        },
        width: boxy::WidthConfig {
            fixed_width: Some(bounds.width),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut container = RenderNode::new(bounds, NodeContent::BoxyContent {
        boxy_object: BoxyObject::new(boxy_config),
    });

    // Add input area as child (positioned inside the boxy frame)
    let input_bounds = Rect::new(2, 2, bounds.width.saturating_sub(4), 1);
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

    /// Clear all text content
    pub fn clear_text(&self) {
        self.config.borrow_mut().text.clear();
        self.mark_dirty();
    }

    /// Update the box color
    pub fn update_box_color(&self, color: String) {
        self.config.borrow_mut().colors.box_color = color;
        self.mark_dirty();
    }

    /// Get the rendered box (re-renders if dirty)
    pub fn render(&self) -> String {
        let dirty = *self.dirty.borrow();
        let has_cache = self.cached_render.borrow().is_some();

        if dirty || !has_cache {
            let config = self.config.borrow().clone();
            let rendered = render_box_to_string(config);
            *self.cached_render.borrow_mut() = Some(rendered.clone());
            *self.dirty.borrow_mut() = false;
            rendered
        } else {
            self.cached_render.borrow().as_ref().unwrap().clone()
        }
    }

    /// Force re-render on next render() call
    pub fn mark_dirty(&self) {
        *self.dirty.borrow_mut() = true;
        *self.cached_render.borrow_mut() = None;
    }

    /// Check if the object needs re-rendering
    pub fn is_dirty(&self) -> bool {
        *self.dirty.borrow()
    }
}

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
        // Always return prompt + buffer without embedded cursor
        format!("{}{}", self.prompt, self.buffer)
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
            dirty_lines: vec![true; height],
        }
    }

    /// Clear the entire screen buffer
    pub fn clear(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.buffer[row][col] = ' ';
                self.color_buffer[row][col].clear();
            }
            self.dirty_lines[row] = true;
        }
    }

    /// Write text at position with optional color
    pub fn write_at(&mut self, x: usize, y: usize, text: &str, color: Option<&str>) {
        if y >= self.height { return; }

        // Handle ANSI escape sequences in the text
        if text.contains('\x1B') {
            self.write_ansi_text(x, y, text);
            return;
        }

        let chars: Vec<char> = text.chars().collect();
        let mut col = x;
        for &ch in chars.iter() {
            if col >= self.width { break; }

            if self.buffer[y][col] != ch ||
               (color.is_some() && self.color_buffer[y][col] != color.unwrap_or("")) {
                self.buffer[y][col] = ch;
                self.color_buffer[y][col] = color.unwrap_or("").to_string();
                self.dirty_lines[y] = true;
            }

            // Advance column position by character width
            let char_width = ch.width().unwrap_or(1);

            // For double-width characters, fill the next column with a placeholder
            if char_width == 2 && col + 1 < self.width {
                self.buffer[y][col + 1] = ' '; // Placeholder for second half of wide char
                self.color_buffer[y][col + 1] = color.unwrap_or("").to_string();
            }

            col += char_width;
        }
    }

    /// Handle text that contains ANSI escape sequences (like boxy output)
    fn write_ansi_text(&mut self, x: usize, y: usize, text: &str) {
        if y >= self.height { return; }

        let mut col = x;
        let mut current_color = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1B' && chars.peek() == Some(&'[') {
                // Parse ANSI escape sequence
                chars.next(); // consume '['
                let mut ansi_code = String::from("\x1B[");

                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    ansi_code.push(next_ch);
                    if next_ch == 'm' { // End of color code
                        current_color = if ansi_code == "\x1B[0m" {
                            String::new() // Reset
                        } else {
                            ansi_code
                        };
                        break;
                    }
                }
            } else if col < self.width {
                // Regular character - handle Unicode width properly
                if self.buffer[y][col] != ch || self.color_buffer[y][col] != current_color {
                    self.buffer[y][col] = ch;
                    self.color_buffer[y][col] = current_color.clone();
                    self.dirty_lines[y] = true;
                }

                // Advance column position by character width
                let char_width = ch.width().unwrap_or(1);

                // For double-width characters, fill the next column with a placeholder
                if char_width == 2 && col + 1 < self.width {
                    self.buffer[y][col + 1] = ' '; // Placeholder for second half
                    self.color_buffer[y][col + 1] = current_color.clone();
                }

                col += char_width;
            }
        }
    }

    /// Render only the dirty (changed) lines to terminal
    pub fn render_differential(&mut self) -> String {
        let mut output = String::new();

        for row in 0..self.height {
            if self.dirty_lines[row] {
                // Position cursor at start of line
                output.push_str(&format!("\x1B[{};1H", row + 1));

                // Render the entire line with color grouping
                let mut current_color = String::new();
                for col in 0..self.width {
                    let cell_color = &self.color_buffer[row][col];

                    // Only change color if it's different
                    if cell_color != &current_color {
                        if !current_color.is_empty() {
                            output.push_str("\x1B[0m"); // Reset previous color
                        }
                        if !cell_color.is_empty() {
                            output.push_str(cell_color);
                        }
                        current_color = cell_color.clone();
                    }

                    output.push(self.buffer[row][col]);
                }

                // Reset color at end of line if needed
                if !current_color.is_empty() {
                    output.push_str("\x1B[0m");
                }

                self.dirty_lines[row] = false;
            }
        }

        output
    }

    /// Initialize terminal for screen rendering
    pub fn init_terminal() -> String {
        "\x1B[2J\x1B[?25l\x1B[1;1H".to_string() // Clear screen, hide cursor, go to top
    }

    /// Restore terminal when done
    pub fn restore_terminal() -> String {
        "\x1B[?25h".to_string() // Show cursor
    }
}

/// The main rendering engine that renders to a Screen
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

fn main() {
    // Enable RSB glyphs
    glyph_enable();

    // Get terminal size (simplified - in real implementation we'd use termios)
    let terminal_size = (80, 24);

    // Create render engine
    let mut engine = RenderEngine::new(terminal_size);

    // Create tmux-style multi-pane layout with focus management
    let mut focus_manager = FocusManager::new();
    focus_manager.add_pane(0); // Left pane
    focus_manager.add_pane(1); // Right pane
    focus_manager.add_pane(2); // Bottom pane

    // Title at top
    let title = RenderNode::new(
        Rect::new(1, 1, 78, 1),
        NodeContent::Text {
            content: format!("{} Rolo Terminal Multiplexer - Press Tab to switch panes!", glyph("star")),
            color: Color::Cyan,
        },
    );
    engine.add_root_child(title);

    // Create tmux-style panes layout

    // Left pane (focused initially)
    let left_pane = create_input_pane(
        Rect::new(1, 3, 38, 10),
        0,
        "bash",
        &focus_manager
    );
    engine.add_root_child(left_pane);

    // Right pane (unfocused)
    let right_pane = create_input_pane(
        Rect::new(41, 3, 38, 10),
        1,
        "vim",
        &focus_manager
    );
    engine.add_root_child(right_pane);

    // Bottom pane (unfocused)
    let bottom_pane = create_input_pane(
        Rect::new(1, 15, 78, 8),
        2,
        "htop",
        &focus_manager
    );
    engine.add_root_child(bottom_pane);

    // Initialize terminal and render first frame
    print!("{}", engine.init_terminal());
    engine.render_to_screen();
    print!("{}", engine.get_screen_updates());
    io::stdout().flush().unwrap();

    // TMUX-STYLE TERMINAL MULTIPLEXER!
    println!("\n{} Tmux-style multi-pane terminal! Tab switches focus, Ctrl+C to exit...", glyph("target"));

    let mut last_move = Instant::now();
    let mut last_focus_switch = Instant::now();

    loop {
        let now = Instant::now();

        // Move a random pane every 3 seconds (fun demo)
        if now.duration_since(last_move) >= Duration::from_secs(3) {
            println!("\n{} Moving random pane...", glyph("bolt"));
            engine.move_random_child(terminal_size);
            last_move = now;
        }

        // Auto-switch focus every 5 seconds for demo (normally user would press Tab)
        if now.duration_since(last_focus_switch) >= Duration::from_secs(5) {
            println!("\n{} Switching focus...", glyph("right"));
            focus_manager.next_pane();

            // TODO: In real implementation, we'd update the existing panes' focus state
            // For now, just re-create the layout (inefficient but works for demo)

            last_focus_switch = now;
        }


        // Re-render with differential updates
        engine.render_to_screen();
        print!("{}", engine.get_screen_updates());
        io::stdout().flush().unwrap();

        // Small sleep to prevent busy loop
        std::thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    print!("{}", engine.restore_terminal());
    print!("\x1B[2J\x1B[1;1H"); // Clear screen and go to top
    io::stdout().flush().unwrap();
}
