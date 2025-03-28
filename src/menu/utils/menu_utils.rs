use std::sync::atomic::Ordering;

use crate::menu::{
    alignment::Alignment, items::{button::Button, dropdown::Dropdown}, menu::Menu,
    menu_item::MenuItem,
};
use console_engine::ConsoleEngine;
use termsize;

// /// Initializes a menu with common parameters.
// pub fn initialize_menu(
//     menu_items: Vec<Box<dyn MenuItem>>,
//     alignment: Alignment,
// ) -> Menu {
//     let screen_size = termsize::get().unwrap();
//     let menu_width = screen_size.cols as i32 / 4;
//     let menu_height = screen_size.rows as i32 - 2;

//     // Menu::new(0, 0, menu_width, menu_height, menu_items, alignment)
// }

/// Creates a button with default dimensions.
pub fn create_button(label: &str) -> Box<Button> {
    Box::new(Button::new(20, 3, label.to_string()))
}

/// Creates a dropdown with the given options.
pub fn create_dropdown(options: Vec<String>) -> Box<Dropdown> {
    Box::new(Dropdown::new(22, options, 0, false, false))
}

// /// Handles the main menu loop logic.
// pub fn handle_menu_loop<F>(
//     engine: &mut ConsoleEngine,
//     menu: &mut Menu,
//     on_confirm: F,
// ) where
//     F: Fn(&mut ConsoleEngine, &mut Menu),
// {
//     loop {
//         engine.wait_frame();
//         engine.clear_screen();

//         menu.draw(engine);
//         menu.handle_input(engine);
//         menu.handle_key_event(engine);

//         if menu._quit.load(Ordering::SeqCst) {
//             break;
//         }

//         if menu.confirmed() {
//             menu.set_confirmed(false);
//             on_confirm(engine, menu);
//         }

//         engine.draw();
//     }
// }