// Menu struct to create a menu with options (buttons)
use crate::menu::alignment::Alignment;
use crate::menu::{items::{button::Button, dropdown::Dropdown, text::Text}, menu_item::MenuItem};
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

        // Calculate the maximum content width for consistent alignment
        let max_content_width = self
            .items
            .iter()
            .map(|item| {
                if let Some(button) = item.as_any().downcast_ref::<Button>() {
                    button.label.len() as i32
                } else if let Some(dropdown) = item.as_any().downcast_ref::<Dropdown>() {
                    dropdown.options[dropdown.selected_index].len() as i32
                } else if let Some(text) = item.as_any().downcast_ref::<Text>() {
                    text.content.len() as i32
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        let x_positions: Vec<i32> = self
            .items
            .iter()
            .map(|_item| self.calculate_x_position(max_content_width))
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
        engine.rect_border(
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height,
            console_engine::rect_style::BorderStyle::new_solid(),
        );
    }

    fn calculate_x_position(&self, max_content_width: i32) -> i32 {
        match self.alignment {
            Alignment::Left => self.x,
            Alignment::Center => self.x + (self.width - max_content_width) / 2,
            Alignment::Right => self.x + self.width - max_content_width,
        }
    }

    fn mouse_pressed(&mut self, engine: &mut ConsoleEngine) {
        let mouse_coords = engine.get_mouse_press(console_engine::MouseButton::Left);
        let mouse_x = match mouse_coords {
            Some((x, _)) => x as i32,
            None => return,
        };
        let mouse_y = match mouse_coords {
            Some((_, y)) => y as i32,
            None => return,
        };

        for (i, item) in self.items.iter_mut().enumerate() {
            if let Some(button) = item.as_any_mut().downcast_mut::<Button>() {
                if button.is_mouse_over(mouse_x, mouse_y) {
                    if self.selected_index == i && button.selected {
                        self.confirmed = true; // Confirm on double click
                        button.handle_input(engine);
                        print!("Mouse double-clicked on button: {}", button.label);
                    } else {
                        self.selected_index = i;
                        button.selected = true;
                    }
                }
            } else if let Some(dropdown) = item.as_any_mut().downcast_mut::<Dropdown>() {
                if dropdown.is_mouse_over(mouse_x, mouse_y) {
                    if self.selected_index == i && dropdown.selected {
                        self.confirmed = true; // Confirm on double click
                        dropdown.handle_input(engine);
                        print!("Mouse double-clicked on dropdown: {}", dropdown.options[dropdown.selected_index]);
                    } else {
                        self.selected_index = i;
                        dropdown.selected = true;
                    }
                }
            }
        }
    }

    // pub fn get_selected(&self) -> usize {
    //     self.selected_index
    // }

    // fn should_quit(&self) -> bool {
    //     self._quit
    // }

    pub fn handle_key_event(&mut self, engine: &mut console_engine::ConsoleEngine) {
        if engine.is_key_pressed(console_engine::KeyCode::Char('q')) {
            self._quit = true;
        }
    }
    pub fn handle_input(&mut self, engine: &mut ConsoleEngine) {
        self.mouse_pressed(engine);
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
