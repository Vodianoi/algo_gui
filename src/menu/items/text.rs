// src/menu/text.rs

use crate::menu::menu_item::MenuItem;
use console_engine::ConsoleEngine;

use text_to_ascii_art::*;

pub struct Text {
    pub x: i32,
    pub y: i32,
    pub content: String,
}

impl MenuItem for Text {
    fn draw(&self, engine: &mut console_engine::ConsoleEngine) {
        let screeen_width = engine.get_width();
        // Check if the screen width is large enough to draw ASCII art (Menu should take 1/4 of the screen)
        if screeen_width > 500 {
            self.draw_ascii(engine);
        } else {
            self.draw_standard(engine);
        }
    }

    fn get_width(&self) -> i32 {
        self.content.len() as i32
    }

    fn get_height(&self) -> i32 {
        1
    }

    fn draw_standard(&self, engine: &mut ConsoleEngine) {
        engine.print(self.x, self.y, &self.content);
    }

    fn draw_ascii(&self, engine: &mut console_engine::ConsoleEngine) {
        let max_width = engine.get_width() as usize / 40; // Adjust for ASCII art scaling
        let words: Vec<&str> = self.content.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > max_width {
                lines.push(current_line);
                current_line = String::new();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let ascii_lines: Vec<String> = lines
            .iter()
            .flat_map(|line| {
                let art = to_art(line.clone(), "", 1, 1, 1).unwrap();
                art.split("\n").map(|line| line.to_string()).collect::<Vec<_>>()
            })
            .collect();

        let width = ascii_lines.iter().map(|line| line.len()).max().unwrap_or(0) as i32;
        // let x = self.x + self.content.len() as i32 / 2 - width / 2;
        let mut y = self.y + 1;

        for line in ascii_lines {
            engine.print(self.x / 3, y, &line);
            y += 1;
        }
    }

    fn handle_input(&mut self, engine: &mut ConsoleEngine) -> bool {
        let _ = engine;
        // Non-interactive
        false
    }

    fn is_mouse_over(&self, x: i32, y: i32) -> bool {
        let _ = y;
        let _ = x;
        false
    }

    fn is_selectable(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            x: 0,
            y: 0,
            content,
        }
    }
}
