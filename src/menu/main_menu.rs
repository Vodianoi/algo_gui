use super::menu_handler::MenuHandler;
use crate::menu::{
    alignment::Alignment, button::Button, maze_menu::run_maze_menu, menu::Menu,
    pathfinding_menu::run_pathfinding_menu, sort_menu::run_sort_menu, text::Text,
};
use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use termsize;

pub fn main_menu() {
    // Get terminal size to dynamically adjust screen width and height
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let screen_width = cols as u32;
    let screen_height = rows as u32;

    // Initialize the console engine with the full terminal size and 60 FPS
    let mut engine = ConsoleEngine::init(screen_width, screen_height, 60).unwrap();

    // Define the main menu options using the new Button structure
    let menu_items: Vec<Box<dyn crate::menu::menu_item::MenuItem>> = vec![
        Box::new(Text {
            x: 0,
            y: 0,
            content: "Algorithm Visualizer".to_string(),
        }),
        Box::new(Button {
            x: 0,
            y: 0,
            width: 20,
            height: 3,
            label: "Maze Generation".to_string(),
            selected: false,
        }),
        Box::new(Button {
            x: 0,
            y: 0,
            width: 20,
            height: 3,
            label: "Sorting".to_string(),
            selected: false,
        }),
        Box::new(Button {
            x: 0,
            y: 0,
            width: 20,
            height: 3,
            label: "Quit".to_string(),
            selected: false,
        }),
    ];

    // Create a new Menu instance with centered alignment
    let mut menu = Menu::new(0, 0, 20, 10, menu_items, Alignment::Center);

    // Initialize the menu handler

    // Main loop for handling the menu
    loop {
        engine.wait_frame(); // Wait for the next frame
        engine.clear_screen(); // Clear the screen

        // Handle the input with the menu handler (implemented via `MenuTrait`)
        menu.handle_input(&mut engine);
        menu.handle_key_event(&mut engine);

        // Handle the menu selection
        if menu._quit {
            break;
        }
        if engine.is_key_pressed(KeyCode::Enter) {
            match menu.selected_index {
                1 => run_maze_menu(&mut engine), // Maze Generation
                2 => run_sort_menu(&mut engine), // Sorting
                3 => break,                      // Quit
                _ => (),
            }
        }

        // Draw the menu to the screen
        menu.draw(&mut engine);

        // Render the current frame
        engine.draw();
    }
}
