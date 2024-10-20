use crate::algorithms::pathfinding::*;

use crate::menu::dropdown::DropDown;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

// Run the pathfinding menu.
//  - This function displays the pathfinding algorithms menu.
//  It allows the user to select a pathfinding algorithm to visualize.
//  and the maze generation algorithm

pub fn run_pathfinding_menu(engine: &mut ConsoleEngine) {
    let pathfinding_items = vec!["BFS".to_string(), "DFS".to_string()];

    let mut pathfinding_menu = DropDown::new(0, 0, 30, pathfinding_items);
    // Display the pathfinding menu
    // User can select one of each list of pathfinding and maze generation algorithms
    // And also select maze size (w, h)
    // When pressing enter, the CheckBox will be marked as selected,
    // confirm with the confirm from Form
    loop {
        engine.wait_frame();
        engine.clear_screen();
        pathfinding_menu.draw(engine);

        if engine.is_key_pressed(KeyCode::Char('\n')) {
            break;
        }

        pathfinding_menu.update(engine);
    }
}
