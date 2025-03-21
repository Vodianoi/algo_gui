// Maze scene is a simple scene that displays the animation of the maze generation algorithm.

use std::collections::HashMap;
use std::u8;

use crate::data::data_structures::Maze;

pub const EMPTY_CHAR: char = ' ';
pub const WALL_CHAR: char = '#';
pub const PATH_CHAR: char = ' ';
pub const VISITED_CHAR: char = '.';
pub const GOAL_CHAR: char = 'G';
pub const START_CHAR: char = 'S';

pub const WALL_COLOR: Color = Color::White;
pub const PATH_COLOR: Color = Color::Black;
pub const VISITED_COLOR: Color = Color::Green;
pub const START_COLOR: Color = Color::Red;
pub const GOAL_COLOR: Color = Color::Blue;

use console_engine::pixel;
use console_engine::Color;
use rand::random;

pub struct MazeScene {
    pub maze: Maze,
    pub x: i32,
    pub y: i32,
    pub cell_size: i32,
    pub color_wall: Color,
    pub color_path: Color,
    pub color_visited: Color,
    pub shortest_path: Vec<(usize, usize)>,
}

impl MazeScene {
    pub fn new(maze: Maze, x: i32, y: i32, cell_size: i32) -> MazeScene {
        MazeScene {
            maze,
            x,
            y,
            cell_size,
            color_wall: WALL_COLOR,
            color_path: PATH_COLOR,
            color_visited: VISITED_COLOR,
            shortest_path: vec![],
        }
    }
    pub fn draw(
        &self,
        engine: &mut console_engine::ConsoleEngine,
        colored: bool,
        random_colored: bool,
        bfs: bool,
        cache: &mut HashMap<u32, u8>,
    ) {
        let (new_width, new_height) = (self.maze.width * 2 + 1, self.maze.height * 2 + 1);

        let mut laby_with_walls = vec![vec![WALL_CHAR; new_width]; new_height];
        let mut laby_with_walls_values = vec![vec![-1; new_width]; new_height];

        self.populate_maze_with_walls(&mut laby_with_walls, &mut laby_with_walls_values);
        // 2) Unify corridors only when both adjacent cells share the same BFS/region ID
        //    This ensures no "extra squares" get path color unless they're truly path
        if bfs {
            self.unify_corridors_globally(&mut laby_with_walls, &mut laby_with_walls_values);
        }
        self.render_maze(engine, &laby_with_walls, &laby_with_walls_values, colored, random_colored, bfs, cache);
    }

    fn populate_maze_with_walls(
        &self,
        laby_with_walls: &mut Vec<Vec<char>>,
        laby_with_walls_values: &mut Vec<Vec<i32>>,
    ) {
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let cell = self.maze.get_cell(x as i32, y as i32);
                let (draw_x, draw_y) = (x * 2 + 1, y * 2 + 1);
                if self.shortest_path.contains(&(x as usize, y as usize)) {
                    laby_with_walls[draw_y][draw_x] = PATH_CHAR;
                    laby_with_walls_values[draw_y][draw_x] = -2;

                }
                else {
                    laby_with_walls[draw_y][draw_x] = if cell.visited { VISITED_CHAR } else { EMPTY_CHAR };
                    laby_with_walls_values[draw_y][draw_x] = cell.value;
                }


                if (x as i32, y as i32) == self.maze.start {
                    self.mark_start_or_goal(cell, draw_x, draw_y, START_CHAR, EMPTY_CHAR, laby_with_walls, laby_with_walls_values);
                } else if (x as i32, y as i32) == self.maze.goal {
                    self.mark_start_or_goal(cell, draw_x, draw_y, GOAL_CHAR, EMPTY_CHAR, laby_with_walls, laby_with_walls_values);
                } else if self.shortest_path.contains(&(x, y)) {
                    // Mark the cell center as path
                    laby_with_walls[draw_y][draw_x] = PATH_CHAR;
                    laby_with_walls_values[draw_y][draw_x] = -2; // or whatever BFS marker
                
                    // Only open corridor squares if the neighbor is also in the path
                    self.mark_path_walls_if_neighbor_is_path(
                        cell,
                        x,              // Maze coords
                        y,
                        &self.shortest_path,
                        laby_with_walls,
                        laby_with_walls_values,
                    );
                } else {
                    self.mark_walls(cell, draw_x, draw_y, laby_with_walls[draw_y][draw_x],laby_with_walls, laby_with_walls_values);
                }
            }
        }
    }

        /// This final pass ensures corridor squares only adopt the "path color" if
    /// both adjacent cell centers share the same BFS value (or final path marker).
    pub fn unify_corridors_globally(
        &self,
        laby_with_walls: &mut Vec<Vec<char>>,
        laby_with_walls_values: &mut Vec<Vec<i32>>,
    ) {
        let height = laby_with_walls.len();
        let width = laby_with_walls[0].len();

        for y in 0..height {
            for x in 0..width {
                // We only care about corridor squares or wall squares – i.e. places between cell centers.
                // Cell centers are at (odd, odd). So corridors/walls appear at coordinates where
                // either x or y is even, but not both.
                // 
                // For clarity, let's handle horizontal corridors vs vertical corridors separately:

                // 1) Horizontal corridor: (y is odd, x is even)
                //    The cell centers are at (y, x-1) and (y, x+1)
                if y % 2 == 1 && x % 2 == 0 {
                    // Check bounds for neighbor cell centers
                    if x > 0 && x + 1 < width {
                        let left_val  = laby_with_walls_values[y][x - 1];
                        let right_val = laby_with_walls_values[y][x + 1];
                        if left_val >= 0 && right_val == left_val {
                            // They share the same BFS/region ID => unify corridor
                            // If you specifically want to unify only the final BFS path (e.g. -3),
                            // then check `if left_val == -3 && right_val == -3`.
                            // Or check `shortest_path.contains(...)`, etc.
                            laby_with_walls[y][x] = PATH_CHAR;
                            laby_with_walls_values[y][x] = left_val; 
                        }
                    }
                }

                // 2) Vertical corridor: (y is even, x is odd)
                //    The cell centers are at (y-1, x) and (y+1, x)
                if y % 2 == 0 && x % 2 == 1 {
                    if y > 0 && y + 1 < height {
                        let top_val    = laby_with_walls_values[y - 1][x];
                        let bottom_val = laby_with_walls_values[y + 1][x];
                        if top_val >= 0 && bottom_val == top_val {
                            laby_with_walls[y][x] = PATH_CHAR;
                            laby_with_walls_values[y][x] = top_val;
                        }
                    }
                }

                // 3) "Corners" – coordinates where x and y are both even – are typically walls or pillars.
                //    If you want to unify those corners only when 2 or 3 adjacent cells share the same BFS ID,
                //    you'd do a similar check. But usually, they're left as walls.
                
            }
        }
    }

    fn mark_start_or_goal(
        &self,
        cell: &crate::data::data_structures::Cell,
        draw_x: usize,
        draw_y: usize,
        marker: char,
        no_wall_marker: char,
        laby_with_walls: &mut Vec<Vec<char>>,
        laby_with_walls_values: &mut Vec<Vec<i32>>,
    ) {
        laby_with_walls[draw_y][draw_x] = marker;
        self.mark_walls(cell, draw_x, draw_y, no_wall_marker, laby_with_walls, laby_with_walls_values);
    }

    fn mark_walls(
        &self,
        cell: &crate::data::data_structures::Cell,
        draw_x: usize,
        draw_y: usize,
        no_wall_marker: char,
        laby_with_walls: &mut Vec<Vec<char>>,
        laby_with_walls_values: &mut Vec<Vec<i32>>,
    ) {
        // We'll read the "base" value once, so we don't keep borrowing laby_with_walls_values:
        let base_value = laby_with_walls_values[draw_y][draw_x];
    
        // Inline corridor-opening logic:
        // (Open only if it's still a wall, to avoid overwriting path chars.)
        if !cell.has_wall_north() && draw_y > 0 {
            if laby_with_walls[draw_y - 1][draw_x] == WALL_CHAR {
                laby_with_walls[draw_y - 1][draw_x] = no_wall_marker;
                laby_with_walls_values[draw_y - 1][draw_x] = base_value;
            }
        }
        if !cell.has_wall_south() && draw_y + 1 < laby_with_walls.len() {
            if laby_with_walls[draw_y + 1][draw_x] == WALL_CHAR {
                laby_with_walls[draw_y + 1][draw_x] = no_wall_marker;
                laby_with_walls_values[draw_y + 1][draw_x] = base_value;
            }
        }
        if !cell.has_wall_west() && draw_x > 0 {
            if laby_with_walls[draw_y][draw_x - 1] == WALL_CHAR {
                laby_with_walls[draw_y][draw_x - 1] = no_wall_marker;
                laby_with_walls_values[draw_y][draw_x - 1] = base_value;
            }
        }
        if !cell.has_wall_east() && draw_x + 1 < laby_with_walls[0].len() {
            if laby_with_walls[draw_y][draw_x + 1] == WALL_CHAR {
                laby_with_walls[draw_y][draw_x + 1] = no_wall_marker;
                laby_with_walls_values[draw_y][draw_x + 1] = base_value;
            }
        }
    }
    
    

    fn render_maze(
        &self,
        engine: &mut console_engine::ConsoleEngine,
        laby_with_walls: &[Vec<char>],
        laby_with_walls_values: &[Vec<i32>],
        colored: bool,
        random_colored: bool,
        bfs: bool,
        cache: &mut HashMap<u32, u8>,
    ) {
        for (y, row) in laby_with_walls.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                let value = laby_with_walls_values[y][x];
                let pixel_char = if colored {
                    pixel::pxl_bg(
                        ' ',
                        self.choose_color(ch, value, random_colored, bfs, cache),
                    )
                } else {
                    pixel::pxl_fg(
                        value.to_string().chars().next().unwrap_or(' '),
                        self.choose_color(ch, value, random_colored, bfs, cache),
                    )
                };

                for i in 0..self.cell_size {
                    engine.set_pxl(self.x + (x * 2) as i32 + i, self.y + y as i32, pixel_char);
                }
            }
        }
    }

    fn mark_path_walls_if_neighbor_is_path(
        &self,
        cell: &crate::data::data_structures::Cell,
        maze_x: usize,
        maze_y: usize,
        shortest_path: &[(usize, usize)],
        laby_with_walls: &mut [Vec<char>],
        laby_with_walls_values: &mut [Vec<i32>],
    ) {
        // Convert from Maze coords to ASCII grid coords
        let draw_x = maze_x * 2 + 1;
        let draw_y = maze_y * 2 + 1;
        let base_value = laby_with_walls_values[draw_y][draw_x];
    
        // For each direction, only open the corridor as PATH_CHAR if
        //  (a) the cell does NOT have a wall in that direction
        //  (b) the neighbor is also in shortest_path
        // Otherwise, we leave it as a wall or whatever it was.
    
        // North neighbor in Maze coords is (maze_x, maze_y - 1)
        if !cell.has_wall_north() && maze_y > 0
            && shortest_path.contains(&(maze_x, maze_y - 1))
        {
            // The corridor cell in the ASCII grid is [draw_y - 1][draw_x]
            if laby_with_walls[draw_y - 1][draw_x] == WALL_CHAR {
                laby_with_walls[draw_y - 1][draw_x] = PATH_CHAR;
                laby_with_walls_values[draw_y - 1][draw_x] = base_value;
            }
        }
    
        // South
        if !cell.has_wall_south() && maze_y + 1 < self.maze.height
            && shortest_path.contains(&(maze_x, maze_y + 1))
        {
            if laby_with_walls[draw_y + 1][draw_x] == WALL_CHAR {
                laby_with_walls[draw_y + 1][draw_x] = PATH_CHAR;
                laby_with_walls_values[draw_y + 1][draw_x] = base_value;
            }
        }
    
        // West
        if !cell.has_wall_west() && maze_x > 0
            && shortest_path.contains(&(maze_x - 1, maze_y))
        {
            if laby_with_walls[draw_y][draw_x - 1] == WALL_CHAR {
                laby_with_walls[draw_y][draw_x - 1] = PATH_CHAR;
                laby_with_walls_values[draw_y][draw_x - 1] = base_value;
            }
        }
    
        // East
        if !cell.has_wall_east() && maze_x + 1 < self.maze.width
            && shortest_path.contains(&(maze_x + 1, maze_y))
        {
            if laby_with_walls[draw_y][draw_x + 1] == WALL_CHAR {
                laby_with_walls[draw_y][draw_x + 1] = PATH_CHAR;
                laby_with_walls_values[draw_y][draw_x + 1] = base_value;
            }
        }
    }
    

    fn choose_color(
        &self,
        ch: char,
        value: i32,
        random_colored: bool,
        bfs: bool,
        cache: &mut HashMap<u32, u8>,
    ) -> Color {
        if ch == WALL_CHAR {
            return self.color_wall;
        }
        if ch == GOAL_CHAR {
            return GOAL_COLOR;
        }
        if ch == START_CHAR {
            return START_COLOR;
        }
        if random_colored {
            // Return a random color based on the value of the character (use hash)
            return self.get_color_for_cell(value as u32, cache);
        } else if bfs {
            
            if value == -1 {
                return self.color_path;
            }
            else if value == -2 {
                return self.color_visited;
            }

            // Return a color from gradient, percentage is cell value / maze size
            let percentage = value as f32 / (self.maze.width * self.maze.height) as f32 * 100.0;
            let gradient_colors = self.create_gradient((0, 126, 126), 100);
            return Color::AnsiValue(self.get_color_from_percentage(&gradient_colors, percentage));
        }
        match ch {
            WALL_CHAR => self.color_wall,
            VISITED_CHAR => self.color_visited,
            START_CHAR => START_COLOR,
            GOAL_CHAR => GOAL_COLOR,
            _ => self.color_path,
        }
    }

    /// Converts an RGB color to an ANSI 256 color code.
    fn rgb_to_ansi(&self, r: u8, g: u8, b: u8) -> u8 {
        let r_norm = (r as f32 / 255.0 * 5.0).round() as u8;
        let g_norm = (g as f32 / 255.0 * 5.0).round() as u8;
        let b_norm = (b as f32 / 255.0 * 5.0).round() as u8;
        16 + 36 * r_norm + 6 * g_norm + b_norm
    }

    /// Creates a gradient from a base color with decreasing brightness steps.
    fn create_gradient(&self, base_color: (u8, u8, u8), steps: u8) -> Vec<u8> {
        let (r, g, b) = base_color;
        let mut gradient_colors = Vec::new();

        for i in 0..steps {
            let factor = (steps - i) as f32 / steps as f32;
            let r_step = (r as f32 * factor).round() as u8;
            let g_step = (g as f32 * factor).round() as u8;
            let b_step = (b as f32 * factor).round() as u8;
            let ansi_code = self.rgb_to_ansi(r_step, g_step, b_step);
            gradient_colors.push(ansi_code);
        }

        gradient_colors
    }

    /// Maps a percentage to a color on a gradient.
    fn get_color_from_percentage(&self, gradient_colors: &[u8], percentage: f32) -> u8 {
        let index = ((percentage / 100.0).powf(2.0) * (gradient_colors.len() - 1) as f32) as usize;
        gradient_colors[index]
    }

    /// Generates a color based on cell value.
    fn get_color_for_cell(&self, v: u32, cache: &mut HashMap<u32, u8>) -> Color {

        if v == 0 {
            return self.color_path;
        }
        // Check if the color is already cached
        if let Some(color) = cache.get(&v) {
            return Color::AnsiValue(*color);
        }



        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        let color = Color::Rgb { r, g, b };

        // Cache the color
        cache.insert(v, self.rgb_to_ansi(r, g, b));

        color
    }

}

impl Clone for MazeScene {
    fn clone(&self) -> MazeScene {
        MazeScene {
            maze: self.maze.clone(),
            x: self.x,
            y: self.y,
            cell_size: self.cell_size,
            color_wall: self.color_wall,
            color_path: self.color_path,
            color_visited: self.color_visited,
            shortest_path: self.shortest_path.clone(),
        }
    }
}
