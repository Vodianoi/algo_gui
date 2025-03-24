use console_engine::{pixel, Color, ConsoleEngine};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SortScene {
    pub data: Arc<Mutex<Vec<i32>>>,
    pub x: i32,
    pub y: i32,
    pub bar_width: i32,
    pub bar_color: Color,
    pub buffer: VecDeque<Vec<i32>>,
    pub hightlight_buffer: VecDeque<Vec<usize>>,
    pub highlight_color_buffer: VecDeque<Color>,
    last_frame: Option<Vec<i32>>,
    last_highlighted: Vec<usize>,
    last_highlight_color: Color,
}

impl SortScene {
    /// Creates a new `SortScene` with the given data and bar width.
    pub fn new(data: Vec<i32>, bar_width: i32, x: i32, y: i32) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
            x,
            y,
            bar_width,
            bar_color: Color::White,
            buffer: VecDeque::new(),
            hightlight_buffer: VecDeque::new(),
            highlight_color_buffer: VecDeque::new(),
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
                let highlight_indices = &self.last_highlighted;
                let highlight_color = self.last_highlight_color;
                self.draw_frame(
                    engine,
                    last_frame,
                    screen_height,
                    self.bar_color,
                    highlight_indices,
                    highlight_color, // Highlight color
                );
            }
            return;
        }

        // Pop the next frame from the buffer and draw it
        if let Some(frame) = self.buffer.pop_front() {
            self.last_frame = Some(frame.clone());
            let highlight_indices = self.hightlight_buffer.pop_front().unwrap_or_default();
            let highlight_indices = highlight_indices.as_slice();
            let highlight_color = self.highlight_color_buffer.pop_front().unwrap_or(Color::Red);
            self.draw_frame(
                engine,
                &frame,
                screen_height,
                self.bar_color,
                highlight_indices,
                highlight_color,
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
        highlight_indices: &[usize],
        highlight_color: Color,
    ) {
        let max_value = *frame.iter().max().unwrap_or(&1);

        for (i, &value) in frame.iter().enumerate() {
            let bar_height = (value as f32 / max_value as f32 * screen_height as f32) as i32;
            let bar_color = if highlight_indices.contains(&i) {
                highlight_color
            } else {
                color
            };

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
    pub fn update(&mut self, data: Vec<i32>, highlight_indices: &[usize], highlight_color: Color) {
        self.buffer.push_back(data);
        self.hightlight_buffer.push_back(highlight_indices.to_vec());
        self.highlight_color_buffer.push_back(highlight_color);
    }
}
