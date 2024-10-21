// Menu struct to create a menu with options (buttons)

use crate::menu::menu_trait::MenuTrait;

use crate::menu::button::Button;

pub struct Menu {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub items: Vec<String>,
    pub selected: usize,
    pub _quit: bool,
}

impl MenuTrait for Menu {
    fn draw(&mut self, engine: &mut console_engine::ConsoleEngine) {
        for (i, item) in self.items.iter().enumerate() {
            let mut button = Button::new(self.x, self.y + (i as i32) * 5, self.w, self.h, item);
            if self.selected == i {
                button.selected = true;
            }
            button.draw(engine);
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn select(&self) -> Option<String> {
        Some(self.items[self.selected].clone())
    }

    fn get_selected(&self) -> usize {
        self.selected
    }

    fn should_quit(&self) -> bool {
        self._quit
    }

    fn handle_key_event(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if engine.is_key_pressed(console_engine::KeyCode::Char('q')) {
            self._quit = true;
        }
        if engine.is_key_pressed(console_engine::KeyCode::Enter) {
            self._quit = true;
        }
    }

    fn confirmed(&self) -> bool {
        todo!()
    }

    fn set_confirmed(&mut self, value: bool) {
        todo!()
    }
}
