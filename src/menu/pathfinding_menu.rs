use crate::algorithms::pathfinding::*;
use crate::menu::theme::default_theme;

use crate::menu::button::Button;
use crate::menu::dropdown::DropDown;
use console_engine::{ConsoleEngine, KeyCode};

use super::{menu_handler, menu_trait::MenuTrait};

// Run the pathfinding menu.
//  - This function displays the pathfinding algorithms menu.
//  It allows the user to select a pathfinding algorithm to visualize.
//  and the maze generation algorithm

pub fn run_pathfinding_menu(engine: &mut ConsoleEngine) {
    let pathfinding_items = vec!["BFS".to_string(), "DFS".to_string()];

    let mut pathfinding_menu = DropDown {
        x: 0,
        y: 0,
        width: 20,
        height: 1,
        items: pathfinding_items,
        selected: 0,
        opened: false,
        confirmed: false,
        color: default_theme().color,
        color_selected: default_theme().color_selected,
        bg_color: default_theme().bg_color,
        button: Button::new(5, 5, 20, 1, "Pathfinding"),
    };

    let mut menu_handler = menu_handler::MenuHandler::new(Box::new(pathfinding_menu));
    loop {
        engine.wait_frame();
        engine.clear_screen();
        menu_handler.draw(engine);
        menu_handler.handle_input(engine);

        if menu_handler.should_quit {
            break;
        }

        if menu_handler.confirmed() {
            match menu_handler.get_selected() {
                0 => bfs(),
                1 => dfs(),
                _ => bfs(),
            };

            menu_handler.set_confirmed(false);
        }

        engine.draw();
    }
}
