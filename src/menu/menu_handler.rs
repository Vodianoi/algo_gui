use console_engine::{KeyCode, KeyModifiers};

use crate::menu::menu_trait::MenuTrait;

use std::process as system;

pub struct MenuHandler {
    pub menu: Box<dyn MenuTrait>, // Menu, Form, DropDown, etc.
    pub should_quit: bool,
}

impl MenuHandler {
    pub fn new(menu: Box<dyn MenuTrait>) -> Self {
        MenuHandler {
            menu,
            should_quit: false,
        }
    }

    pub fn draw(&mut self, engine: &mut console_engine::ConsoleEngine) {
        self.menu.draw(engine);

        engine.check_resize();
    }

    pub fn handle_input(&mut self, engine: &mut console_engine::ConsoleEngine) {
        self.should_quit = false;
        if engine.is_key_pressed(KeyCode::Down) {
            self.menu.next();
        }
        if engine.is_key_pressed(KeyCode::Up) {
            self.menu.previous();
        }
        if engine.is_key_pressed(KeyCode::Char('q')) {
            self.should_quit = true;
        }
        if engine.is_key_pressed_with_modifier(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
            console_engine::KeyEventKind::Press,
        ) {
            // Clear screen
            engine.clear_screen();
            // Exit application
            system::exit(0);
        }

        self.menu.handle_key_event(engine);
    }

    pub fn get_selected(&self) -> usize {
        self.menu.get_selected()
    }

    pub fn confirmed(&self) -> bool {
        self.menu.confirmed()
    }

    pub fn set_confirmed(&mut self, value: bool) {
        self.menu.set_confirmed(value);
    }
}
