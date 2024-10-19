use crate::algorithms::maze_generation::*;
use std::thread;

pub fn run_maze_menu() {
    let maze_items = vec![
        "Recursive Backtracker", "Randomized Prim's Algorithm", "Randomized Kruskal's Algorithm",
        "Randomized Eller's Algorithm", "Hunt-and-Kill Algorithm", "Aldous-Broder Algorithm", "Wilson's Algorithm"
    ];

    println!("Maze Generation Algorithms Menu:");
    for (i, item) in maze_items.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }

    let handle = thread::spawn(move || {
        recursive_backtracker();
    });

    handle.join().unwrap();
}

