// src/menu/menu_item.rs

pub trait MenuItem: Send{
    fn draw(&self, engine: &mut console_engine::ConsoleEngine);
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn is_mouse_over(&self, x: i32, y: i32) -> bool;
    fn draw_ascii(&self, engine: &mut console_engine::ConsoleEngine);
    fn draw_standard(&self, engine: &mut console_engine::ConsoleEngine);
    fn handle_input(&mut self, engine: &mut console_engine::ConsoleEngine) -> bool;
    fn is_selectable(&self) -> bool;
    fn as_any(&self) -> &dyn std::any::Any; // For downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn MenuItem>;
}

impl Clone for Box<dyn MenuItem> {
    fn clone(&self) -> Box<dyn MenuItem> {
        self.clone_box()
    }
}