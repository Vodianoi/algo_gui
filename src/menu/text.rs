// src/menu/text.rs

use crate::{algorithms::maze_generation::DynClone, menu::menu_item::MenuItem};
use console_engine::ConsoleEngine;

pub struct Text {
    pub x: i32,
    pub y: i32,
    pub content: String,
}

impl MenuItem for Text {
    fn draw(&self, engine: &mut ConsoleEngine) {
        engine.print(self.x, self.y, &self.content);
    }

    fn handle_input(&mut self, engine: &mut ConsoleEngine) -> bool {
        // Non-interactive
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
