use console_engine::screen;

use crate::menu::menu::Menu;
use crate::algorithms::maze_generation::*;
use crate::data::data_structures::{Maze, MazeContext, Runnable, Runner};
use crate::scenes::maze_scene::MazeScene;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::{cell, io};

pub fn run_maze_menu() -> io::Result<()> {
    let mut menu = Menu::new(
        "Maze Menu",
        vec![
            "Recursive Backtracker",
            "Prims Algorithm",
            "Kruskals Algorithm",
            "Ellers Algorithm",
            "Quit",
        ],
    );

    menu.run(|selected_index, running| {
        if selected_index == 4 {
            // Quit
            return Ok(());
        }

        // Select the maze generation algorithm
        let algorithm: Box<dyn Runnable<Maze, MazeContext>> = match selected_index {
            0 => Box::new(RecursiveBacktracker),
            1 => Box::new(PrimsAlgorithm),
            2 => Box::new(KruskalAlgorithm),
            3 => Box::new(EllerAlgorithm),
            _ => return Ok(()), // Default case (shouldn't happen)
        };

        // Initialize the maze and scene
        let size = termsize::get().unwrap();
        let cell_size = 2;
        let maze_width = (size.cols as usize / cell_size) - 2;
        let maze_height = size.rows as usize / cell_size;
        let maze = Maze::new(maze_width, maze_height);
        let scene = Arc::new(Mutex::new(MazeScene::new(0,0, cell_size as i32)));

        // Create a runner to execute the algorithm and render the scene
        let mut runner = Runner::new(vec![algorithm], scene.clone(), maze);

        // Start the maze generation process
        runner.start();

        let mut engine = console_engine::ConsoleEngine::init_fill(60).unwrap();
        // Render the maze generation process
        while runner.running.load(std::sync::atomic::Ordering::SeqCst) && running.load(Ordering::SeqCst) {
            // Render the current state of the maze
            runner.render(&mut engine);
            engine.draw();
        }

        Ok(())
    })
}