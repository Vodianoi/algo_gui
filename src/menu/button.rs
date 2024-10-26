use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

use text_to_ascii_art::Alignment;
use text_to_ascii_art::{align, fonts, to_art};

use super::menu_item::MenuItem;

pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub label: String,
    pub selected: bool,
}

impl Button {
    fn draw_border(&self, engine: &mut ConsoleEngine) {
        // Define border characters for better visual distinction
        let border_char = if self.selected { '▓' } else { '░' };

        // Draw top and bottom borders
        for i in self.x..self.x + self.width + 1 {
            engine.set_pxl(i, self.y, pixel::pxl_bg(border_char, Color::DarkGrey));
            engine.set_pxl(
                i,
                self.y + self.height,
                pixel::pxl_bg(border_char, Color::DarkGrey),
            );
        }

        // Draw left and right borders
        for j in self.y..self.y + self.height + 1 {
            engine.set_pxl(self.x, j, pixel::pxl_bg(border_char, Color::DarkGrey));
            engine.set_pxl(
                self.x + self.width,
                j,
                pixel::pxl_bg(border_char, Color::DarkGrey),
            );
        }
    }
}

impl MenuItem for Button {
    fn draw(&self, engine: &mut ConsoleEngine) {
        let screeen_width = engine.get_width();
        // Check if the screen width is large enough to draw ASCII art (Menu should take 1/4 of the screen)
        if screeen_width > 500 {
            self.draw_ascii(engine);
        } else {
            self.draw_standard(engine);
        }
    }

    fn draw_ascii(&self, engine: &mut ConsoleEngine) {
        // Draw the label centered and styled
        let label_art = to_art(self.label.to_string(), "", 0, 0, 0); // Result<String, String>
        let label_x = self.x + (self.width - self.label.len() as i32) / 2;
        let label_y = self.y + self.height / 2;
        let art = label_art.unwrap();

        for (i, line) in art.split('\n').enumerate() {
            // if selected
            if self.selected {
                engine.print_fbg(
                    label_x,
                    label_y + i as i32,
                    line,
                    Color::Yellow,
                    Color::Black,
                );
            } else {
                engine.print_fbg(
                    label_x,
                    label_y + i as i32,
                    line,
                    Color::White,
                    Color::Black,
                );
            }
        }
    }

    fn draw_standard(&self, engine: &mut ConsoleEngine) {
        // Draw the button background
        let bg_color = if self.selected {
            Color::Yellow
        } else {
            Color::White
        };
        engine.fill_rect(
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height,
            pixel::pxl_bg(' ', bg_color),
        );
        // Draw the button border
        self.draw_border(engine);
        // Draw the label centered
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

impl Button {
    pub fn new(width: i32, height: i32, label: String) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
            label,
            selected: false,
        }
    }
}
