// Take text input from the user

use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

struct TextField {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    text: String,
    cursor_pos: i32,
    cursor_visible: bool,
    cursor_blink_timer: f32,
    cursor_blink_interval: f32,
}

impl TextField {
    fn new(x: i32, y: i32, width: i32, height: i32) -> TextField {
        TextField {
            x,
            y,
            width,
            height,
            text: String::new(),
            cursor_pos: 0,
            cursor_visible: true,
            cursor_blink_timer: 0.0,
            cursor_blink_interval: 0.5,
        }
    }
    fn draw(&self, engine: &mut ConsoleEngine) {
        engine.fill_region(
            self.x,
            self.y,
            self.width,
            self.height,
            pixel::pxl(' ', Color::Black, Color::White),
        );
        let mut text = self.text.clone();
        if text.len() > self.width as usize - 2 {
            text = text[text.len() - (self.width as usize - 2)..].to_string();
        }
        engine.print(self.x + 1, self.y + 1, &text);
        if self.cursor_visible {
            engine.print(self.x + 1 + self.cursor_pos, self.y + 1, "_");
        }
    }
    fn update(&mut self, delta: f32) {
        self.cursor_blink_timer += delta;
        if self.cursor_blink_timer > self.cursor_blink_interval {
            self.cursor_blink_timer = 0.0;
            self.cursor_visible = !self.cursor_visible;
        }
    }
    fn key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.text.remove(self.cursor_pos as usize - 1);
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.text.len() as i32 {
                    self.text.remove(self.cursor_pos as usize);
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < self.text.len() as i32 {
                    self.cursor_pos += 1;
                }
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
            }
            KeyCode::End => {
                self.cursor_pos = self.text.len() as i32;
            }
            KeyCode::Char(c) => {
                self.text.insert(self.cursor_pos as usize, c);
            }
            _ => {}
        }
    }
}
