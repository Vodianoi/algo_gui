use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use termsize;

use crate::algorithms::maze_generation::*;
use crate::algorithms::pathfinding::*;
use crate::data::data_structures::Maze;
use crate::menu::{
    alignment::Alignment, button::Button, dropdown::Dropdown, menu::Menu, menu_item::MenuItem,
};
use console_engine::{ConsoleEngine, KeyCode};

use super::maze_scene::MazeScene;

pub fn run_maze_menu(engine: &mut ConsoleEngine) {
    // Define the dropdown items for maze generation and pathfinding algorithms
    let maze_generation_items = vec![
        "Recursive Backtracker".to_string(),
        "Prims Algorithm".to_string(),
        "Kruskals Algorithm".to_string(),
        "Ellers Algorithm".to_string(),
    ];

    let pathfinding_items = vec!["BFS".to_string(), "DFS".to_string()];

    // Create dropdowns for selecting maze and pathfinding algorithms
    let maze_dropdown = Box::new(Dropdown {
        x: 5,
        y: 5,
        width: 22,
        options: maze_generation_items,
        selected_index: 0,
        is_open: false,
        selected: false,
    });

    let pathfinding_dropdown = Box::new(Dropdown {
        x: 5,
        y: 10,
        width: 22,
        options: pathfinding_items,
        selected_index: 0,
        is_open: false,
        selected: false,
    });

    // Create the "Start" button
    let start_button = Box::new(Button {
        x: 5,
        y: 15,
        width: 20,
        height: 3,
        label: "Start".to_string(),
        selected: false,
    });

    // Create the main menu with dropdowns and start button
    let menu_items: Vec<Box<dyn MenuItem>> =
        vec![maze_dropdown, pathfinding_dropdown, start_button];
    let screen_size = termsize::get().unwrap();
    let menu_width = screen_size.cols as i32 / 4;
    let menu_height = screen_size.rows as i32 - 2;

    let combined_menu = Menu::new(0, 0, menu_width, menu_height, menu_items, Alignment::Left);
    let mut menu = Box::new(combined_menu);

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

        // Handle "Start" button confirmation
        if menu.confirmed() {
            menu.set_confirmed(false);

            let form_values = menu.get_values();
            let maze_algorithm = form_values[0].clone();
            let pathfinding_algorithm = form_values[1].clone();

            let screen_size = termsize::get().unwrap();
            let display_width = screen_size.cols * 3 / 4 - 2;
            let display_height = screen_size.rows;

            // Maze dimensions and scene setup
            let maze_width = (display_width - 1) / 4;
            let maze_height = (display_height - 1) / 2;
            let maze = Maze::new(maze_width as usize, maze_height as usize);
            let x = screen_size.cols as i32 - display_width as i32;
            let y = 0;
            let scene = MazeScene::new(maze.clone(), x, y, 2);

            // Maze generation algorithm selection
            let maze_alg: Box<dyn Algorithm> = match maze_algorithm.as_str() {
                "Recursive Backtracker" => Box::new(RecursiveBacktracker),
                "Prims Algorithm" => Box::new(PrimsAlgorithm),
                "Kruskals Algorithm" => Box::new(KruskalAlgorithm),
                "Ellers Algorithm" => Box::new(EllerAlgorithm),
                _ => Box::new(RecursiveBacktracker),
            };

            // Pathfinding algorithm selection
            let path_alg: Box<dyn Algorithm> = match pathfinding_algorithm.as_str() {
                "BFS" => Box::new(BFS),
                "DFS" => Box::new(DFS),
                _ => Box::new(BFS),
            };

            // Determine if running a maze generation or pathfinding algorithm
            let running = Arc::new(AtomicBool::new(true));
            let selected_algorithm: Box<dyn Algorithm> = if !maze_algorithm.is_empty() {
                maze_alg
            } else {
                path_alg
            };

            // Start the selected algorithm
            let runner = AlgorithmRunner::new(selected_algorithm, maze, scene);
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
