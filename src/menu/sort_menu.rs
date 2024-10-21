use crate::algorithms::sorting::*;

use crate::menu::menu::Menu;
use console_engine::KeyCode;

use super::menu_trait::MenuTrait;

pub fn run_sort_menu(engine: &mut console_engine::ConsoleEngine) {
    let sort_items = vec![
        "Bubble Sort".to_string(),
        "Quick Sort".to_string(),
        "Quick3 Sort".to_string(),
        "Merge Sort".to_string(),
        "Heap Sort".to_string(),
        "Shell Sort".to_string(),
        "Insertion Sort".to_string(),
        "Selection Sort".to_string(),
    ];

    let mut sort_menu = Menu {
        x: 10,
        y: 10,
        w: 20,
        h: 5,
        items: sort_items,
        selected: 0,
        _quit: false,
    };
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
