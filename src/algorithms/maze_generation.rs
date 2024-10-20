use rand::seq::SliceRandom;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use termsize;

use crate::helpers::engine_helpers::print_framerate;

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
    let width = 25;
    let height = 25;
    let maze = Arc::new(Mutex::new(Maze::new(width, height)));
    let screen_size = termsize::get().unwrap();
    let scene = Arc::new(Mutex::new(MazeScene::new(
        maze.lock().unwrap().clone(),
        screen_size.cols as i32 / 2 - width as i32,
        screen_size.rows as i32 / 2 - height as i32,
        2,
    )));

    let maze_clone = Arc::clone(&maze);
    let scene_clone = Arc::clone(&scene);

    let framerate = Arc::new(Mutex::new(1000));
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let thread = thread::spawn(move || {
        while running_clone.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(framerate.lock().unwrap().clone()));
            kruskal_algorithm_thread(maze_clone.clone(), scene_clone.clone());
        }
    });

    loop {
        engine.wait_frame();
        engine.clear_screen();

        {
            let scene = scene.lock().unwrap();
            scene.draw(engine, false, false, Vec::new(), false);
        }

        if engine.is_key_pressed(console_engine::KeyCode::Char('q')) {
            running.store(false, Ordering::SeqCst);
            break;
        }

        print_framerate(engine);
        print_thread_info(engine, &thread.thread());

        engine.draw();
    }

    // Wait for the thread to finish
    thread.join().unwrap();
}

fn print_thread_info(engine: &mut console_engine::ConsoleEngine, thread: &thread::Thread) {
    // print thread info
    engine.print(0, 0, "Thread Info:");
    engine.print(0, 3, "Algorithm: Kruskal's Algorithm");
    engine.print(0, 4, "Speed: 1 step per second");
    engine.print(0, 5, "Press 'q' to quit");
    engine.print(0, 6, "Press 'p' to pause");
    engine.print(0, 7, "Press 'r' to resume");
    engine.print(0, 8, "Press 's' to step");
}

fn kruskal_algorithm_thread(maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>) {
    let mut maze = maze.lock().unwrap();
    let mut scene = scene.lock().unwrap();

    kruskal_step(&mut maze, &mut scene)
}

fn kruskal_step(maze: &mut Maze, scene: &mut MazeScene) {
    let mut sets: Vec<Vec<(usize, usize)>> = Vec::new();
    for y in 0..maze.height {
        for x in 0..maze.width {
            let mut set = Vec::new();
            set.push((x, y));
            sets.push(set);
        }
    }
    let mut walls: Vec<(usize, usize, usize, usize)> = Vec::new();
    for y in 0..maze.height {
        for x in 0..maze.width {
            if x < maze.width - 1 {
                walls.push((x, y, x + 1, y));
            }
            if y < maze.height - 1 {
                walls.push((x, y, x, y + 1));
            }
        }
    }
    walls.shuffle(&mut rand::thread_rng());
    for wall in walls {
        let (x1, y1, x2, y2) = wall;
        let set1 = find_set(&sets, x1, y1);
        let set2 = find_set(&sets, x2, y2);
        if set1 != set2 {
            maze.remove_wall(x1 as i32, y1 as i32, x2 as i32, y2 as i32);
            let mut new_set = set1.clone();
            new_set.append(&mut set2.clone());
            sets.retain(|s| s != &set1 && s != &set2);
            sets.push(new_set);
        }
    }
    scene.maze = maze.clone();
}

fn find_set(sets: &Vec<Vec<(usize, usize)>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    for set in sets {
        for cell in set {
            if cell == &(x, y) {
                return set.clone();
            }
        }
    }
    Vec::new()
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
