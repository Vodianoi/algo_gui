use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::algorithms::sorting::*;
use crate::menu::{
    alignment::Alignment, button::Button, dropdown::Dropdown, menu::Menu, menu_item::MenuItem,
};
use crate::scenes::sort_scene::SortScene;
use console_engine::ConsoleEngine;
use termsize;

pub fn run_sort_menu(engine: &mut ConsoleEngine) {
    // Define the dropdown items for sorting algorithms
    let sort_items = vec![
        "Bubble Sort".to_string(),
        "Quick Sort".to_string(),
        "Merge Sort".to_string(),
        "Heap Sort".to_string(),
        "Shell Sort".to_string(),
        "Insertion Sort".to_string(),
        "Selection Sort".to_string(),
    ];

    // Create the dropdown menu item for sorting algorithms
    let sort_dropdown = Box::new(Dropdown {
        x: 5,
        y: 5,
        width: 22,
        options: sort_items,
        selected_index: 0,
        is_open: false,
        selected: false,
    });

    // Create the "Start" button to begin the sorting visualization
    let start_button = Box::new(Button {
        x: 5,
        y: 15,
        width: 20,
        height: 3,
        label: "Start".to_string(),
        selected: false,
    });

    // Create the main menu with the dropdown and start button
    let menu_items: Vec<Box<dyn MenuItem>> = vec![sort_dropdown, start_button];
    let screen_size = termsize::get().unwrap();
    let menu_width = screen_size.cols as i32 / 4;
    let menu_height = screen_size.rows as i32 - 2;
    let display_width = screen_size.cols * 3 / 4 - 2;
    let display_height = screen_size.rows;
    let x = screen_size.cols as i32 - display_width as i32;
    let y = -2;

    let nb_values = 20;

    let mut sort_menu = Menu::new(0, 0, menu_width, menu_height, menu_items, Alignment::Left);

    loop {
        engine.wait_frame();
        engine.clear_screen();

        // Draw and handle input for the menu
        sort_menu.draw(engine);
        sort_menu.handle_input(engine);
        sort_menu.handle_key_event(engine);

        // Exit the loop if the user requests to quit
        if sort_menu._quit {
            break;
        }

        // Handle "Start" button confirmation
        if sort_menu.confirmed() {
            sort_menu.set_confirmed(false);

            let form_values = sort_menu.get_values();
            let selected_algorithm = form_values[0].clone();

            // Generate a random dataset for sorting
            let data = (0..nb_values).map(|_| rand::random::<i32>() % 100).collect();

            // Run the sorting scene with the selected algorithm
            // run_sort_scene(engine, &selected_algorithm, data);

            let algorithm_name = selected_algorithm.as_str();
            // Map algorithm names to their corresponding implementations
            let algorithm: Box<dyn SortingAlgorithm> = match algorithm_name {
                "Bubble Sort" => Box::new(BubbleSort),
                // Add other sorting algorithms here
                // "Quick Sort" => Box::new(QuickSort),
                // "Merge Sort" => Box::new(MergeSort),
                // "Heap Sort" => Box::new(HeapSort),
                // "Shell Sort" => Box::new(ShellSort),
                // "Insertion Sort" => Box::new(InsertionSort),
                // "Selection Sort" => Box::new(SelectionSort),
                _ => Box::new(BubbleSort), // Default to Bubble Sort
            };

            // Initialize the sorting scene
            let scene = SortScene::new(data, 2, x, y);
            let runner = SortingRunner::new(algorithm, scene);
            let running =  Arc::clone(&runner.running);
            
            runner.start();

            // Render the maze while the algorithm is running
            while running.lock().unwrap().clone() {
                engine.wait_frame();
                engine.clear_screen();

                sort_menu.draw(engine);
                sort_menu.handle_input(engine);
                sort_menu.handle_key_event(engine);

                if sort_menu._quit || sort_menu.confirmed() {
                    sort_menu._quit = false;
                    runner.stop();
                    break;
                }

                runner.render(engine);

                engine.draw();
            }
        }

        engine.draw();
    }
}
