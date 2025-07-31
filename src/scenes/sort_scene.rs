use ratatui::widgets::{BarChart, Block, Borders};

use crate::data::data_structures::FrameType;

use crate::data::data_structures::{Scene, SortHighlight, SortingContext};

pub struct SortScene {
    data: Vec<i32>,
    highlights: Vec<SortHighlight>,
}

impl SortScene {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            highlights: Vec::new(),
        }
    }
}

impl Scene<Vec<i32>, SortingContext> for SortScene {
    fn render(&mut self, f: &mut FrameType<'_>) {
        let area = f.size();
        let max = self.data.iter().copied().max().unwrap_or(1) as u64;
        let items: Vec<(&str, u64)> = self
            .data
            .iter()
            .enumerate()
            .map(|(_i, v)| ("", *v as u64))
            .collect();
        let barchart = BarChart::default()
            .block(Block::default().title("Sorting").borders(Borders::ALL))
            .data(&items)
            .bar_width(2)
            .max(max);
        f.render_widget(barchart, area);
    }

    fn update(&mut self, data: &Vec<i32>, context: &SortingContext) {
        self.data = data.clone();
        self.highlights = context.highlights.clone();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
