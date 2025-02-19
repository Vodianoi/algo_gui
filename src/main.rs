mod algorithms {
    pub mod maze_generation;
    pub mod pathfinding;
    pub mod sorting;
}

mod helpers {
    pub mod engine_helpers;
}

mod data {
    pub mod data_structures;
}

mod menu {
    pub mod button;
    pub mod dropdown;
    pub mod form;
    pub mod main_menu;
    pub mod maze_menu;
    pub mod maze_scene;
    pub mod menu;
    pub mod menu_handler;
    pub mod menu_trait;
    pub mod pathfinding_menu;
    pub mod sort_menu;
    pub mod theme;
}

fn main() {
    menu::main_menu::main_menu();
}
