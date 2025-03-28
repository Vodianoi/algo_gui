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
    pub mod menu;
    pub mod menu_item;
    pub mod theme;

    pub mod factories {
        pub mod main_menu;
        pub mod maze_menu;
        pub mod sort_menu;
        pub mod pathfinding_menu;
    }

    pub mod items {
        pub mod button;
        pub mod dropdown;
        pub mod dropdown_menu;
        pub mod text;
    }

    pub mod utils {
        pub mod menu_utils;
    }
}


mod scenes {
    pub mod maze_scene;
    pub mod sort_scene;
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
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    if let Err(e) = menu::factories::main_menu::main_menu() {
        eprintln!("Error: {}", e);
    }
}
