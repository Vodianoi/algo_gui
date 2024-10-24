use console_engine::Color;
use console_engine::ConsoleEngine;
/// Button that can be clicked, using console_engine.
/// Can also be used as a menu item.
use console_engine::KeyCode;

use text_to_ascii_art::Alignment;
use text_to_ascii_art::{align, fonts, to_art};

use termsize;

use console_engine::pixel;

// src/menu/button.rs

use crate::algorithms::maze_generation::DynClone;
use crate::menu::menu_item::MenuItem;

pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub label: String,
    pub selected: bool,
}

impl MenuItem for Button {
    fn draw(&self, engine: &mut ConsoleEngine) {
        // Draw the button background
        let bg_char = if self.selected {
            pixel::pxl('█')
        } else {
            pixel::pxl(' ')
        };
        engine.rect(
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height,
            bg_char,
        );

        // Draw the label
        let label_x = self.x + (self.width - self.label.len() as i32) / 2;
        let label_y = self.y + self.height / 2;
        engine.print(label_x, label_y, &self.label);
    }

    fn handle_input(&mut self, engine: &mut ConsoleEngine) -> bool {
        // Button-specific input handling if necessary
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
