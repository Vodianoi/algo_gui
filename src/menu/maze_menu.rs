use crate::algorithms::maze_generation::*;

use crate::menu::button::Button;
use crate::menu::dropdown::DropDown;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

use crate::menu::menu_handler::MenuHandler;
use crate::menu::theme::default_theme;

use super::menu_trait::MenuTrait;

pub fn run_maze_menu(engine: &mut ConsoleEngine) {
    let maze_items = vec![
        "Recursive Backtracker".to_string(),
        "Prim's Algorithm".to_string(),
        "Kruskal's Algorithm".to_string(),
        "Eller's Algorithm".to_string(),
        "Hunt and Kill".to_string(),
        "Aldous-Broder".to_string(),
        "Wilson's Algorithm".to_string(),
    ];

    let mut maze_menu = Box::new(DropDown {
        x: 5,
        y: 5,
        width: 20,
        height: 1,
        items: maze_items,
        selected: 0,
        opened: false,
        confirmed: false,
        color: default_theme().color,
        color_selected: default_theme().color_selected,
        bg_color: default_theme().bg_color,
        button: Button::new(5, 5, 20, 1, "Select Algorithm"),
    });

    let mut menu_handler = MenuHandler::new(maze_menu);

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
                0 => recursive_backtracker(),
                1 => prim_algorithm(),
                2 => kruskal_algorithm(engine),
                3 => eller_algorithm(),
                4 => hunt_and_kill(),
                5 => aldous_broder(),
                6 => wilson_algorithm(),
                _ => (),
            }
            menu_handler.set_confirmed(false);
        }

        engine.draw();
    }
}
