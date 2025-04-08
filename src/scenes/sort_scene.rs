use console_engine::{pixel, Color, ConsoleEngine};
use std::
    collections::VecDeque
;

use crate::data::data_structures::{Scene, SortHighlight, SortingContext};

pub struct SortScene {
    pub x: i32,
    pub y: i32,
    pub bar_width: i32,
    pub bar_color: Color,
    pub buffer: VecDeque<Vec<i32>>,
    pub hightlight_buffer: VecDeque<Vec<SortHighlight>>,
    last_frame: Option<Vec<i32>>,
    last_highlighted: Vec<SortHighlight>,
    last_highlight_color: Color,
}

impl Scene<Vec<i32>, SortingContext> for SortScene {
    fn render(&mut self, engine: &mut ConsoleEngine) {
        self.draw(engine);
    }

    fn update(&mut self, data: &Vec<i32>, context: &SortingContext) {
        self.update(data.clone(), &context.highlights);
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SortScene {
    /// Creates a new `SortScene` with the given data and bar width.
    pub fn new(bar_width: i32, x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            bar_width,
            bar_color: Color::White,
            buffer: VecDeque::new(),
            hightlight_buffer: VecDeque::new(),
            last_frame: None,
            last_highlighted: Vec::new(),
            last_highlight_color: Color::Red,
        }
    }

    /// Draws the next frame from the buffer if available.
    /// This method renders one frame per call without blocking.
    pub fn draw(&mut self, engine: &mut ConsoleEngine) {
        let screen_height = engine.get_height() as usize;

        // If the buffer is empty, draw the last frame if it exists
        if self.buffer.is_empty() {
            if let Some(last_frame) = &self.last_frame {
                let highlights = &self.last_highlighted;
                // let highlight_color = self.last_highlight_color;
                self.draw_frame(
                    engine,
                    last_frame,
                    screen_height,
                    self.bar_color,
                    highlights,
                );
            }
            return;
        }

        // If the buffer has only one frame, render it without popping
        if self.buffer.len() == 1 {
            if let Some(frame) = self.buffer.front() {
                let default_highlight = vec![SortHighlight {
                    indices: (0..frame.len()).collect(),
                    color: self.last_highlight_color,
                }];
                let highlights = self
                    .hightlight_buffer
                    .front()
                    .unwrap_or(&default_highlight);
                // let highlight_color = *self.highlight_color_buffer.front().unwrap_or(&Color::Red);
                self.draw_frame(
                    engine,
                    frame,
                    screen_height,
                    self.bar_color,
                    highlights,
                );
            }
            return;
        }

        // Pop the next frame from the buffer and draw it
        if let Some(frame) = self.buffer.pop_front() {
            self.last_frame = Some(frame.clone());
            let highlight_indices = self.hightlight_buffer.pop_front().unwrap_or_default();
            let hightlights = highlight_indices.as_slice();
            // let highlight_color = self
            //     .highlight_color_buffer
            //     .pop_front()
            //     .unwrap_or(Color::Red);
            self.draw_frame(
                engine,
                &frame,
                screen_height,
                self.bar_color,
                hightlights,
            );
        }
    }

    /// Helper method to draw a frame on the console engine.
    fn draw_frame(
        &self,
        engine: &mut ConsoleEngine,
        frame: &[i32],
        screen_height: usize,
        color: Color,
        highlights: &[SortHighlight],
    ) {
        let max_value = *frame.iter().max().unwrap_or(&1);

        for (i, &value) in frame.iter().enumerate() {
            let bar_height = (value as f32 / max_value as f32 * screen_height as f32) as i32;
            let bar_color = highlights
                .iter()
                .find(|h| h.indices.contains(&(i as usize)))
                .map_or(color, |h| h.color);

            for y in 0..bar_height {
                engine.set_pxl(
                    self.x + i as i32 * self.bar_width,
                    self.y + (screen_height as i32) - y - 1,
                    pixel::pxl_bg(' ', bar_color),
                );
            }
        }
    }

    /// Updates the scene data with the given data.
    /// This method is used to update the scene data during sorting.
    /// The `buffer` field is used to store the data temporarily before updating the scene data.
    pub fn update(&mut self, data: Vec<i32>, highlight_indices: &[SortHighlight]) {
        self.buffer.push_back(data);
        self.hightlight_buffer.push_back(highlight_indices.to_vec());
        // self.highlight_color_buffer.push_back(highlight_color);
    }
}
