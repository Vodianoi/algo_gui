use crate::algorithms::sorting::*;

use crate::menu::menu::Menu;
use console_engine::KeyCode;

pub fn run_sort_menu(engine: &mut console_engine::ConsoleEngine) {
    let sort_items = vec![
        "Bubble Sort",
        "Quick Sort",
        "Quick3 Sort",
        "Merge Sort",
        "Heap Sort",
        "Shell Sort",
        "Insertion Sort",
        "Selection Sort",
    ];

    let mut sort_menu = Menu::new(10, 10, 20, 5, sort_items);
    loop {
        engine.clear_screen();
        engine.wait_frame();

        sort_menu.draw(engine);
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        if engine.is_key_pressed(KeyCode::Up) {
            sort_menu.previous();
        }
        if engine.is_key_pressed(KeyCode::Down) {
            sort_menu.next();
        }
        if engine.is_key_pressed(KeyCode::Enter) {
            match sort_menu.selected {
                0 => bubble_sort(),
                1 => quick_sort(),
                2 => quick3_sort(),
                3 => merge_sort(),
                4 => heap_sort(),
                5 => shell_sort(),
                6 => insertion_sort(),
                7 => selection_sort(),
                _ => (),
            }
        }
        engine.draw();
    }
}
