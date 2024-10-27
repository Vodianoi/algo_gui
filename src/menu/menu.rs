// Menu struct to create a menu with options (buttons)

use std::borrow::Borrow;

use crate::menu::alignment::Alignment;
use crate::menu::button::Button;
use crate::menu::dropdown::Dropdown;
use crate::menu::menu_item::MenuItem;
use crate::menu::text::Text;
use console_engine::ConsoleEngine;

pub struct Menu {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub items: Vec<Box<dyn MenuItem>>,
    pub selected_index: usize,
    pub _quit: bool,
    pub alignment: Alignment,
    pub confirmed: bool,
}

impl Menu {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        items: Vec<Box<dyn MenuItem>>,
        alignment: Alignment,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            items,
            selected_index: 0,
            _quit: false,
            alignment,
            confirmed: false,
        }
    }
    pub fn draw(&mut self, engine: &mut ConsoleEngine) {
        engine.check_resize();

        self.width = engine.get_width() as i32 / 4;
        self.height = engine.get_height() as i32 - 2;

        let item_count = self.items.len() as i32;
        let spacing = if item_count > 1 {
            self.height / item_count
        } else {
            self.height
        };

        let x_positions: Vec<i32> = self
            .items
            .iter()
            .map(|item| {
                if let Some(button) = item.as_any().downcast_ref::<Button>() {
                    self.calculate_x_position(&button.label)
                } else if let Some(dropdown) = item.as_any().downcast_ref::<Dropdown>() {
                    self.calculate_x_position(&dropdown.options[dropdown.selected_index])
                } else if let Some(text) = item.as_any().downcast_ref::<Text>() {
                    self.calculate_x_position(&text.content)
                } else {
                    self.x // Default position if item type is unknown
                }
            })
            .collect();

        for (i, item) in self.items.iter_mut().enumerate() {
            let y_position = self.y + i as i32 * spacing;

            // Adjust the item's position
            if let Some(button) = item.as_any_mut().downcast_mut::<Button>() {
                button.x = x_positions[i];
                button.y = y_position;
                button.selected = self.selected_index == i;
            } else if let Some(dropdown) = item.as_any_mut().downcast_mut::<Dropdown>() {
                dropdown.x = x_positions[i];
                dropdown.y = y_position;
                dropdown.selected = self.selected_index == i;
            } else if let Some(text) = item.as_any_mut().downcast_mut::<Text>() {
                text.x = x_positions[i];
                text.y = y_position;
            }

            // Draw the item
            item.draw(engine);
        }

        self.draw_border(engine);
    }

    fn draw_border(&self, engine: &mut ConsoleEngine) {
        for x in self.x..self.x + self.width {
            engine.print(x, self.y, "─");
            engine.print(x, self.y + self.height, "─");
        }
        for y in self.y..self.y + self.height {
            engine.print(self.x, y, "│");
            engine.print(self.x + self.width, y, "│");
        }
        engine.print(self.x, self.y, "┌");
        engine.print(self.x + self.width, self.y, "┐");
        engine.print(self.x, self.y + self.height, "└");
        engine.print(self.x + self.width, self.y + self.height, "┘");

        for i in 1..self.height {
            engine.print(self.x + self.width, self.y + i, "│");
        }
    }

    fn calculate_x_position(&self, content: &str) -> i32 {
        match self.alignment {
            Alignment::Left => self.x,
            Alignment::Center => self.x + (self.width - content.len() as i32) / 2,
            Alignment::Right => self.x + self.width - content.len() as i32,
        }
    }

    pub fn get_selected(&self) -> usize {
        self.selected_index
    }

    fn should_quit(&self) -> bool {
        self._quit
    }

    pub fn handle_key_event(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if engine.is_key_pressed(console_engine::KeyCode::Char('q')) {
            self._quit = true;
        }
    }
    pub fn handle_input(&mut self, engine: &mut ConsoleEngine) {
        let is_dropdown_open: bool = self.items.iter().any(|item| {
            if let Some(dropdown) = item.as_any().downcast_ref::<Dropdown>() {
                return dropdown.is_open;
            }
            false
        });
        if is_dropdown_open {
            for item in &mut self.items {
                if let Some(dropdown) = item.as_any_mut().downcast_mut::<Dropdown>() {
                    if dropdown.handle_input(engine) {
                        break;
                    }
                }
            }
        } else {
            if engine.is_key_pressed(console_engine::KeyCode::Enter) {
                if let Some(dropdown) = self.items[self.selected_index]
                    .as_any_mut()
                    .downcast_mut::<Dropdown>()
                {
                    dropdown.is_open = !dropdown.is_open
                } else if let Some(button) = self.items[self.selected_index]
                    .as_any_mut()
                    .downcast_mut::<Button>()
                {
                    button.selected = true;
                    self.confirmed = true;
                }
            }
            if engine.is_key_pressed(console_engine::KeyCode::Up) {
                self.previous_selectable();
            } else if engine.is_key_pressed(console_engine::KeyCode::Down) {
                self.next_selectable();
            }
        }
    }

    fn next_selectable(&mut self) {
        for _ in 0..self.items.len() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
            if self.items[self.selected_index].is_selectable() {
                break;
            }
        }
    }

    fn previous_selectable(&mut self) {
        for _ in 0..self.items.len() {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            if self.items[self.selected_index].is_selectable() {
                break;
            }
        }
    }

    pub fn confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn set_confirmed(&mut self, value: bool) {
        self.confirmed = value;
    }

    pub fn get_values(&self) -> Vec<String> {
        let mut values = vec![];
        for item in &self.items {
            if let Some(dropdown) = item.as_any().downcast_ref::<Dropdown>() {
                values.push(dropdown.options[dropdown.selected_index].clone());
            }
        }
        values
    }
}
