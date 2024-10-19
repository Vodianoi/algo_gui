use crate::menu::{
    maze_menu::run_maze_menu, menu::Menu, pathfinding_menu::run_pathfinding_menu,
    sort_menu::run_sort_menu,
};

use console_engine::{ConsoleEngine, KeyCode};
use termsize;
pub fn main_menu() {
    // Take the screen width and height from the engine, take full terminal size
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let screen_width = cols as u32;
    let screen_height = rows as u32;
    let mut engine = ConsoleEngine::init(screen_width, screen_height, 60).unwrap();

    let menu_items = vec![
        "Sort Algorithms",
        "Pathfinding Algorithms",
        "Maze Generation Algorithms",
        "Exit",
    ];

    let mut menu = Menu::new(10, 10, 20, 5, menu_items);
    let mut count: i32 = 0;
    loop {
        engine.clear_screen();
        menu.draw(&mut engine);
        print!("{}", count);
        count += 1;
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        if engine.is_key_pressed(KeyCode::Up) {
            menu.previous();
        }
        if engine.is_key_pressed(KeyCode::Down) {
            menu.next();
        }
        if engine.is_key_pressed(KeyCode::Enter) {
            match menu.selected {
                0 => run_sort_menu(),
                1 => run_pathfinding_menu(),
                2 => run_maze_menu(),
                3 => break,
                _ => (),
            }
        }
        engine.draw();
    }
}

// Print framerate
