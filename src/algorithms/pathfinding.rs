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
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut maze = scene.lock().unwrap().maze.clone();
        let start = maze.start;
        let goal = maze.goal;
        let width = maze.width;
        let height = maze.height;

        let start = (start.0 as usize, start.1 as usize);

        let goal = (goal.0 as usize, goal.1 as usize);

        let mut queue: Vec<(usize, usize)> = vec![start];

        let mut value = 0;

        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        visited[start.1][start.0] = true;
        maze.get_cell_mut(start.0 as i32, start.1 as i32).value = value;

        while running.load(Ordering::SeqCst) && !queue.is_empty() {
            let (x, y) = queue.remove(0);
            {
                let cell = maze.get_cell_mut(x as i32, y as i32);
                cell.visited = true;
                cell.value = value;
                value += 1;
                scene.lock().unwrap().maze = maze.clone();
            }
            thread::sleep(Duration::from_millis(10)); // Adjust visualization speed

            // If we reach the end, stop the search
            if (x, y) == goal {
                break;
            }

            // Check each neighboring cell and add it to the queue if unvisited
            let neighbors = maze.get_valid_neighbors(x as i32, y as i32);

            for &(nx, ny) in &neighbors {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < width && ny < height && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push((nx, ny));
                }
            }
        }
        // Final update to visualize the completed path
        scene.lock().unwrap().maze = maze.clone();
        thread::sleep(Duration::from_secs(2));
        self.find_shortest_path(&mut maze, scene);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl BFS {
    pub fn find_shortest_path(&self, maze: &mut Maze, scene: Arc<Mutex<MazeScene>>) {
        maze.clear_path();
        scene.lock().unwrap().maze = maze.clone();
        let width = maze.width;
        let height = maze.height;
        let goal = maze.goal;
        let goal = (goal.0 as usize, goal.1 as usize);
        let mut value = maze.get_cell(goal.0 as i32, goal.1 as i32).value;
        let mut path = vec![(goal.0, goal.1)];
        while value > 0 {
            let (x, y) = path.last().unwrap();
            let neighbors = maze.get_valid_neighbors(*x as i32, *y as i32);
            // Add min value of neighbors to the path
            let mut min = value;
            for &(nx, ny) in &neighbors {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < width && ny < height {
                    let cell = maze.get_cell(nx as i32, ny as i32);
                    if cell.value < min && cell.value != -1 {
                        min = cell.value;
                    }
                }
            }
            if min == value {
                break;
            }

            for &(nx, ny) in &neighbors {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < width && ny < height {
                    let cell = maze.get_cell(nx as i32, ny as i32);
                    if cell.value == min {
                        path.push((nx, ny));
                        value = min;
                        maze.get_cell_mut(nx as i32, ny as i32).visited = true;
                        scene.lock().unwrap().shortest_path = path.clone();
                        thread::sleep(Duration::from_millis(10)); // Adjust visualization speed

                        break;
                    }
                }
            }
        }
        for &(x, y) in &path {
            maze.get_cell_mut(x as i32, y as i32).visited = true;
        }
        scene.lock().unwrap().maze = maze.clone();
    }
}

#[derive(Clone)]
pub struct DFS;

// Depth-first search algorithm
// maze is a grid of cells, each cell has a set of walls (n,s,w, e) that can be removed by setting the value to false
impl Algorithm for DFS {
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut maze = scene.lock().unwrap().maze.clone();
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
            thread::sleep(Duration::from_millis(10)); // Adjust visualization speed
                                                      // If we reach the end, stop the search
            if (x, y) == goal {
                break;
            }
            // Check each neighboring cell and add it to the stack if unvisited

            let neighbors = maze.get_valid_neighbors(x as i32, y as i32);

            for &(nx, ny) in &neighbors {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < width && ny < height && !visited[ny][nx] {
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
