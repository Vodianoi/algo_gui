// Maze scene is a simple scene that displays the animation of the maze generation algorithm.

use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
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
use rand::Rng;

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
        BFS: bool,
        cache: &mut HashMap<u32, u8>,
    ) {
        // Adjust the maze dimensions to include walls
        let new_width = self.maze.width * 2 + 1;
        let new_height = self.maze.height * 2 + 1;

        let mut laby_with_walls = vec![vec![WALL_CHAR; new_width]; new_height];
        let mut laby_with_walls_values = vec![vec![-1; new_width]; new_height];

        // Construct the maze with walls around it
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let cell = self.maze.get_cell(x as i32, y as i32);

                // Draw the cell's position
                let draw_x = x * 2 + 1;
                let draw_y = y * 2 + 1;
                // Mark visited cells or path
                if cell.visited {
                    laby_with_walls[draw_y][draw_x] = VISITED_CHAR;
                } else {
                    laby_with_walls[draw_y][draw_x] = EMPTY_CHAR;
                }

                // Draw walls if needed

                laby_with_walls_values[draw_y][draw_x] = cell.value;
                if (x as i32, y as i32) == self.maze.start {
                    laby_with_walls[draw_y][draw_x] = START_CHAR;
                } else if (x as i32, y as i32) == self.maze.goal {
                    laby_with_walls[draw_y][draw_x] = GOAL_CHAR;
                }

                if !cell.has_wall_north() && draw_y > 0 {
                    laby_with_walls[draw_y - 1][draw_x] = laby_with_walls[draw_y][draw_x];
                    laby_with_walls_values[draw_y - 1][draw_x] =
                        laby_with_walls_values[draw_y][draw_x];
                }
                if !cell.has_wall_south() && draw_y < new_height - 1 {
                    laby_with_walls[draw_y + 1][draw_x] = laby_with_walls[draw_y][draw_x];
                    laby_with_walls_values[draw_y + 1][draw_x] =
                        laby_with_walls_values[draw_y][draw_x];
                }
                if !cell.has_wall_west() && draw_x > 0 {
                    laby_with_walls[draw_y][draw_x - 1] = laby_with_walls[draw_y][draw_x];
                    laby_with_walls_values[draw_y][draw_x - 1] =
                        laby_with_walls_values[draw_y][draw_x];
                }
                if !cell.has_wall_east() && draw_x < new_width - 1 {
                    laby_with_walls[draw_y][draw_x + 1] = laby_with_walls[draw_y][draw_x];
                    laby_with_walls_values[draw_y][draw_x + 1] =
                        laby_with_walls_values[draw_y][draw_x];
                }
            }
        }

        // Finally, render the maze to the console
        (0..new_height).for_each(|y| {
            for x in 0..new_width {
                let ch = laby_with_walls[y][x];

                let pixel_char = if colored {
                    pixel::pxl_bg(
                        ' ',
                        self.choose_color(
                            ch,
                            laby_with_walls_values[y][x],
                            random_colored,
                            BFS,
                            cache,
                        ),
                    )
                } else {
                    pixel::pxl_fg(
                        ch,
                        self.choose_color(
                            ch,
                            laby_with_walls_values[y][x],
                            random_colored,
                            BFS,
                            cache,
                        ),
                    )
                };

                for i in 0..self.cell_size {
                    engine.set_pxl(self.x + (x * 2) as i32 + i, self.y + y as i32, pixel_char);
                }
            }
        });
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
        if random_colored {
            // Return a random color based on the value of the character (use hash)

            return self.get_color_for_cell(value as u32, cache);
        } else if bfs && value > 0 {
            // Return a color from gradient, percentage is cell value / maze size
            let percentage = value as f32 / (self.maze.width * self.maze.height) as f32 * 100.0;
            let gradient_colors = self.create_gradient((255, 0, 0), 20);
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
