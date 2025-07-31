use ratatui::{
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::data::data_structures::FrameType;

use crate::data::data_structures::{Maze, MazeContext, Scene};

pub struct MazeScene {
    maze: Maze,
    context: MazeContext,
}

impl MazeScene {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            maze: Maze::new(width, height),
            context: MazeContext::default(),
        }
    }
}

impl Scene<Maze, MazeContext> for MazeScene {
    fn render(&mut self, f: &mut FrameType<'_>) {
        let area = f.size();
        let mut lines = Vec::new();
        for y in 0..self.maze.height {
            let mut line = String::new();
            for x in 0..self.maze.width {
                line.push(self.maze.cells[y][x].c);
            }
            lines.push(Spans::from(Span::raw(line)));
        }
        let paragraph =
            Paragraph::new(lines).block(Block::default().title("Maze").borders(Borders::ALL));
        f.render_widget(paragraph, area);
    }

    fn update(&mut self, data: &Maze, context: &MazeContext) {
        self.maze = data.clone();
        self.context = context.clone();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
