use std::io::{self};
use crate::menu::menu::Menu;

use super::{maze_menu::run_maze_menu, sort_menu::run_sort_menu};

pub fn main_menu() -> io::Result<()> {
    let mut menu = Menu::new(
        "Main Menu",
        vec!["Algorithm Visualizer", "Maze Generation", "Sorting", "Quit"],
    );

    menu.run(|selected_index, _running| {
        match selected_index {
            1 => run_maze_menu()?, // Maze Generation
            2 => run_sort_menu()?, // Sorting
            _ => {}
        }
        Ok(())
    })
}