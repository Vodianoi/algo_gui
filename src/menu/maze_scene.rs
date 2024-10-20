// Maze scene is a simple scene that displays the animation of the maze generation algorithm.

use crate::algorithms::maze_generation::*;
use crate::data::data_structures::Maze;

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
            maze: maze,
            x: x,
            y: y,
            cell_size: cell_size,
            color_wall: Color::Black,
            color_path: Color::White,
            color_visited: Color::Green,
        }
    }
    pub fn draw(&self, engine: &mut console_engine::ConsoleEngine) {
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let cell = self.maze.get_cell(x as i32, y as i32);
                let color = match cell.value {
                    0 => self.color_wall,
                    1 => self.color_path,
                    2 => self.color_visited,
                    _ => Color::White,
                };
                engine.fill_rect(
                    self.x + x as i32 * self.cell_size,
                    self.y + y as i32 * self.cell_size,
                    self.cell_size,
                    self.cell_size,
                    pixel::pxl_bg(' ', color),
                );
            }
        }
    }
}
