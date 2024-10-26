use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use termsize;

use crate::algorithms::maze_generation::*;
use crate::data::data_structures::Maze;
use crate::menu::{
    alignment::Alignment, button::Button, dropdown::Dropdown, menu::Menu, menu_item::MenuItem,
};
use console_engine::ConsoleEngine;

use super::maze_scene::MazeScene;

pub fn run_maze_menu(engine: &mut ConsoleEngine) {
    // Define the dropdown items for maze generation algorithms
    let maze_items = vec![
        "Recursive Backtracker".to_string(),
        "Prims Algorithm".to_string(),
        "Kruskals Algorithm".to_string(),
        "Ellers Algorithm".to_string(),
        //"Hunt and Kill".to_string(),
        //"Aldous-Broder".to_string(),
        //"Wilson's Algorithm".to_string(),
    ];

    let max_item_length = maze_items.iter().map(|item| item.len()).max().unwrap() as i32;

    // Create the dropdown for selecting maze algorithms
    let maze_dropdown = Box::new(Dropdown {
        x: 5,
        y: 5,
        width: max_item_length + 2,
        options: maze_items,
        selected_index: 0,
        is_open: false,
        selected: false,
    });

    // Create the "Back" button to return to the previous menu
    let back_button = Box::new(Button {
        x: 5,
        y: 5,
        width: 20,
        height: 3,
        label: "Continue".to_string(),
        selected: false,
    });

    // Create the main menu with the dropdown and back button
    let menu_items: Vec<Box<dyn MenuItem>> = vec![maze_dropdown, back_button];

    let screen_size = termsize::get().unwrap();
    let menu_width = screen_size.cols as i32 / 4;
    let menu_height = screen_size.rows as i32 - 2; // Leave some space for the menu

    let maze_menu = Menu::new(0, 0, menu_width, menu_height, menu_items, Alignment::Left);

    // Create the menu handler
    let mut menu = Box::new(maze_menu);

    loop {
        engine.wait_frame();
        engine.clear_screen();

        // Draw and handle input for the menu
        menu.draw(engine);
        menu.handle_input(engine);
        menu.handle_key_event(engine);

        // Exit the loop if the user requests to quit
        if menu._quit {
            break;
        }

        // Handle confirmation when the user presses "Enter"
        if menu.confirmed() {
            menu.set_confirmed(false);

            let screen_size = termsize::get().unwrap();
            let display_width = screen_size.cols * 3 / 4 - 2;
            let display_height = screen_size.rows; // Leave some space for the menu

            // Calculate the maze dimensions
            let maze_width = (display_width - 1) / 4;
            let maze_height = (display_height - 1) / 2;

            // Create the maze
            let maze = Maze::new(maze_width as usize, maze_height as usize);
            let x = screen_size.cols as i32 - display_width as i32;
            let y = 0; // Start just below the menu
            let scene = MazeScene::new(maze.clone(), x, y, 2);

            // Handle algorithm selection
            let running = Arc::new(AtomicBool::new(true));
            let form_values = menu.get_values();
            let drop_down_value = form_values.first().unwrap();
            let algorithm: Box<dyn Algorithm> = match drop_down_value.as_str() {
                "Recursive Backtracker" => Box::new(RecursiveBacktracker),
                "Prims Algorithm" => Box::new(PrimsAlgorithm),
                "Kruskals Algorithm" => Box::new(KruskalAlgorithm),
                "Ellers Algorithm" => Box::new(EllerAlgorithm),
                // 4 => Box::new(HuntAndKill),
                // 5 => Box::new(AldousBroder),
                // 6 => Box::new(WilsonsAlgorithm),
                _ => Box::new(RecursiveBacktracker),
            };

            // Start the maze generation algorithm
            let runner = AlgorithmRunner::new(algorithm, maze, scene);
            runner.start();

            // Render the maze while the algorithm is running
            while running.load(std::sync::atomic::Ordering::SeqCst) {
                engine.wait_frame();
                engine.clear_screen();

                menu.draw(engine);
                menu.handle_input(engine);
                menu.handle_key_event(engine);

                if menu._quit || menu.confirmed() {
                    menu._quit = false;
                    runner.stop();
                    break;
                }

                runner.render(engine);

                engine.draw();
            }
        }

        engine.draw();
    }
}
