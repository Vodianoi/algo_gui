mod algorithms {
    pub mod maze_generation;
    pub mod pathfinding;
    pub mod sorting;
}

mod helpers {
    pub mod engine_helpers;
}

mod menu {
    pub mod button;
    pub mod main_menu;
    pub mod maze_menu;
    pub mod menu;
    pub mod pathfinding_menu;
    pub mod sort_menu;
}

fn main() {
    menu::main_menu::main_menu();
}
