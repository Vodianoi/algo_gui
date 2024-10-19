use console_engine::Color;
use console_engine::ConsoleEngine;
/// Button that can be clicked, using console_engine.
/// Can also be used as a menu item.
use console_engine::KeyCode;

use console_engine::pixel;

pub struct Button {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub text: String,
    pub selected: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, w: i32, h: i32, text: &str) -> Button {
        Button {
            x,
            y,
            w,
            h,
            text: text.to_string(),
            selected: false,
        }
    }

    // Draw the button on the screen
    // If the button is selected, draw it with a different color
    // draw a frame around the button
    pub fn draw(&self, engine: &mut ConsoleEngine) {
        if self.selected {
            engine.print_fbg(
                self.x,
                self.y,
                &self.text,
                console_engine::Color::Red,
                console_engine::Color::Black,
            );
        } else {
            engine.print_fbg(
                self.x,
                self.y,
                &self.text,
                console_engine::Color::White,
                console_engine::Color::Black,
            )
        }
        self.print_frame(engine);
    }

    pub fn print_frame(&self, engine: &mut ConsoleEngine) {
        let start_x = self.x - 1;
        let start_y = self.y - 1;
        let end_x = self.x + self.text.len() as i32 + 1;
        let end_y = self.y + 1;
        let character = '#';
        let pix = pixel::pxl(character);
        for x in start_x..end_x {
            engine.set_pxl(x, start_y, pix);
            engine.set_pxl(x, end_y, pix);
        }
    }

    pub fn is_pressed(&self, engine: &ConsoleEngine) -> bool {
        engine.is_key_pressed(KeyCode::Char(' '))
    }

    pub fn is_selected(&self, engine: &ConsoleEngine) -> bool {
        engine.is_key_pressed(KeyCode::Enter)
    }
}
