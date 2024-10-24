use crate::algorithms::sorting::*;
use crate::menu::{
    alignment::Alignment, button::Button, dropdown::Dropdown, menu::Menu, menu_item::MenuItem,
};
use console_engine::KeyCode;

pub fn run_sort_menu(engine: &mut console_engine::ConsoleEngine) {
    // Define the dropdown items for sorting algorithms
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

    // Create the dropdown menu item for sorting algorithms
    let sort_dropdown = Box::new(Dropdown {
        x: 0,
        y: 0,
        width: 30,
        options: sort_items,
        selected_index: 0,
        is_open: false,
        selected: false,
    });

    // Create the "Back" button to return to the previous menu
    let back_button = Box::new(Button {
        x: 0,
        y: 0,
        width: 20,
        height: 3,
        label: "Back".to_string(),
        selected: false,
    });

    // Create the main menu with the dropdown and back button
    let menu_items: Vec<Box<dyn MenuItem>> = vec![sort_dropdown, back_button];

    let mut sort_menu = Menu::new(10, 10, 30, 10, menu_items, Alignment::Center);

    loop {
        engine.clear_screen();
        engine.wait_frame();

        // Draw and handle input for the menu
        sort_menu.draw(engine);
        sort_menu.handle_input(engine);

        // Exit the loop if the user requests to quit
        if sort_menu._quit {
            break;
        }

        // Handle confirmation when the user presses "Enter"
        if sort_menu.confirmed() {
            match sort_menu.get_selected() {
                0 => {
                    // Sort dropdown confirmed
                    let values = sort_menu.get_values();
                    let selected_algorithm = values.get(0).unwrap();

                    match selected_algorithm.as_str() {
                        "Bubble Sort" => bubble_sort(),
                        "Quick Sort" => quick_sort(),
                        "Quick3 Sort" => quick3_sort(),
                        "Merge Sort" => merge_sort(),
                        "Heap Sort" => heap_sort(),
                        "Shell Sort" => shell_sort(),
                        "Insertion Sort" => insertion_sort(),
                        "Selection Sort" => selection_sort(),
                        _ => bubble_sort(),
                    }
                }
                1 => break, // Back button selected
                _ => {}
            }

            // Reset the confirmation state after handling it
            sort_menu.set_confirmed(false);
        }

        engine.draw();
    }
}
