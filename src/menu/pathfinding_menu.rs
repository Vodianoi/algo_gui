use std::sync::atomic::AtomicBool;

use crate::algorithms::maze_generation::{Algorithm, AlgorithmRunner};
use crate::algorithms::pathfinding::*;
use crate::menu::theme::default_theme;
use crate::menu::{
    alignment::Alignment, button::Button, dropdown::Dropdown, menu::Menu, menu_item::MenuItem,
};
use console_engine::{ConsoleEngine, KeyCode};

use super::{menu_handler, menu_trait::MenuTrait};

// Run the pathfinding menu.
// This function displays the pathfinding algorithms menu.
// It allows the user to select a pathfinding algorithm to visualize.
pub fn run_pathfinding_menu(engine: &mut ConsoleEngine) {
    // Define the dropdown items for pathfinding algorithms
    let pathfinding_items = vec!["BFS".to_string(), "DFS".to_string()];

    // Create the dropdown as a MenuItem (it will be part of the main Menu)
    let pathfinding_dropdown = Box::new(Dropdown {
        x: 0,
        y: 0,
        width: 20,
        options: pathfinding_items,
        selected_index: 0,
        is_open: false,
        selected: false,
    });
    // Create the "Back" button to allow returning to the previous menu
    let back_button = Box::new(Button {
        x: 0,
        y: 0,
        width: 20,
        height: 3,
        label: "Back".to_string(),
        selected: false,
    });
    // Create the main menu with the dropdown and back button
    let menu_items: Vec<Box<dyn MenuItem>> = vec![pathfinding_dropdown, back_button];

    let mut pathfinding_menu = Menu::new(0, 0, 20, 10, menu_items, Alignment::Center);
    // Create the menu handler
    let mut menu = Box::new(pathfinding_menu);

    loop {
        engine.wait_frame();
        engine.clear_screen();

        // Draw and handle input for the menu
        menu.draw(engine);
        menu.handle_input(engine);

        // Exit the loop if the user requests to quit
        if menu._quit {
            break;
        }

        // Handle confirmation when the user presses "Enter"
        if menu.confirmed() {
            // Pathfinding dropdown confirmed
            let values = menu.get_values();

            let selected_algorithm = values.get(0).unwrap();
            let algorithm: Box<dyn Algorithm> = match selected_algorithm.as_str() {
                "BFS" => Box::new(BFS),
                "DFS" => Box::new(DFS),
                _ => Box::new(BFS),
            };

            // Reset the confirmation state after handling it
            menu.set_confirmed(false);
        }

        engine.draw();
    }
}
