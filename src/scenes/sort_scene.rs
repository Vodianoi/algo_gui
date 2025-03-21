use console_engine::{pixel, ConsoleEngine, Color};
use std::sync::{Arc, Mutex};

pub struct SortScene {
    pub data: Arc<Mutex<Vec<i32>>>,
    pub x: i32,
    pub y: i32,
    pub bar_width: i32,
    pub bar_color: Color,
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
        }
    }


    /// Draws the current state of the sorting visualization.
    pub fn draw(&self, engine: &mut ConsoleEngine) {
        let data = self.data.lock().unwrap();
        let max_value = *data.iter().max().unwrap_or(&1);
        let screen_height = engine.get_height();

        for (i, &value) in data.iter().enumerate() {
            let bar_height = (value as f32 / max_value as f32 * screen_height as f32) as i32;
            for y in 0..bar_height {
                engine.set_pxl(
                    self.x + i as i32 * self.bar_width,
                    self.y + (screen_height as i32) - y - 1,
                    pixel::pxl_bg(' ', self.bar_color),
                );
            }
        }
    }
}