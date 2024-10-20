// Menu struct to create a menu with options (buttons)

use console_engine::pixel;

use crate::helpers::engine_helpers::print_framerate;
use crate::menu::button::Button;

pub struct Menu {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub items: Vec<String>,
    pub selected: usize,
}

impl Menu {
    pub fn new(x: i32, y: i32, w: i32, h: i32, items: Vec<&str>) -> Menu {
        Menu {
            x,
            y,
            w,
            h,
            items: items.iter().map(|s| s.to_string()).collect(),
            selected: 0,
        }
    }
    pub fn draw(&self, engine: &mut console_engine::ConsoleEngine) {
        for (i, item) in self.items.iter().enumerate() {
            let mut button = Button::new(self.x, self.y + (i as i32) * 5, self.w, self.h, item);
            if self.selected == i {
                button.selected = true;
            }
            print_framerate(engine);
            button.draw(engine);
        }
    }
    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }
    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}
