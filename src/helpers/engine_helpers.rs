/// Console engine helpers
use console_engine::ConsoleEngine;
/// Print debug info using console_engine
pub fn print_debug_info(engine: &mut ConsoleEngine, info: &str, x: i32, y: i32) {
    engine.print(x, y, info);
    engine.draw();
}

pub fn print_framerate(engine: &mut ConsoleEngine) {
    let frame_count = engine.frame_count;
    engine.print(0, 0, &format!("Frames: {}", frame_count));
}
