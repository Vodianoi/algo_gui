use crate::algorithms::pathfinding::*;
use std::thread;

pub fn run_pathfinding_menu() {
    let pathfinding_items = vec!["Breadth First Search", "Depth First Search"];

    println!("Pathfinding Algorithms Menu:");
    for (i, item) in pathfinding_items.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }

    let handle = thread::spawn(move || {
        bfs();
    });

    handle.join().unwrap();
}
