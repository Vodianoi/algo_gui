use crate::data::data_structures::{Maze, MazeContext, MazeSettings, Runnable, Scene};
use rand::seq::SliceRandom;
use rand::Rng;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const COLORED: bool = true;

// Recursive Backtracker Algorithm
#[derive(Clone)]
pub struct RecursiveBacktracker;

impl RecursiveBacktracker {
    pub const SETTINGS: MazeSettings = MazeSettings {
        bfs: false,
        colored: COLORED,
        random_colors: false,
        show_values: false,
    };
}

impl Runnable<Maze, MazeContext> for RecursiveBacktracker {
    fn run(
        &self,
        maze: &mut Maze,
        scene: Arc<Mutex<dyn Scene<Maze, MazeContext>>>,
        running: Arc<AtomicBool>,
    ) {
        let mut rng = rand::thread_rng();

        let width = maze.width;
        let height = maze.height;
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let mut current = (rng.gen_range(0..width), rng.gen_range(0..height));
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

                    // Update the scene for visualization
                    let context = MazeContext {
                        path: stack
                            .clone()
                            .into_iter()
                            .map(|(x, y)| (x as i32, y as i32))
                            .collect(),
                        settings: Self::SETTINGS,
                    };
                    scene.lock().unwrap().update(maze, &context);
                }
                None => {
                    current = stack.pop().unwrap();
                }
            }
        }

        // Final cleanup
        maze.clear_path();
        maze.clear_values();
        let context = MazeContext {
            path: vec![],
            settings: Self::SETTINGS,
        };
        scene.lock().unwrap().update(maze, &context);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Maze, MazeContext>> {
        Box::new(self.clone())
    }
}

// Kruskal's Algorithm
#[derive(Clone)]
pub struct KruskalAlgorithm;

impl KruskalAlgorithm {
    pub const SETTINGS: MazeSettings = MazeSettings {
        bfs: false,
        colored: COLORED,
        random_colors: true,
        show_values: false,
    };
}

impl Runnable<Maze, MazeContext> for KruskalAlgorithm {
    fn run(
        &self,
        maze: &mut Maze,
        scene: Arc<Mutex<dyn Scene<Maze, MazeContext>>>,
        running: Arc<AtomicBool>,
    ) {
        let mut rng = rand::thread_rng();
        let width = maze.width;
        let height = maze.height;
        let mut sets: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut walls: Vec<(usize, usize, usize, usize)> = Vec::new();

        // Initialize each cell as a separate set
        for y in 0..height {
            for x in 0..width {
                sets.push(vec![(x, y)]);
                maze.get_cell_mut(x as i32, y as i32).value = (y * width + x) as i32;
                // Assign initial unique value
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

                // Merge the two sets into one
                let mut new_set = set1.clone();
                new_set.extend(set2.clone());
                sets.push(new_set.clone());
                sets.retain(|set| set != &set1 && set != &set2);

                // Update the values of the cells in the merged set (visual purpose)
                let new_value = maze.get_cell(x as i32, y as i32).value;
                for &(cx, cy) in &new_set {
                    maze.get_cell_mut(cx as i32, cy as i32).value = new_value;
                }

                // Update the scene for visualization
                scene.lock().unwrap().update(
                    maze,
                    &MazeContext {
                        path: vec![],
                        settings: Self::SETTINGS,
                    },
                );
            }
        }

        // Final cleanup
        maze.clear_path();
        maze.clear_values();
        scene.lock().unwrap().update(
            maze,
            &MazeContext {
                path: vec![],
                settings: Self::SETTINGS,
            },
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Maze, MazeContext>> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct PrimsAlgorithm;

impl PrimsAlgorithm {
    pub const SETTINGS: MazeSettings = MazeSettings {
        bfs: false,
        colored: COLORED,
        random_colors: false,
        show_values: false,
    };
}

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
impl Runnable<Maze, MazeContext> for PrimsAlgorithm {
    fn run(
        &self,
        maze: &mut Maze,
        scene: Arc<Mutex<dyn Scene<Maze, MazeContext>>>,
        running: Arc<AtomicBool>,
    ) {
        let mut rng = rand::thread_rng();

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
                let context = MazeContext {
                    path: vec![],
                    settings: Self::SETTINGS,
                };
                update_maze(&scene, &maze, &context);
            }
        }

        // Final update of the maze for visualization
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        maze.clear_values();
        let context = MazeContext {
            path: vec![],
            settings: Self::SETTINGS,
        };
        update_maze(&scene, &maze, &context);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Maze, MazeContext>> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct EllerAlgorithm;

impl EllerAlgorithm {
    pub const SETTINGS: MazeSettings = MazeSettings {
        bfs: false,
        colored: COLORED,
        random_colors: false,
        show_values: false,
    };
}

impl Runnable<Maze, MazeContext> for EllerAlgorithm {
    fn run(
        &self,
        maze: &mut Maze,
        scene: Arc<Mutex<dyn Scene<Maze, MazeContext>>>,
        running: Arc<AtomicBool>,
    ) {
        let mut rng = rand::thread_rng();

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
                        for ((_sx, _sy), set) in sets.iter_mut() {
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
                    let context = MazeContext {
                        path: vec![],
                        settings: Self::SETTINGS,
                    };
                    update_maze(&scene, &maze, &context);
                }
            }

            // Visualize step-by-step for debugging
            let context = MazeContext {
                path: vec![],
                settings: Self::SETTINGS,
            };
            update_maze(&scene, &maze, &context);

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
                for ((_sx, _sy), set) in sets.iter_mut() {
                    if *set == right_set {
                        *set = left_set;
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
        maze.clear_path();
        maze.clear_values();
        let context = MazeContext {
            path: vec![],
            settings: Self::SETTINGS,
        };
        update_maze(&scene, &maze, &context);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Maze, MazeContext>> {
        Box::new(self.clone())
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

fn update_maze(
    scene: &Arc<Mutex<dyn Scene<Maze, MazeContext>>>,
    maze: &Maze,
    context: &MazeContext,
) {
    scene.lock().unwrap().update(maze, context);
}
