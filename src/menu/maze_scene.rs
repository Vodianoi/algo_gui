// Maze scene is a simple scene that displays the animation of the maze generation algorithm.

use crate::data::data_structures::Cell;
use crate::data::data_structures::Maze;

use std::thread;
use std::time;

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
use console_engine::KeyCode;

pub struct MazeScene {
    pub maze: Maze,
    pub x: i32,
    pub y: i32,
    pub cell_size: i32,
    pub color_wall: Color,
    pub color_path: Color,
    pub color_visited: Color,
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
        }
    }
    pub fn draw(
        &self,
        engine: &mut console_engine::ConsoleEngine,
        colored: bool,
        random_colored: bool,
        shortest_path: Vec<Cell>,
        BFS: bool,
    ) {
        // Adjust the maze dimensions to include walls
        let new_width = self.maze.width * 2 + 1;
        let new_height = self.maze.height * 2 + 1;

        let mut laby_with_walls = vec![vec![WALL_CHAR; new_width]; new_height];

        // Construct the maze with walls around it
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let cell = self.maze.get_cell(x as i32, y as i32);

                // Draw the cell's position
                let draw_x = x * 2 + 1;
                let draw_y = y * 2 + 1;

                // Draw walls if needed
                if !cell.has_wall_north() && draw_y > 0 {
                    laby_with_walls[draw_y - 1][draw_x] = cell.c;
                }
                if !cell.has_wall_south() && draw_y < new_height - 1 {
                    laby_with_walls[draw_y + 1][draw_x] = cell.c;
                }
                if !cell.has_wall_west() && draw_x > 0 {
                    laby_with_walls[draw_y][draw_x - 1] = cell.c;
                }
                if !cell.has_wall_east() && draw_x < new_width - 1 {
                    laby_with_walls[draw_y][draw_x + 1] = cell.c;
                }

                // Mark visited cells or path
                if cell.visited {
                    laby_with_walls[draw_y][draw_x] = VISITED_CHAR;
                } else {
                    laby_with_walls[draw_y][draw_x] = EMPTY_CHAR;
                }
            }
        }

        // Finally, render the maze to the console
        (0..new_height).for_each(|y| {
            for x in 0..new_width {
                let ch = laby_with_walls[y][x];

                let pixel_char;
                // Draw based on mode (colored or not)
                if colored {
                    pixel_char = match ch {
                        WALL_CHAR => pixel::pxl_bg(' ', self.color_wall),
                        VISITED_CHAR => pixel::pxl_bg(' ', self.color_visited),
                        _ => pixel::pxl_bg(' ', self.color_path),
                    };
                } else {
                    // Use different colors based on content
                    pixel_char = match ch {
                        WALL_CHAR => pixel::pxl_fg(WALL_CHAR, self.color_wall),
                        VISITED_CHAR => pixel::pxl_fg(VISITED_CHAR, self.color_visited),
                        _ => pixel::pxl_fg(PATH_CHAR, self.color_path),
                    };
                }
                for i in 0..self.cell_size {
                    //
                    //        if sizeX % 2 == 0 and x == sizeX - 1:
                    //            return
                    //        elif sizeY % 2 == 0 and y == sizeY - 1:
                    //            return
                    engine.set_pxl(self.x + (x * 2) as i32 + i, self.y + y as i32, pixel_char);
                }
            }
        });
    }

    fn choose_color(&self, ch: char) -> Color {
        match ch {
            WALL_CHAR => self.color_wall,
            VISITED_CHAR => self.color_visited,
            _ => self.color_path,
        }
    }
}
