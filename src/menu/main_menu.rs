use crate::menu::{
    maze_menu::run_maze_menu, menu::Menu, pathfinding_menu::run_pathfinding_menu,
    sort_menu::run_sort_menu,
};

use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use termsize;
pub fn main_menu() {
    // Take the screen width and height from the engine, take full terminal size
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let screen_width = cols as u32;
    let screen_height = rows as u32;
    let mut engine = ConsoleEngine::init(screen_width, screen_height, 60).unwrap();

    let menu_items = vec!["Sort Algorithms", "Maze Generation Algorithms", "Exit"];

    let mut menu = Menu::new(10, 10, 10, 5, menu_items);
    loop {
        engine.wait_frame();
        engine.clear_screen();
        menu.draw(&mut engine);
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        if engine.is_key_pressed(KeyCode::Up)
            || engine.is_key_pressed_with_modifier(
                KeyCode::Tab,
                KeyModifiers::SHIFT,
                console_engine::KeyEventKind::Press,
            )
            || engine.is_key_pressed(KeyCode::Char('k'))
        {
            menu.previous();
        }
        if engine.is_key_pressed(KeyCode::Down)
            || engine.is_key_pressed(KeyCode::Tab)
            || engine.is_key_pressed(KeyCode::Char('j'))
        {
            menu.next();
        }
        if engine.is_key_pressed(KeyCode::Enter) || engine.is_key_pressed(KeyCode::Char(' ')) {
            match menu.selected {
                0 => run_sort_menu(&mut engine),
                1 => run_maze_menu(&mut engine),
                2 => break,
                _ => (),
            }
        }
        engine.draw();
    }
}

// Print framerate
