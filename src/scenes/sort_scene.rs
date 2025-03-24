use console_engine::{pixel, ConsoleEngine, Color};
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread, time::Duration};

pub struct SortScene {
    pub data: Arc<Mutex<Vec<i32>>>,
    pub x: i32,
    pub y: i32,
    pub bar_width: i32,
    pub bar_color: Color,
    pub buffer: VecDeque<Vec<i32>>,
    last_frame: Option<Vec<i32>>,
}

impl SortScene {
    /// Creates a new `SortScene` with the given data and bar width.
    pub fn new(data: Vec<i32>, bar_width: i32, x: i32, y:i32) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
            x,
            y,
            bar_width,
            bar_color: Color::White,
            buffer: VecDeque::new(),
            last_frame: None,
        }
    }

    /// Draws the next frame from the buffer if available.
    /// This method renders one frame per call without blocking.
    pub fn draw(&mut self, engine: &mut ConsoleEngine) {
        let screen_height = engine.get_height() as usize;

        // If the buffer is empty, draw the last frame if it exists
        if self.buffer.is_empty() {
            if let Some(last_frame) = &self.last_frame {
                self.draw_frame(engine, last_frame, screen_height, self.bar_color);
            }
            return;
        }

        // Pop the next frame from the buffer and draw it
        if let Some(frame) = self.buffer.pop_front() {
            self.last_frame = Some(frame.clone());
            self.draw_frame(engine, &frame, screen_height, self.bar_color);
        }
    }

    /// Helper method to draw a frame on the console engine.
    fn draw_frame(&self, engine: &mut ConsoleEngine, frame: &[i32], screen_height: usize, color: Color) {
        let max_value = *frame.iter().max().unwrap_or(&1);

        for (i, &value) in frame.iter().enumerate() {
            let bar_height = (value as f32 / max_value as f32 * screen_height as f32) as i32;
            for y in 0..bar_height {
                engine.set_pxl(
                    self.x + i as i32 * self.bar_width,
                    self.y + (screen_height as i32) - y - 1,
                    pixel::pxl_bg(' ', color),
                );
            }
        }
    }

    /// Updates the scene data with the given data.
    /// This method is used to update the scene data during sorting.
    /// The `buffer` field is used to store the data temporarily before updating the scene data.
    pub fn update(&mut self, data: Vec<i32>) {
        self.buffer.push_back(data);
    }
}