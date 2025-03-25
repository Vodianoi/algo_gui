use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Result},
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

use console_engine::{Color, ConsoleEngine};
use rand::{random, Rng};


pub const EMPTY_CHAR: char = ' ';
pub const WALL_CHAR: char = '#';
pub const PATH_CHAR: char = ' ';
// pub const VISITED_CHAR: char = '.';
pub const GOAL_CHAR: char = 'G';
pub const START_CHAR: char = 'S';

#[derive(Clone,PartialEq)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub start: (i32, i32),
    pub goal: (i32, i32),
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut cells = Vec::new();
        let mut count = 0;
        (0..height).for_each(|_y| {
            let mut row = Vec::new();
            (0..width).for_each(|_x| {
                let value: i32 = count;
                count += 2;
                row.push(Cell::new(value, WALL_CHAR));
            });
            cells.push(row);
        });

        let start = (random::<usize>() % width as usize, random::<usize>() % height as usize);
        let goal =  (random::<usize>() % width as usize, random::<usize>() % height as usize);

        Maze {
            width,
            height,
            cells,
            start: (start.0 as i32, start.1 as i32),
            goal: (goal.0 as i32, goal.1 as i32),
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }

    pub fn get_random_cell(&self) -> (i32, i32) {
        let x = rand::thread_rng().gen_range(0..self.width as i32);
        let y = rand::thread_rng().gen_range(0..self.height as i32);
        (x, y)
    }

    pub fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width as i32 - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height as i32 - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    pub fn get_walls(&self) -> Vec<(i32, i32)> {
        let mut walls = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_cell(x as i32, y as i32).c == WALL_CHAR {
                    walls.push((x as i32, y as i32));
                }
            }
        }
        walls
    }

    pub fn get_neighbor(&self, x: i32, y: i32) -> (i32, i32) {
        let neighbors = self.get_neighbors(x, y);
        let index = rand::thread_rng().gen_range(0..neighbors.len());
        neighbors[index]
    }

    pub fn remove_wall(&mut self, x: i32, y: i32, nx: i32, ny: i32) {
        let dx = nx - x;
        let dy = ny - y;
        // North, South, West, East
        if dx == 1 {
            self.get_cell_mut(x, y).walls[3] = false;
            self.get_cell_mut(nx, ny).walls[2] = false;
        } else if dx == -1 {
            self.get_cell_mut(x, y).walls[2] = false;
            self.get_cell_mut(nx, ny).walls[3] = false;
        } else if dy == 1 {
            self.get_cell_mut(x, y).walls[1] = false;
            self.get_cell_mut(nx, ny).walls[0] = false;
        } else if dy == -1 {
            self.get_cell_mut(x, y).walls[0] = false;
            self.get_cell_mut(nx, ny).walls[1] = false;
        }
    }

    pub fn set_start(&mut self, x: i32, y: i32) {
        self.start = (x, y);
        self.cells[y as usize][x as usize].c = START_CHAR;
    }

    pub fn set_goal(&mut self, x: i32, y: i32) {
        self.goal = (x, y);
        self.cells[y as usize][x as usize].c = GOAL_CHAR;
    }

    pub fn clear_path(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &mut self.cells[y][x];
                if cell.c == PATH_CHAR {
                    cell.c = EMPTY_CHAR;
                } else if cell.visited {
                    cell.visited = false;
                    cell.c = EMPTY_CHAR;
                }
            }
        }
    }

    pub fn clear_values(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y][x].value = -1;
            }
        }
    }

    pub fn set_cell(&mut self, x: i32, y: i32, value: i32) {
        self.cells[y as usize][x as usize].value = value;
    }

    pub fn clone(&self) -> Maze {
        let mut cells = Vec::new();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                row.push(self.cells[y][x].clone());
            }
            cells.push(row);
        }
        Maze {
            width: self.width,
            height: self.height,
            cells,
            start: self.start,
            goal: self.goal,
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        let mut content = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                content.push(self.cells[y][x].c);
            }
            content.push('\n');
        }
        file.write_all(content.as_bytes()).unwrap();
    }

    // Get valid neighbors of a cell
    // A valid neighbor is a cell that is within the maze bounds and has not wall between the two
    pub fn get_valid_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        let cell = self.get_cell(x, y);
        let walls = &cell.walls;
        if !walls[0] && y > 0 {
            neighbors.push((x, y - 1));
        }
        if !walls[1] && y < self.height as i32 - 1 {
            neighbors.push((x, y + 1));
        }
        if !walls[2] && x > 0 {
            neighbors.push((x - 1, y));
        }
        if !walls[3] && x < self.width as i32 - 1 {
            neighbors.push((x + 1, y));
        }
        neighbors
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.cells[y][x].c);
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl Debug for Maze {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut output = String::new();

        // Output percentage of WALL_CHAR in the maze
        let mut wall_count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                for wall in &cell.walls {
                    if *wall {
                        wall_count += 1;
                    }
                }
            }
        }
        let total_cells = self.width * self.height * 4;
        let wall_percentage = (wall_count as f32 / total_cells as f32) * 100.0;
        output.push_str(&format!(
            "Maze ({}x{}) with {}% walls\n",
            self.width, self.height, wall_percentage
        ));
        write!(f, "{}", output)
    }
}

#[derive(Clone,PartialEq)]
pub struct Cell {
    pub walls: [bool; 4],
    pub visited: bool,
    pub value: i32,
    pub c: char,
}

impl Cell {
    pub fn new(value: i32, c: char) -> Cell {
        Cell {
            walls: [true, true, true, true],
            visited: false,
            value,
            c,
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn visit(&mut self, value: i32) {
        self.visited = true;
        self.value = value;
    }

    pub fn operator_eq(&self, other: &Cell) -> bool {
        self.value == other.value
    }

    pub fn operator_ne(&self, other: &Cell) -> bool {
        self.value != other.value
    }

    pub fn clone(&self) -> Cell {
        Cell {
            walls: self.walls,
            visited: self.visited,
            value: self.value,
            c: self.c,
        }
    }

    pub fn has_wall_north(&self) -> bool {
        self.walls[0]
    }
    pub fn has_wall_south(&self) -> bool {
        self.walls[1]
    }
    pub fn has_wall_west(&self) -> bool {
        self.walls[2]
    }
    pub fn has_wall_east(&self) -> bool {
        self.walls[3]
    }

    pub fn has_all_walls(&self) -> bool {
        self.walls.iter().all(|&wall| wall)
    }

    pub fn has_wall(&self, index: usize) -> bool {
        self.walls[index]
    }
}

// Common trait for tasks that can be run by the Runner
pub trait Runnable<T: Send + Sync, C: VisualizationContext>: Send + Sync {
    fn run(&self, data: &mut T, scene: Arc<Mutex<dyn Scene<T, C>>>, running: Arc<AtomicBool>);
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Runnable<T, C>>;
}

impl<T: Send + Sync, F, C: VisualizationContext> Runnable<T, C> for F
where
    F: 'static + Send + Sync + Clone + Fn(&mut T, Arc<Mutex<dyn Scene<T,C>>>, Arc<AtomicBool>),
{
    fn run(&self, data: &mut T, scene: Arc<Mutex<dyn Scene<T, C>>>, running: Arc<AtomicBool>) {
        self(data, scene, running);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<T, C>> {
        Box::new(self.clone())
    }
}

impl<T: Send + Sync,C: VisualizationContext> Clone for Box<dyn Runnable<T, C>> {
    fn clone(&self) -> Box<dyn Runnable<T, C>> {
        self.clone_box()
    }
}

// Common trait for scenes (MazeScene, SortScene, etc.)
pub trait Scene<T, C>: Send + Sync + Any {
    fn render(&mut self, engine: &mut ConsoleEngine);
    fn update(&mut self, data: &T, context: &C);
    fn as_any(&self) -> &dyn Any;
}

// Shared wrapper for scenes
pub struct SharedScene<T: 'static, C: 'static>(Arc<Mutex<dyn Scene<T, C>>>);

impl<T: 'static, C: 'static> SharedScene<T, C> {
    pub fn new(scene: Arc<Mutex<dyn Scene<T, C>>>) -> Self {
        Self(scene)
    }

    pub fn render(&self, engine: &mut ConsoleEngine)
    where
        T: 'static,
        C: 'static,
    {
        self.0.lock().unwrap().render(engine);
    }

    pub fn update(&self, data: &T, context: &C) {
        self.0.lock().unwrap().update(data, context);
    }
}

impl<T, C> Clone for SharedScene<T, C> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

// Generic Runner for both sorting and algorithms
pub struct Runner<T: Clone + 'static, C: 'static> {
    tasks: Vec<Box<dyn Runnable<T, C>>>,
    current_task: Arc<AtomicUsize>,
    scene: SharedScene<T, C>,
    pub running: Arc<AtomicBool>,
    data: T, // Store the data here
}

impl<T: Clone, C> Runner<T, C>
where
    T: 'static + Send + Sync,
    C: 'static + VisualizationContext,
{
    pub fn new(tasks: Vec<Box<dyn Runnable<T, C>>>, scene: Arc<Mutex<dyn Scene<T, C>>>, data: T) -> Self {
        Runner {
            tasks,
            current_task: Arc::new(AtomicUsize::new(0)),
            scene: SharedScene::new(scene),
            running: Arc::new(AtomicBool::new(true)),
            data,
        }
    }

    pub fn start(&mut self) {
        let scene_clone = self.scene.clone();
        let running_clone = Arc::clone(&self.running);
        let tasks = self.tasks.clone();
        let current_task_clone = Arc::clone(&self.current_task);
        let mut data = self.data.clone();

        thread::spawn(move || {
            current_task_clone.store(0, Ordering::Relaxed);
            let _task_length = tasks.len();

            for (index, task) in tasks.into_iter().enumerate() {
                current_task_clone.store(index, Ordering::Relaxed);

                if running_clone.load(Ordering::SeqCst) {
                    task.run(&mut data, scene_clone.0.clone(), running_clone.clone());
                } else {
                    break;
                }
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn render(&mut self, engine: &mut ConsoleEngine) {
        self.scene.render(engine);
    }
}


pub trait VisualizationContext: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

// impl<T: 'static + Send + Sync> VisualizationContext for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

#[derive(Clone, Debug)]
pub struct MazeContext {
    pub path: Vec<(i32, i32)>,
    pub settings: MazeSettings,
}
impl MazeContext {
    pub(crate) fn default() -> MazeContext {
        MazeContext {
            path: Vec::new(),
            settings: MazeSettings {
                colored: false,
                show_values: false,
                random_colors: false,
                bfs: false,
            },
        }
    }
}

impl VisualizationContext for MazeContext {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct SortingContext {
    pub highlights: Vec<usize>,
    pub color: Color,
}

impl VisualizationContext for SortingContext {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct MazeSettings {
    pub colored: bool,
    pub show_values: bool,
    pub random_colors: bool,
    pub bfs: bool,
}