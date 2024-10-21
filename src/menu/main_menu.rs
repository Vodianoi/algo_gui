use super::menu_handler::MenuHandler;
use crate::menu::{
    maze_menu::run_maze_menu, menu::Menu, pathfinding_menu::run_pathfinding_menu,
    sort_menu::run_sort_menu,
};
use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use termsize;

pub fn main_menu() {
    // Get terminal size to dynamically adjust screen width and height
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let screen_width = cols as u32;
    let screen_height = rows as u32;

    // Initialize the console engine with the full terminal size and 60 FPS
    let mut engine = ConsoleEngine::init(screen_width, screen_height, 240).unwrap();

    // Define the main menu options
    let menu_items = vec![
        "Maze Generation".to_string(),
        "Sorting".to_string(),
        "Quit".to_string(),
    ];

    // Create a new Menu instance
    let mut menu = Menu {
        x: 5,
        y: 5,
        w: 20,
        h: 1,
        items: menu_items,
        selected: 0,
        _quit: false,
    };
    // Here we create a handler that implements `MenuTrait`.
    //     pub fn new(menu: Box<dyn MenuTrait>) -> Self {
    //    MenuHandler { menu }
    //}
    let mut menu_handler = MenuHandler::new(Box::new(menu));

    // Main loop for handling the menu
    loop {
        engine.wait_frame(); // Wait for the next frame
        engine.clear_screen(); // Clear the screen

        // Handle the input with the menu handler (implemented via `MenuTrait`)
        menu_handler.handle_input(&mut engine);

        // Handle the menu selection
        if menu_handler.should_quit {
            break;
        }
        if engine.is_key_pressed(KeyCode::Enter) {
            match menu_handler.get_selected() {
                0 => run_maze_menu(&mut engine),
                1 => run_sort_menu(&mut engine),
                2 => break,
                _ => (),
            }
        }

        // Draw the menu to the screen
        menu_handler.draw(&mut engine);

        // Render the current frame
        engine.draw();
    }
}
