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
    pub mod alignment;
    pub mod button;
    pub mod dropdown;
    pub mod dropdown_menu;
    pub mod form;
    pub mod main_menu;
    pub mod maze_menu;
    pub mod maze_scene;
    pub mod menu;
    pub mod menu_handler;
    pub mod menu_item;
    pub mod menu_trait;
    pub mod pathfinding_menu;
    pub mod sort_menu;
    pub mod text;
    pub mod theme;
}

mod tests {
    #[cfg(test)]
    mod tests {
        #[test]
        fn test() {
            assert_eq!(1, 1);
        }
    }
}

fn main() {
    menu::main_menu::main_menu();
}
