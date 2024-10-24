// src/menu/dropdown.rs

use crate::menu::menu_item::MenuItem;
use console_engine::pixel;
use console_engine::ConsoleEngine;

pub struct Dropdown {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub is_open: bool,
    pub selected: bool,
}

impl MenuItem for Dropdown {
    fn draw(&self, engine: &mut ConsoleEngine) {
        // Draw the dropdown header
        let bg_char = if self.selected {
            pixel::pxl('█')
        } else {
            pixel::pxl(' ')
        };
        engine.rect(self.x, self.y, self.x + self.width, self.y + 2, bg_char);

        // Display the selected option
        engine.print(self.x + 1, self.y + 1, &self.options[self.selected_index]);

        // If open, display all options
        if self.is_open {
            for (i, option) in self.options.iter().enumerate() {
                let option_y = self.y + 3 + i as i32;
                engine.print(self.x + 1, option_y, option);
            }
        }
    }

    fn handle_input(&mut self, engine: &mut ConsoleEngine) -> bool {
        if engine.is_key_pressed(console_engine::KeyCode::Enter) {
            self.is_open = !self.is_open;
            return true;
        } else if self.is_open {
            if engine.is_key_pressed(console_engine::KeyCode::Up) {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                return true;
            } else if engine.is_key_pressed(console_engine::KeyCode::Down)
                && self.selected_index < self.options.len() - 1
            {
                self.selected_index += 1;
                return true;
            }

            if engine.is_key_pressed(console_engine::KeyCode::Enter) {
                self.selected = true;
                return true;
            }
        }
        false
    }

    fn is_selectable(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Clone for Dropdown {
    fn clone(&self) -> Self {
        Dropdown {
            x: self.x,
            y: self.y,
            width: self.width,
            options: self.options.clone(),
            selected_index: self.selected_index,
            is_open: self.is_open,
            selected: self.selected,
        }
    }
}
