use crate::menu::menu::Menu;
use crate::algorithms::sorting::*;
use crate::data::data_structures::{Runnable, Runner, SortingContext};
use crate::scenes::sort_scene::SortScene;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io;

pub fn run_sort_menu() -> io::Result<()> {
    let mut menu = Menu::new(
        "Sorting Menu",
        vec![
            "Bubble Sort",
            "Quick Sort",
            "Merge Sort",
            "Heap Sort",
            "Shell Sort",
            "Insertion Sort",
            "Selection Sort",
            "Quit",
        ],
    );

    menu.run(|selected_index, running| {
        if selected_index == 7 {
            // Quit
            return Ok(());
        }

        // Select the sorting algorithm
        let algorithm: Box<dyn Runnable<Vec<i32>, SortingContext>> = match selected_index {
            0 => Box::new(BubbleSort),
            1 => Box::new(QuickSort),
            2 => Box::new(MergeSort),
            3 => Box::new(HeapSort),
            4 => Box::new(ShellSort),
            5 => Box::new(InsertionSort),
            6 => Box::new(SelectionSort),
            _ => return Ok(()), // Default case (shouldn't happen)
        };

        // Generate random data for sorting
        let data = (0..50).map(|_| rand::random::<i32>() % 100).collect::<Vec<_>>();

        // Create a sorting scene
        let scene = Arc::new(Mutex::new(SortScene::new(data.clone(), 2, 0, 0)));
        let mut runner = Runner::new(vec![algorithm], scene.clone(), data);

        // Start the sorting visualization
        runner.start();

        // Render the sorting visualization
        let mut engine = console_engine::ConsoleEngine::init_fill(60).unwrap();
        while runner.running.load(Ordering::SeqCst) && running.load(Ordering::SeqCst) {
            // Render the current state of the sorting process
            runner.render(&mut engine);

            engine.draw();
        }

        // Stop the runner if the user exits
        runner.running.store(false, Ordering::SeqCst);

        Ok(())
    })
}