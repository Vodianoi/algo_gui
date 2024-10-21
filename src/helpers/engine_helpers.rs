/// Console engine helpers
use console_engine::ConsoleEngine;
/// Print debug info using console_engine
pub fn print_debug_info(engine: &mut ConsoleEngine, info: &str, x: i32, y: i32) {
    engine.print(x, y, info);
    engine.draw();
}

struct Logger {
    log: Vec<String>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { log: Vec::new() }
    }
    pub fn log(&mut self, message: String) {
        self.log.push(message);
    }
    pub fn print(&self, engine: &mut ConsoleEngine, x: i32, y: i32) {
        for (i, message) in self.log.iter().enumerate() {
            engine.print(x, y + i as i32, message);
        }
    }
}

pub struct FPSCounter {
    frames: u32,
    time: std::time::Instant,
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        FPSCounter {
            frames: 0,
            time: std::time::Instant::now(),
        }
    }
    pub fn update(&mut self) {
        self.frames += 1;
        if self.time.elapsed().as_secs() >= 1 {
            self.time = std::time::Instant::now();
            self.frames = 0;
        }
    }
    pub fn get_fps(&self) -> u32 {
        self.frames
    }
}
