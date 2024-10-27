use crate::data::data_structures::{Cell, Graph, Maze};
use crate::menu::maze_scene::MazeScene;

use rand::seq::SliceRandom;
use rand::Rng;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::pathfinding::{BFS, DFS};

// Add a helper trait for cloning trait objects
pub trait DynClone {
    fn clone_box(&self) -> Box<dyn Algorithm>;
}

// Implement `DynClone` for all types that implement `Clone`
impl<T> DynClone for T
where
    T: 'static + Algorithm + Clone,
{
    fn clone_box(&self) -> Box<dyn Algorithm> {
        Box::new(self.clone())
    }
}

// Make the `Algorithm` trait extend `DynClone`
pub trait Algorithm: DynClone + Any + Send + Sync {
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Implement `Clone` for `Box<dyn Algorithm>`
impl Clone for Box<dyn Algorithm> {
    fn clone(&self) -> Box<dyn Algorithm> {
        self.clone_box()
    }
}

// Recursive Backtracker Algorithm
#[derive(Clone)]
pub struct RecursiveBacktracker;

impl Algorithm for RecursiveBacktracker {
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut rng = rand::thread_rng();
        let mut maze = scene.lock().unwrap().maze.clone();

        let width = maze.width;
        let height = maze.height;
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let mut current = (rng.gen_range(0..width), rng.gen_range(0..height));
        let mut count = 0;
        visited[current.1][current.0] = true;
        maze.get_cell_mut(current.0 as i32, current.1 as i32)
            .visited = true;

        stack.push(current);
        while running.load(Ordering::SeqCst) && !stack.is_empty() {
            let neighbor = choose_random_neighbor(current, width, height, &visited);
            maze.get_cell_mut(current.0 as i32, current.1 as i32)
                .visited = true;
            match neighbor {
                Some(next) => {
                    let (nx, ny) = next;
                    maze.remove_wall(current.0 as i32, current.1 as i32, nx as i32, ny as i32);
                    current = next;
                    visited[current.1][current.0] = true;
                    stack.push(current);
                    // Update scene and visualization after each step
                    scene.lock().unwrap().maze = maze.clone();
                    thread::sleep(Duration::from_millis(10));
                }
                None => {
                    current = stack.pop().unwrap();
                }
            }
        }
        maze.get_cell_mut(current.0 as i32, current.1 as i32)
            .visited = true;
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        maze.clear_values();
        scene.lock().unwrap().maze = maze.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Kruskal's Algorithm
#[derive(Clone)]
pub struct KruskalAlgorithm;

// maze is a grid of cells, each cell has a set of walls (n,s,e,w) that can be removed by setting the value to false
impl Algorithm for KruskalAlgorithm {
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut maze = scene.lock().unwrap().maze.clone();
        let mut rng = rand::thread_rng();
        let width = maze.width;
        let height = maze.height;
        let mut sets: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut walls: Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut count = 0;
        // Initialize each cell as a separate set
        for y in 0..height {
            for x in 0..width {
                sets.push(vec![(x, y)]);
            }
        }
        // Add all walls to the list
        for y in 0..height {
            for x in 0..width {
                if x > 0 {
                    walls.push((x, y, x - 1, y));
                }
                if y > 0 {
                    walls.push((x, y, x, y - 1));
                }
            }
        }
        // Shuffle the walls
        walls.shuffle(&mut rng);
        while running.load(Ordering::SeqCst) && !walls.is_empty() {
            let (x, y, nx, ny) = walls.pop().unwrap();
            let set1 = find_set(&sets, x, y);
            let set2 = find_set(&sets, nx, ny);
            if set1 != set2 {
                maze.remove_wall(x as i32, y as i32, nx as i32, ny as i32);

                // Determine the minimum value between the two sets
                let min_value = std::cmp::min(
                    maze.get_cell(x as i32, y as i32).value,
                    maze.get_cell(nx as i32, ny as i32).value,
                );

                // Merge the two sets into one and update cell values
                let mut new_set = set1.clone();
                new_set.extend(set2.clone());

                // Update cell values
                for (x, y) in new_set.iter() {
                    maze.get_cell_mut(*x as i32, *y as i32).value = min_value;
                }

                sets.push(new_set);
                sets.retain(|set| set != &set1 && set != &set2);
            }

            // Update scene and visualization after each step
            scene.lock().unwrap().maze = maze.clone();
            thread::sleep(Duration::from_millis(10));
        }

        // Final update of the maze for visualization
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        maze.clear_values();
        scene.lock().unwrap().maze = maze.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct PrimsAlgorithm;

// maze is a grid of cells, each cell has a set of walls (n,s,e,w) that can be removed by setting the value to false
// This algorithm is a randomized version of Prim's algorithm.
//
//    Start with a grid full of walls.
//    Pick a cell, mark it as part of the maze. Add the walls of the cell to the wall list.
//    While there are walls in the list:
//        Pick a random wall from the list. If only one of the cells that the wall divides is visited, then:
//            Make the wall a passage and mark the unvisited cell as part of the maze.
//            Add the neighboring walls of the cell to the wall list.
//        Remove the wall from the list.
//
// Uses Graph data structure to represent the maze
impl Algorithm for PrimsAlgorithm {
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut rng = rand::thread_rng();
        let mut maze = scene.lock().unwrap().maze.clone();

        // Initialize the walls list with the start cell's neighbors
        let mut walls: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let start = maze.start;
        maze.get_cell_mut(start.0, start.1).visited = true;

        for neighbor in maze.get_neighbors(start.0, start.1) {
            walls.push((start, neighbor));
        }

        // Run the algorithm while there are walls to process and the running flag is true
        while running.load(Ordering::SeqCst) && !walls.is_empty() {
            // Select a random wall from the list
            let random_wall_index = rng.gen_range(0..walls.len());
            let (cell, next) = walls.swap_remove(random_wall_index);

            let (cx, cy) = cell;
            let (nx, ny) = next;

            // If next cell is not visited, remove the wall between cell and next
            if !maze.get_cell(nx, ny).is_visited() {
                maze.remove_wall(cx, cy, nx, ny);

                // Mark the cell as visited and add its neighbors to the wall list
                maze.get_cell_mut(nx, ny).visit(0);

                for neighbor in maze.get_neighbors(nx, ny) {
                    if !maze.get_cell(neighbor.0, neighbor.1).is_visited() {
                        walls.push(((nx, ny), neighbor));
                    }
                }

                // Update the scene for visualization
                scene.lock().unwrap().maze = maze.clone();
            }
        }

        // Final update of the maze for visualization
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        maze.clear_values();
        scene.lock().unwrap().maze = maze.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct EllerAlgorithm;

impl Algorithm for EllerAlgorithm {
    fn run(&self, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut rng = rand::thread_rng();
        let mut maze = scene.lock().unwrap().maze.clone();

        let width = maze.width;
        let height = maze.height;

        // Each cell has a unique set to start
        let mut sets = HashMap::new();
        let mut next_set = 1;

        for y in 0..height {
            // Step 1: Initialize each cell in the row with a unique set (if it doesn't have one)
            for x in 0..width {
                maze.set_cell(x as i32, y as i32, next_set);
                sets.insert((x, y), next_set);
                next_set += 1;
            }

            // Step 2: Randomly merge adjacent cells in the row
            for x in 0..(width - 1) {
                if rng.gen_bool(0.5) {
                    // Randomly decide to join sets
                    let current_set = sets[&(x, y)];
                    let right_set = sets[&(x + 1, y)];

                    if current_set != right_set {
                        maze.remove_wall(x as i32, y as i32, (x + 1) as i32, y as i32);

                        // Update set references
                        for ((sx, sy), set) in sets.iter_mut() {
                            if *set == right_set {
                                *set = current_set;
                            }
                        }
                    }
                }
            }

            // Step 3: Randomly extend cells in each set vertically downwards
            if y < height - 1 {
                let mut cells_in_set = HashMap::new();
                for x in 0..width {
                    let set = sets[&(x, y)];
                    cells_in_set.entry(set).or_insert(Vec::new()).push(x);
                }

                for (set, cells) in cells_in_set.iter() {
                    let mut extended = false;
                    for &x in cells {
                        if rng.gen_bool(0.5) || !extended {
                            maze.remove_wall(x as i32, y as i32, x as i32, (y + 1) as i32);
                            sets.insert((x, y + 1), *set);
                            extended = true;
                        } else {
                            sets.insert((x, y + 1), next_set);
                            next_set += 1;
                        }
                    }
                    scene.lock().unwrap().maze = maze.clone();
                }
            }

            // Visualize step-by-step for debugging
            scene.lock().unwrap().maze = maze.clone();
            thread::sleep(Duration::from_millis(10));

            if !running.load(Ordering::SeqCst) {
                break;
            }
        }

        // Step 4: Final row - join all cells to complete the maze
        let final_y = height - 1;
        for x in 0..(width - 1) {
            let left_set = sets[&(x, final_y)];
            let right_set = sets[&(x + 1, final_y)];
            if left_set != right_set {
                maze.remove_wall(x as i32, final_y as i32, (x + 1) as i32, final_y as i32);
                for ((sx, sy), set) in sets.iter_mut() {
                    if *set == right_set {
                        *set = left_set;
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        maze.clear_values();
        scene.lock().unwrap().maze = maze.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Helper functions to find the set containing a specific cell
fn find_set(sets: &Vec<Vec<(usize, usize)>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    for set in sets {
        if set.contains(&(x, y)) {
            return set.clone();
        }
    }
    vec![]
}

// Algorithm runner using multithreading and visualization
pub struct AlgorithmRunner {
    algorithms: Vec<Box<dyn Algorithm>>,
    current_algorithm: Arc<AtomicUsize>,
    scene: Arc<Mutex<MazeScene>>,
    running: Arc<AtomicBool>,
    cache: HashMap<u32, u8>,
    last_settings: (bool, bool, bool),
}

impl AlgorithmRunner {
    pub fn new(algorithms: Vec<Box<dyn Algorithm>>, scene: MazeScene) -> Self {
        AlgorithmRunner {
            algorithms,
            current_algorithm: Arc::new(AtomicUsize::new(0)),
            last_settings: (true, false, false),

            scene: Arc::new(Mutex::new(scene)),
            running: Arc::new(AtomicBool::new(true)),
            cache: HashMap::new(),
        }
    }

    pub fn start(&self) {
        let scene_clone = Arc::clone(&self.scene);
        let running_clone = Arc::clone(&self.running);
        let algorithms = self.algorithms.clone();
        let current_algorithm_clone = Arc::clone(&self.current_algorithm);

        thread::spawn(move || {
            current_algorithm_clone.store(0, Ordering::Relaxed);
            let algorithm_length = algorithms.len();

            for algorithm in algorithms {
                // Run each algorithm only if `running` is still true
                if running_clone.load(Ordering::SeqCst) {
                    algorithm.run(scene_clone.clone(), running_clone.clone());
                } else {
                    break;
                }
                // Add 1 modulo the number of algorithms to the current index
                current_algorithm_clone.fetch_add(1, Ordering::Relaxed);
                current_algorithm_clone.store(
                    current_algorithm_clone.load(Ordering::Relaxed) % algorithm_length,
                    Ordering::Relaxed,
                );
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn render(&mut self, engine: &mut console_engine::ConsoleEngine) {
        let scene = self.scene.lock().unwrap();
        self.last_settings =
            self.get_settings(&*self.algorithms[self.current_algorithm.load(Ordering::Relaxed)]);

        scene.draw(
            engine,
            self.last_settings.0,
            self.last_settings.1,
            self.last_settings.2,
            &mut self.cache,
        );
    }

    fn get_settings(&self, algorithm: &dyn Algorithm) -> (bool, bool, bool) {
        if algorithm.as_any().is::<RecursiveBacktracker>() {
            (true, false, false)
        } else if algorithm.as_any().is::<KruskalAlgorithm>() {
            (true, true, false)
        } else if algorithm.as_any().is::<PrimsAlgorithm>() {
            (true, false, false)
        } else if algorithm.as_any().is::<BFS>() {
            (true, false, true)
        } else if algorithm.as_any().is::<DFS>() {
            (true, false, false)
        } else if algorithm.as_any().is::<EllerAlgorithm>() {
            (true, false, false)
        } else {
            self.last_settings
        }
    }
}
// Utility functions

fn choose_random_neighbor(
    current: (usize, usize),
    width: usize,
    height: usize,
    visited: &Vec<Vec<bool>>,
) -> Option<(usize, usize)> {
    //  Choose random neighbor within maze bounds
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let (x, y) = current;

    if x > 0 && !visited[y][x - 1] {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 && !visited[y][x + 1] {
        neighbors.push((x + 1, y));
    }
    if y > 0 && !visited[y - 1][x] {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 && !visited[y + 1][x] {
        neighbors.push((x, y + 1));
    }

    if neighbors.is_empty() {
        None
    } else {
        Some(*neighbors.choose(&mut rand::thread_rng()).unwrap())
    }
}
