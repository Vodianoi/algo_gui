pub trait MenuTrait {
    fn draw(&mut self, engine: &mut console_engine::ConsoleEngine);
    fn next(&mut self);
    fn previous(&mut self);
    fn select(&self) -> Option<String>;
    fn confirmed(&self) -> bool;
    fn set_confirmed(&mut self, value: bool);
    fn get_selected(&self) -> usize;
    fn should_quit(&self) -> bool;
    fn handle_key_event(&mut self, engine: &mut console_engine::ConsoleEngine);
}
