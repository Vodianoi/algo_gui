use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::data::data_structures::Maze;
use crate::menu::maze_scene::MazeScene;

// This file contains the maze generation algorithms.
// Use multiple threads to run the algorithms  and visualize them in the console.

pub fn recursive_backtracker() {
    println!("Running Recursive Backtracker...");
    thread::sleep(Duration::from_secs(2));
}

pub fn prim_algorithm() {
    println!("Running Randomized Prim's Algorithm...");
    thread::sleep(Duration::from_secs(2));
}

pub fn kruskal_algorithm(engine: &mut console_engine::ConsoleEngine) {
    let maze = Arc::new(Mutex::new(Maze::new(10, 10)));
    let scene = Arc::new(Mutex::new(MazeScene::new(
        maze.lock().unwrap().clone(),
        0,
        0,
        1,
    )));

    let maze_clone = Arc::clone(&maze);
    let scene_clone = Arc::clone(&scene);

    thread::spawn(move || loop {
        kruskal_algorithm_thread(maze_clone.clone(), scene_clone.clone());
        thread::sleep(Duration::from_millis(100));
    });

    loop {
        engine.wait_frame();
        engine.clear_screen();

        {
            let scene = scene.lock().unwrap();
            scene.draw(engine);
        }

        if engine.is_key_pressed(console_engine::KeyCode::Char('\n')) {
            break;
        }
    }
}

fn kruskal_algorithm_thread(maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>) {
    let mut maze = maze.lock().unwrap();
    let mut scene = scene.lock().unwrap();
    kruskal_step(&mut maze, &mut scene);
}

fn kruskal_step(maze: &mut Maze, scene: &mut MazeScene) {
    let mut walls = maze.get_walls();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..walls.len());
    let wall = walls[index];
    let (x, y) = wall;
    let (nx, ny) = maze.get_neighbor(x, y);
    if maze.get_cell(x, y).value != maze.get_cell(nx, ny).value {
        maze.set_cell(x, y, maze.get_cell(nx, ny).value);
        scene.maze = maze.clone();
    }
    walls.remove(index);
}

pub fn eller_algorithm() {
    println!("Running Randomized Eller's Algorithm...");
    thread::sleep(Duration::from_secs(2));
}

pub fn hunt_and_kill() {
    println!("Running Hunt-and-Kill Algorithm...");
    thread::sleep(Duration::from_secs(2));
}

pub fn aldous_broder() {
    println!("Running Aldous-Broder Algorithm...");
    thread::sleep(Duration::from_secs(2));
}

pub fn wilson_algorithm() {
    println!("Running Wilson's Algorithm...");
    thread::sleep(Duration::from_secs(2));
}
