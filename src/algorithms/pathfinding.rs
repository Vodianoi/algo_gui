use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::{data::data_structures::Maze, menu::maze_scene::MazeScene};

use super::maze_generation::Algorithm;

#[derive(Clone)]
pub struct BFS;

impl Algorithm for BFS {
    fn run(&self, maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut maze = maze.lock().unwrap();
        let start = maze.start;
        let goal = maze.goal;
        let width = maze.width;
        let height = maze.height;

        let start = (start.0 as usize, start.1 as usize);

        let goal = (goal.0 as usize, goal.1 as usize);

        let mut queue: Vec<(usize, usize)> = vec![start];

        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        visited[start.1][start.0] = true;

        while running.load(Ordering::SeqCst) && !queue.is_empty() {
            let (x, y) = queue.remove(0);
            {
                maze.get_cell_mut(x as i32, y as i32).visited = true;
                scene.lock().unwrap().maze = maze.clone();
            }
            thread::sleep(Duration::from_millis(50)); // Adjust visualization speed

            // If we reach the end, stop the search
            if (x, y) == goal {
                break;
            }

            // Check each neighboring cell and add it to the queue if unvisited
            let neighbors = [
                (x.wrapping_sub(1), y), // Left
                (x + 1, y),             // Right
                (x, y.wrapping_sub(1)), // Up
                (x, y + 1),             // Down
            ];

            for &(nx, ny) in &neighbors {
                if nx < width
                    && ny < height
                    && !visited[ny][nx]
                    && !maze.get_cell(nx as i32, ny as i32).has_all_walls()
                {
                    visited[ny][nx] = true;
                    queue.push((nx, ny));
                }
            }
        }

        // Final update to visualize the completed path
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        scene.lock().unwrap().maze = maze.clone();
    }
}

#[derive(Clone)]
pub struct DFS;

impl Algorithm for DFS {
    fn run(&self, maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut maze = maze.lock().unwrap();
        let start = maze.start;
        let goal = maze.goal;
        let width = maze.width;
        let height = maze.height;
        let start = (start.0 as usize, start.1 as usize);
        let goal = (goal.0 as usize, goal.1 as usize);
        let mut stack: Vec<(usize, usize)> = vec![start];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        visited[start.1][start.0] = true;
        while running.load(Ordering::SeqCst) && !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            {
                maze.get_cell_mut(x as i32, y as i32).visited = true;
                scene.lock().unwrap().maze = maze.clone();
            }
            thread::sleep(Duration::from_millis(50)); // Adjust visualization speed
                                                      // If we reach the end, stop the search
            if (x, y) == goal {
                break;
            }
            // Check each neighboring cell and add it to the stack if unvisited
            let neighbors = [
                (x.wrapping_sub(1), y), // Left
                (x + 1, y),             // Right
                (x, y.wrapping_sub(1)), // Up
                (x, y + 1),             // Down
            ];
            for &(nx, ny) in &neighbors {
                if nx < width
                    && ny < height
                    && !visited[ny][nx]
                    && !maze.get_cell(nx as i32, ny as i32).has_all_walls()
                {
                    visited[ny][nx] = true;
                    stack.push((nx, ny));
                }
            }
        }
        // Final update to visualize the completed path
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        scene.lock().unwrap().maze = maze.clone();
    }
}
