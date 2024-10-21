use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

use crate::menu::button;
use crate::menu::menu_trait::MenuTrait;

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
    pub color_selected: Color,
    pub bg_color: Color,
    pub button: button::Button,
}

impl DropDown {
    pub fn new(x: i32, y: i32, width: i32, items: Vec<String>) -> DropDown {
        let items_len = items.len();
        DropDown {
            x,
            y,
            width,
            height: 1,
            items,
            selected: 0,
            opened: false,
            confirmed: false,
            color: Color::White,
            color_selected: Color::Blue,
            bg_color: Color::Black,
            button: button::Button::new(x, y + items_len as i32 * 2, 20, 10, "Confirm"),
        }
    }

    pub fn is_confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn get_selected_item(&self) -> &String {
        &self.items[self.selected as usize]
    }
}

// Implementing the MenuTrait for DropDown
impl MenuTrait for DropDown {
    fn draw(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if self.opened {
            // Draw dropdown border
            engine.rect(
                self.x - 1,
                self.y - 1,
                self.x + self.width + 1,
                self.y + self.items.len() as i32,
                pixel::pxl_fbg(' ', self.color, self.color),
            );

            for (i, item) in self.items.iter().enumerate() {
                let fg = if i as i32 == self.selected {
                    self.color_selected
                } else {
                    self.color
                };
                let bg = if i as i32 == self.selected {
                    self.color
                } else {
                    self.bg_color
                };

                // Adding padding and centering text within the width
                let padding_left =
                    " ".repeat((self.width as usize + 1).saturating_sub(item.len()) / 2);
                let padding_right = " ".repeat(
                    (self.width as usize).saturating_sub(item.len()) - padding_left.len() + 1,
                );

                // IF length of longest item is greater than width, truncate it
                if item.len() > self.width as usize {
                    let item = item.chars().take(self.width as usize).collect::<String>();
                    let display_text = format!("{}{}{}", padding_left, item, padding_right);
                    engine.print_fbg(self.x, self.y + i as i32, display_text.as_str(), fg, bg);
                    continue;
                }

                let display_text = format!("{}{}{}", padding_left, item, padding_right);

                engine.print_fbg(self.x, self.y + i as i32, display_text.as_str(), fg, bg);
            }
        } else {
            let fg = if self.button.selected {
                self.color
            } else {
                self.bg_color
            };
            let bg = if self.button.selected {
                self.bg_color
            } else {
                self.color
            };

            // Display the currently selected item with some padding and background color
            let selected_item = &self.items[self.selected as usize];
            let padding = " ".repeat((self.width as usize).saturating_sub(selected_item.len()) / 2);
            let display_text = format!("{}{}", padding, selected_item);

            engine.print_fbg(self.x, self.y, display_text.as_str(), fg, bg);
        }

        // Adjust button position depending on dropdown state
        if self.opened {
            self.button.y = self.y + self.items.len() as i32 + 4; // Better spacing below dropdown
        } else {
            self.button.y = self.y + 4;
        }

        // Draw the button
        self.button.draw(engine);
    }

    fn next(&mut self) {
        if self.opened {
            self.selected += 1;
            if self.selected >= self.items.len() as i32 {
                self.selected = 0;
            }
        } else {
            self.button.selected = true;
        }
    }

    fn previous(&mut self) {
        if self.opened {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.items.len() as i32 - 1;
            }
        } else {
            self.button.selected = false;
        }
    }

    fn select(&self) -> Option<String> {
        if self.confirmed {
            Some(self.items[self.selected as usize].clone())
        } else {
            None
        }
    }

    fn get_selected(&self) -> usize {
        self.selected as usize
    }

    fn should_quit(&self) -> bool {
        self.confirmed
    }

    fn handle_key_event(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if self.opened {
            if engine.is_key_pressed(KeyCode::Enter) {
                self.opened = false;
            }
        } else {
            if engine.is_key_pressed(KeyCode::Down) {
                self.button.selected = true;
            }

            if engine.is_key_pressed(KeyCode::Up) {
                self.button.selected = false;
            }

            if engine.is_key_pressed(KeyCode::Enter) {
                if self.button.selected {
                    self.confirmed = true;
                } else {
                    self.opened = !self.opened;
                }
            }
        }
    }

    fn confirmed(&self) -> bool {
        self.confirmed
    }

    fn set_confirmed(&mut self, value: bool) {
        self.confirmed = value;
    }
}
