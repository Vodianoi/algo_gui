// src/menu/dropdown.rs

use crate::menu::menu_item::MenuItem;
use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;

use text_to_ascii_art::*;

pub struct Dropdown {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub is_open: bool,
    pub selected: bool,
}

// Helper function to print ASCII art using console_engine and text_to_ascii_art
pub fn print_ascii(
    engine: &mut ConsoleEngine,
    x: i32,
    y: i32,
    text: &str,
    color_fg: Color,
    color_bg: Color,
) {
    let art = to_art(text.to_string(), "", 0, 0, 0).unwrap();
    let lines = art.split("\n").map(|line| line.to_string());
    let y = y + 1;
    for (i, line) in lines.enumerate() {
        engine.print_fbg(x, y + i as i32, &line, color_fg, color_bg);
    }
}
impl MenuItem for Dropdown {
    fn draw(&self, engine: &mut ConsoleEngine) {
        let screeen_width = engine.get_width();
        // Check if the screen width is large enough to draw ASCII art (Menu should take 1/4 of the screen)
        if screeen_width > 500 {
            self.draw_ascii(engine);
        } else {
            self.draw_standard(engine);
        }
    }

    fn draw_ascii(&self, engine: &mut console_engine::ConsoleEngine) {
        let bg_color = if self.selected {
            Color::DarkGrey
        } else {
            Color::Black
        };
        let fg_color = if self.selected {
            Color::Green
        } else {
            Color::White
        };
        let dropdown_text = if self.is_open {
            self.options[self.selected_index].clone()
        } else {
            ">".to_string()
        };
        print_ascii(
            engine,
            self.x + 10,
            self.y,
            &self.options[self.selected_index].clone(),
            fg_color,
            bg_color,
        );

        print_ascii(
            engine,
            self.x + 1,
            self.y,
            &dropdown_text,
            fg_color,
            bg_color,
        );

        if self.is_open {
            let height = 10;
            for (i, option) in self.options.iter().enumerate() {
                if i == self.selected_index {
                    print_ascii(
                        engine,
                        self.x + 1 + 8,
                        self.y + 10 + i as i32 * height,
                        option,
                        Color::Green,
                        bg_color,
                    );
                } else {
                    print_ascii(
                        engine,
                        self.x + 1 + 8,
                        self.y + 10 + i as i32 * height,
                        option,
                        Color::White,
                        bg_color,
                    );
                }
            }
        }
    }

    fn draw_standard(&self, engine: &mut ConsoleEngine) {
        let bg_color = if self.selected {
            Color::DarkGrey
        } else {
            Color::Black
        };
        let fg_color = if self.selected {
            Color::Green
        } else {
            Color::White
        };

        engine.rect(
            self.x - 1,
            self.y - 1,
            self.x + self.width + 1,
            self.y + 2,
            pixel::Pixel {
                // Draw the dropdown background
                chr: ' ',
                fg: Color::White,
                bg: bg_color,
            },
        );

        engine.fill_rect(
            // Draw the dropdown background
            self.x,
            self.y,
            self.x + self.width,
            self.y + 1,
            pixel::pxl_bg(' ', bg_color),
        );

        engine.print_fbg(
            self.x + 1,
            self.y + 1,
            &self.options[self.selected_index],
            fg_color,
            bg_color,
        );

        if self.is_open {
            for (i, option) in self.options.iter().enumerate() {
                let option_y = self.y + 3 + i as i32;
                if i == self.selected_index {
                    let option = &format!("> {}", option);
                    engine.print(self.x + 1, option_y, option);
                } else {
                    engine.print(self.x + 1, option_y, option);
                }
            }
        }
    }

    fn handle_input(&mut self, engine: &mut ConsoleEngine) -> bool {
        if self.is_open {
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
                self.is_open = false;
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
