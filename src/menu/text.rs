// src/menu/text.rs

use crate::{algorithms::maze_generation::DynClone, menu::menu_item::MenuItem};
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
    fn draw_standard(&self, engine: &mut ConsoleEngine) {
        engine.print(self.x, self.y, &self.content);
    }

    fn draw_ascii(&self, engine: &mut console_engine::ConsoleEngine) {
        let art = to_art(self.content.clone(), "", 1, 1, 1).unwrap();
        let lines = art.split("\n").map(|line| line.to_string());
        let width = lines.clone().map(|line| line.len()).max().unwrap() as i32;
        let height = lines.clone().count() as i32;
        let x = self.x + self.content.len() as i32 / 2 - width / 2;
        let y = self.y + 1;
        for (i, line) in lines.enumerate() {
            engine.print(x, y + i as i32, &line);
        }
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

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            x: 0,
            y: 0,
            content,
        }
    }
}
