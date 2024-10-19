use crate::algorithms::sorting::*;
use std::thread;

pub fn run_sort_menu() {
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

    println!("Sort Algorithms Menu:");
    for (i, item) in sort_items.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }

    let handle = thread::spawn(move || {
        bubble_sort();
    });

    handle.join().unwrap();
}
