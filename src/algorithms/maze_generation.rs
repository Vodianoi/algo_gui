use crate::data::data_structures::{Cell, Maze};
use crate::menu::maze_scene::MazeScene;
use rand::seq::SliceRandom;
use rand::Rng;
use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termsize;
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
    fn run(&self, maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>);
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
    fn run(&self, maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut rng = rand::thread_rng();
        let width = maze.lock().unwrap().width;
        let height = maze.lock().unwrap().height;
        let mut maze = maze.lock().unwrap();
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let mut current = (rng.gen_range(0..width), rng.gen_range(0..height));
        visited[current.1][current.0] = true;
        stack.push(current);
        while running.load(Ordering::SeqCst) && !stack.is_empty() {
            let neighbor = choose_random_neighbor(current, width, height, &visited);
            let current_cell = maze.get_cell_mut(current.0 as i32, current.1 as i32);
            current_cell.visited = true;
            match neighbor {
                Some(next) => {
                    let (nx, ny) = next;
                    maze.remove_wall(current.0 as i32, current.1 as i32, nx as i32, ny as i32);
                    current = next;
                    visited[current.1][current.0] = true;
                    stack.push(current);
                }
                None => {
                    current = stack.pop().unwrap();
                }
            }

            // Update scene and visualization after each step
            scene.lock().unwrap().maze = maze.clone();
        }
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        scene.lock().unwrap().maze = maze.clone();
    }
}

// Kruskal's Algorithm
#[derive(Clone)]
pub struct KruskalAlgorithm;

// maze is a grid of cells, each cell has a set of walls (n,s,e,w) that can be removed by setting the value to false
impl Algorithm for KruskalAlgorithm {
    fn run(&self, maze: Arc<Mutex<Maze>>, scene: Arc<Mutex<MazeScene>>, running: Arc<AtomicBool>) {
        let mut rng = rand::thread_rng();
        let width = maze.lock().unwrap().width;
        let height = maze.lock().unwrap().height;
        let mut sets: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut walls: Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut maze = maze.lock().unwrap();
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
        }
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
    algorithm: Box<dyn Algorithm>,
    maze: Arc<Mutex<Maze>>,
    scene: Arc<Mutex<MazeScene>>,
    running: Arc<AtomicBool>,
}
impl AlgorithmRunner {
    pub fn new(algorithm: Box<dyn Algorithm>, maze: Maze, scene: MazeScene) -> Self {
        AlgorithmRunner {
            algorithm,
            maze: Arc::new(Mutex::new(maze)),
            scene: Arc::new(Mutex::new(scene)),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&self) {
        let maze_clone = Arc::clone(&self.maze);
        let scene_clone = Arc::clone(&self.scene);
        let running_clone = Arc::clone(&self.running);
        let algorithm_clone = self.algorithm.clone();

        // Spawn a thread to run the algorithm
        thread::spawn(move || {
            algorithm_clone.run(maze_clone, scene_clone, running_clone);
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn render(&self, engine: &mut console_engine::ConsoleEngine) {
        engine.clear_screen();
        let scene = self.scene.lock().unwrap();
        // Different settings by algorithm:
        let algorithm_type = self.algorithm.as_ref();
        let settings: (bool, bool, Vec<Cell>, bool) =
            if algorithm_type.type_id() == RecursiveBacktracker.type_id() {
                (true, false, Vec::new(), false)
            } else if algorithm_type.type_id() == KruskalAlgorithm.type_id() {
                (true, true, Vec::new(), false)
            } else {
                (true, false, Vec::new(), false)
            };
        scene.draw(engine, settings.0, settings.1, settings.2, settings.3);
        engine.draw();
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
