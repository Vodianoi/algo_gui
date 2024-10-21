/// Form menu module to create a form with input fields
/// and submit button
/// using console_engine
use console_engine::pixel;
use console_engine::Color;

use crate::menu::menu_trait::MenuTrait;

use crate::menu::button::Button;
use crate::menu::button::CheckBox;

pub struct Form {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub fields: Vec<String>,
    pub selected: usize,
    pub confirmed: bool,
    pub _quit: bool,
}

impl MenuTrait for Form {
    fn draw(&mut self, engine: &mut console_engine::ConsoleEngine) {
        for (i, field) in self.fields.iter().enumerate() {
            let mut checkbox =
                CheckBox::new(self.x, self.y + (i as i32) * 5, self.w, self.h, field);
            if self.selected == i {
                checkbox.selected = true;
            }
            checkbox.draw(engine);
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.fields.len();
    }

    fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.fields.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn select(&self) -> Option<String> {
        Some(self.fields[self.selected].clone())
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
            self.confirmed = true;
        }
    }

    fn confirmed(&self) -> bool {
        self.confirmed
    }

    fn set_confirmed(&mut self, value: bool) {
        self.confirmed = value;
    }
}

pub struct MultiFormSelect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub fields: Vec<Vec<String>>,
    pub selected: Vec<usize>,
}

// MultiFormSelect is a form with multiple fields
// It uses multiple menus to select multiple items
// And submit the form
impl MultiFormSelect {
    pub fn new(x: i32, y: i32, w: i32, h: i32, fields: Vec<Vec<&str>>) -> MultiFormSelect {
        MultiFormSelect {
            x,
            y,
            w,
            h,
            fields: fields
                .iter()
                .map(|s| s.iter().map(|s| s.to_string()).collect())
                .collect(),
            selected: vec![0; fields.len()],
        }
    }
    pub fn draw(&self, engine: &mut console_engine::ConsoleEngine) {
        // Last field
        let last_x = self.fields.len() - 1;
        let last_field = &self.fields[last_x];
        for (i, field) in self.fields.iter().enumerate() {
            for (j, item) in field.iter().enumerate() {
                if item == &last_field[0] {
                    // Confirm button
                    let mut button = Button::new(
                        self.x + (j as i32) * 5,
                        self.y + (i as i32) * 5,
                        self.w,
                        self.h,
                        item,
                    );

                    if self.selected[i] == j {
                        button.selected = true;
                    }
                    button.draw(engine);
                    break;
                }
                let mut checkbox = CheckBox::new(
                    self.x + (j as i32) * 5,
                    self.y + (i as i32) * 5,
                    self.w,
                    self.h,
                    item,
                );
                if self.selected[i] == j {
                    checkbox.selected = true;
                }
                checkbox.draw(engine);
            }
        }
    }

    pub fn next(&mut self) {
        let len = self.selected.len() - 1;
        self.selected[len] = (self.selected[self.selected.len() - 1] + 1) % self.fields[0].len();
    }

    pub fn previous(&mut self) {
        let len = self.selected.len() - 1;
        if self.selected[len] == 0 {
            self.selected[len] = self.fields[0].len() - 1;
        } else {
            self.selected[len] -= 1;
        }
    }

    pub fn select(&mut self) {
        // mark the selected checkbox as selected
        // and unmark the others
        for i in 0..self.fields.len() {
            for j in 0..self.fields[i].len() {
                if j == self.selected[i] {
                    self.selected[i] = j;
                }
            }
        }
    }
}
