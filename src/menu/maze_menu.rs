use crate::algorithms::maze_generation::*;

use crate::menu::dropdown::DropDown;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

pub fn run_maze_menu(engine: &mut ConsoleEngine) {
    let maze_items = vec![
        "Recursive Backtracker".to_string(),
        "Prim's Algorithm".to_string(),
        "Kruskal's Algorithm".to_string(),
        "Eller's Algorithm".to_string(),
        "Hunt and Kill".to_string(),
        "Aldous-Broder".to_string(),
        "Wilson's Algorithm".to_string(),
        "Confirm".to_string(),
    ];

    let mut maze_menu = DropDown::new(10, 10, 20, maze_items);
    loop {
        engine.wait_frame();
        engine.clear_screen();
        maze_menu.draw(engine);
        maze_menu.update(engine);

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        if maze_menu.confirmed {
            match maze_menu.selected {
                0 => recursive_backtracker(),
                1 => prim_algorithm(),
                2 => kruskal_algorithm(engine),
                3 => eller_algorithm(),
                4 => hunt_and_kill(),
                5 => aldous_broder(),
                6 => wilson_algorithm(),
                _ => (),
            }
            maze_menu.confirmed = false;
        }

        engine.draw();
    }
}
