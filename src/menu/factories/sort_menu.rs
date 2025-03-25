use std::sync::Arc;

use crate::algorithms::sorting::*;
use crate::data::data_structures::{Runnable, Runner, SortingContext};
use crate::menu::{
    alignment::Alignment, items::button::Button, items::dropdown::Dropdown, menu::Menu, menu_item::MenuItem,
};
use crate::scenes::sort_scene::SortScene;
use std::sync::Mutex;
use console_engine::ConsoleEngine;

pub fn run_sort_menu(engine: &mut ConsoleEngine) {
    let sort_items = get_sort_items();
    let sort_dropdown = create_sort_dropdown(&sort_items);
    let start_button = create_start_button();

    let menu_items: Vec<Box<dyn MenuItem>> = vec![sort_dropdown, start_button];
    let (menu_width, menu_height, display_width, display_height, x, y) = calculate_dimensions(engine);

    let mut sort_menu = Menu::new(0, 0, menu_width, menu_height, menu_items, Alignment::Center);

    main_loop(engine, &mut sort_menu, display_width as usize, display_height as usize, x, y);
}

fn get_sort_items() -> Vec<String> {
    vec![
        "Bubble Sort".to_string(),
        "Quick Sort".to_string(),
        "Merge Sort".to_string(),
        "Heap Sort".to_string(),
        "Shell Sort".to_string(),
        "Insertion Sort".to_string(),
        "Selection Sort".to_string(),
    ]
}

fn create_sort_dropdown(sort_items: &[String]) -> Box<Dropdown> {
    Box::new(Dropdown {
        x: 5,
        y: 5,
        width: 22,
        options: sort_items.to_vec(),
        selected_index: 0,
        is_open: false,
        selected: false,
    })
}

fn create_start_button() -> Box<Button> {
    Box::new(Button {
        x: 5,
        y: 15,
        width: 20,
        height: 3,
        label: "Start".to_string(),
        selected: false,
    })
}

fn calculate_dimensions(engine: &ConsoleEngine) -> (i32, i32, u32, u32, i32, i32) {
    let screen = engine.get_screen();
    let menu_width = screen.get_width() as i32 / 4;
    let menu_height = screen.get_height() as i32 - 2;
    let display_width = screen.get_width() * 3 / 4 - 2;
    let display_height = screen.get_height() - 2;
    let x = screen.get_width() as i32 - display_width as i32;
    let y = -2;
    (menu_width, menu_height, display_width, display_height, x, y)
}

fn main_loop(
    engine: &mut ConsoleEngine,
    sort_menu: &mut Menu,
    display_width: usize,
    display_height: usize,
    x: i32,
    y: i32,
) {
    loop {
        engine.wait_frame();
        engine.clear_screen();

        sort_menu.draw(engine);
        sort_menu.handle_input(engine);
        sort_menu.handle_key_event(engine);

        if sort_menu._quit {
            break;
        }

        if sort_menu.confirmed() {
            sort_menu.set_confirmed(false);
            handle_sorting(engine, sort_menu, display_width, display_height, x, y);
        }

        engine.draw();
    }
}

fn handle_sorting(
    engine: &mut ConsoleEngine,
    sort_menu: &mut Menu,
    display_width: usize,
    display_height: usize,
    x: i32,
    y: i32,
) {
    let form_values = sort_menu.get_values();
    let selected_algorithm = form_values[0].clone();

    let data = generate_dataset(display_width, display_height);
    let algorithm = get_sorting_algorithm(&selected_algorithm);
    let scene = Arc::new(Mutex::new(SortScene::new(data.clone(), x, y, 2)));
    let mut runner = Runner::new(vec![algorithm], scene, data);
    let running = runner.running.clone();

    runner.start();

    while running.load(std::sync::atomic::Ordering::Relaxed) {
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

        // Render the sorting scene with highlighted indices
        runner.render(engine); // Pass indices to highlight here

        engine.draw();
    }
}

fn generate_dataset(display_width: usize, display_height: usize) -> Vec<i32> {
    (0..display_width / 2)
        .map(|_| (rand::random::<u32>() % (display_height as u32)) + 1)
        .map(|value| value as i32)
        .collect()
}

fn get_sorting_algorithm(name: &str) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
    match name {
        "Bubble Sort" => Box::new(BubbleSort),
        "Selection Sort" => Box::new(SelectionSort),
        "Insertion Sort" => Box::new(InsertionSort),
        "Merge Sort" => Box::new(MergeSort),
        "Quick Sort" => Box::new(QuickSort),
        "Heap Sort" => Box::new(HeapSort),
        "Shell Sort" => Box::new(ShellSort),
        _ => Box::new(BubbleSort),
    }
}
