// Drop down menu widget using console_engine

use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

use crate::menu::button;

pub struct DropDown {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub items: Vec<String>,
    pub selected: i32,
    pub opened: bool,
    pub confirmed: bool,
    pub color: Color,
    pub color_hover: Color,
    pub color_selected: Color,
    pub color_selected_hover: Color,
    pub button: button::Button,
}

impl DropDown {
    pub fn new(x: i32, y: i32, width: i32, items: Vec<String>) -> DropDown {
        let items_len = items.len();
        DropDown {
            x: x,
            y: y,
            width: width,
            height: 1,
            items: items,
            selected: 0,
            opened: false,
            confirmed: false,
            color: Color::White,
            color_hover: Color::Blue,
            color_selected: Color::Black,
            color_selected_hover: Color::White,
            button: button::Button::new(x, y + items_len as i32 * 2, 20, 10, "Confirm"),
        }
    }
    pub fn draw(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if self.opened {
            for (i, item) in self.items.iter().enumerate() {
                // ignore last item
                if i == self.items.len() - 1 {
                    continue;
                }

                let color: Color = if i as i32 == self.selected {
                    self.color_selected_hover
                } else {
                    self.color_hover
                };

                engine.print_fbg(
                    self.x,
                    self.y + i as i32,
                    item.as_str(),
                    color,
                    self.color_selected,
                )
            }
        } else {
            engine.print_fbg(
                self.x,
                self.y,
                self.items[self.selected as usize].as_str(),
                self.color,
                self.color_hover,
            );
        }

        self.button.draw(engine);
    }

    // Update the dropdown menu
    // - This function updates the dropdown menu.
    // It allows the user to select an item from the list.
    // Last item is a button to confirm
    pub fn update(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if self.opened {
            if engine.is_key_pressed(KeyCode::Enter) {
                self.opened = false;
            }
            if engine.is_key_pressed(KeyCode::Up) {
                self.selected -= 1;
                if self.selected < 0 {
                    self.selected = self.items.len() as i32 - 2;
                }
            }
            if engine.is_key_pressed(KeyCode::Down) {
                self.selected += 1;
                if self.selected >= self.items.len() as i32 - 1 {
                    self.selected = 0;
                }
            }
        } else {
            // Go to confirm button or dropdown
            if engine.is_key_pressed(KeyCode::Down) {
                self.button.selected = true;
            }

            if engine.is_key_pressed(KeyCode::Up) {
                self.button.selected = false;
            }

            if engine.is_key_pressed(KeyCode::Enter) {
                if !self.button.selected {
                    self.opened = !self.opened;
                } else {
                    self.confirmed = true;
                }
            }
        }
    }
    pub fn get_selected(&self) -> &String {
        &self.items[self.selected as usize]
    }
}
