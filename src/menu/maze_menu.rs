use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

use termsize;

use crate::algorithms::maze_generation::*;

use crate::data::data_structures::Maze;
use crate::menu::button::Button;
use crate::menu::dropdown::DropDown;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

use crate::menu::menu_handler::MenuHandler;
use crate::menu::theme::default_theme;

use super::maze_scene::MazeScene;
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
            menu_handler.set_confirmed(false);
            let screen_size = termsize::get().unwrap();
            let display_width = screen_size.cols * 3 / 4;
            let display_height = screen_size.rows - 2; // Leave some space for the menu

            // Calculate the maze width and height
            let maze_width = (display_width - 1) / 4;
            let maze_height = (display_height - 1) / 2;

            // Create the maze
            let maze = Maze::new(maze_width as usize, maze_height as usize);
            let x = (screen_size.cols as i32 - display_width as i32);
            let y = 1; // Start just below the menu
            let scene = MazeScene::new(maze.clone(), x, y, 2);

            let running = Arc::new(AtomicBool::new(true));
            let algorithm: Box<dyn Algorithm> = match menu_handler.get_selected() {
                0 => Box::new(RecursiveBacktracker),
                2 => Box::new(KruskalAlgorithm),
                _ => Box::new(RecursiveBacktracker),
            };

            let runner = AlgorithmRunner::new(algorithm, maze, scene);
            runner.start();

            while running.load(std::sync::atomic::Ordering::SeqCst) {
                engine.wait_frame();
                engine.clear_screen();

                menu_handler.draw(engine);
                menu_handler.handle_input(engine);
                if menu_handler.should_quit || menu_handler.confirmed() {
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
