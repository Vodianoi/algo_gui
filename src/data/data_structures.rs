// Maze data structure
use rand::Rng;
use std::{
    fmt::{Debug, Display, Formatter, Result},
    io::Write,
};

pub const EMPTY_CHAR: char = ' ';
pub const WALL_CHAR: char = '#';
pub const PATH_CHAR: char = ' ';
pub const VISITED_CHAR: char = '.';
pub const GOAL_CHAR: char = 'G';
pub const START_CHAR: char = 'S';

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
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let value: i32 = x as i32 + y as i32;
                row.push(Cell::new(value, WALL_CHAR));
            }
            cells.push(row);
        }

        Maze {
            width,
            height,
            cells,
            start: (1, 1),
            goal: (width as i32 - 2, height as i32 - 2),
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
}

pub struct Graph {
    vertices: Vec<i32>,
    edges: Vec<(i32, i32)>,
}

impl Graph {
    pub fn new(maze: &Maze) -> Graph {
        let mut graph = Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        };
        for y in 0..maze.height {
            for x in 0..maze.width {
                let cell = maze.get_cell(x as i32, y as i32);
                graph.add_vertex(cell.value);
                let neighbors = maze.get_neighbors(x as i32, y as i32);
                for neighbor in neighbors {
                    graph.add_edge(cell.value, maze.get_cell(neighbor.0, neighbor.1).value);
                }
            }
        }
        graph
    }
    pub fn add_vertex(&mut self, value: i32) {
        self.vertices.push(value);
    }
    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.edges.push((from, to));
    }
    pub fn get_neighbors(&self, vertex: i32) -> Vec<i32> {
        let mut neighbors = Vec::new();
        for (from, to) in &self.edges {
            if *from == vertex {
                neighbors.push(*to);
            }
        }
        neighbors
    }
    pub fn get_vertices(&self) -> Vec<i32> {
        self.vertices.clone()
    }
}
