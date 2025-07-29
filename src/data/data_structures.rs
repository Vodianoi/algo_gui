// Maze data structure
use rand::Rng;

pub const EMPTY_CHAR: char = ' ';
pub const WALL_CHAR: char = '#';
pub const PATH_CHAR: char = ' ';
pub const VISITED_CHAR: char = '.';
pub const GOAL_CHAR: char = 'G';
pub const START_CHAR: char = 'S';

// Indices for wall directions in the `walls` array of a `Cell`
pub const NORTH: usize = 0;
pub const SOUTH: usize = 1;
pub const WEST: usize = 2;
pub const EAST: usize = 3;

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
                row.push(Cell::new(value, EMPTY_CHAR));
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
        if dx == 1 {
            // remove east wall of the current cell and west wall of the neighbour
            self.get_cell_mut(x, y).walls[EAST] = false;
            self.get_cell_mut(nx, ny).walls[WEST] = false;
        } else if dx == -1 {
            // remove west wall of the current cell and east wall of the neighbour
            self.get_cell_mut(x, y).walls[WEST] = false;
            self.get_cell_mut(nx, ny).walls[EAST] = false;
        } else if dy == 1 {
            // remove south wall of the current cell and north wall of the neighbour
            self.get_cell_mut(x, y).walls[SOUTH] = false;
            self.get_cell_mut(nx, ny).walls[NORTH] = false;
        } else if dy == -1 {
            // remove north wall of the current cell and south wall of the neighbour
            self.get_cell_mut(x, y).walls[NORTH] = false;
            self.get_cell_mut(nx, ny).walls[SOUTH] = false;
        }

        self.get_cell_mut(x, y).c = EMPTY_CHAR;
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
        self.walls[NORTH]
    }
    pub fn has_wall_south(&self) -> bool {
        self.walls[SOUTH]
    }
    pub fn has_wall_west(&self) -> bool {
        self.walls[WEST]
    }
    pub fn has_wall_east(&self) -> bool {
        self.walls[EAST]
    }
}
