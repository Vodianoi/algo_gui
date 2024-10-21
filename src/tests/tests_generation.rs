#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    // Mock Maze struct
    #[derive(Clone, Debug)]
    pub struct MockMaze {
        pub width: usize,
        pub height: usize,
        pub walls_removed: Vec<((i32, i32), (i32, i32))>, // Tracks removed walls for verification
    }

    impl MockMaze {
        pub fn new(width: usize, height: usize) -> Self {
            MockMaze {
                width,
                height,
                walls_removed: Vec::new(),
            }
        }

        pub fn remove_wall(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
            self.walls_removed.push(((x1, y1), (x2, y2)));
        }

        pub fn has_wall_removed(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
            self.walls_removed.contains(&((x1, y1), (x2, y2)))
        }
    }

    // Mock MazeScene struct
    #[derive(Clone, Debug)]
    pub struct MockMazeScene {
        pub maze: MockMaze,
    }

    impl MockMazeScene {
        pub fn new(maze: MockMaze) -> Self {
            MockMazeScene { maze }
        }

        pub fn draw(
            &self,
            _engine: &mut console_engine::ConsoleEngine,
            _arg1: bool,
            _arg2: bool,
            _arg3: Vec<()>,
            _arg4: bool,
        ) {
            // Mock the draw function
        }
    }

    // Helper function to set up the algorithm runner
    fn setup_algorithm_runner(algorithm: Box<dyn Algorithm>) -> AlgorithmRunner {
        let maze = MockMaze::new(5, 5); // 5x5 test maze
        let scene = MockMazeScene::new(maze.clone());
        AlgorithmRunner::new(algorithm, maze, scene)
    }

    #[test]
    fn test_recursive_backtracker_runs() {
        let algorithm = Box::new(RecursiveBacktracker);
        let runner = setup_algorithm_runner(algorithm);

        runner.start();
        thread::sleep(Duration::from_secs(2)); // Allow the algorithm to run for 2 seconds
        runner.stop();

        let maze = runner.maze.lock().unwrap();
        assert!(
            !maze.walls_removed.is_empty(),
            "RecursiveBacktracker should remove walls."
        );
    }

    #[test]
    fn test_kruskal_algorithm_runs() {
        let algorithm = Box::new(KruskalAlgorithm);
        let runner = setup_algorithm_runner(algorithm);

        runner.start();
        thread::sleep(Duration::from_secs(2)); // Allow the algorithm to run for 2 seconds
        runner.stop();

        let maze = runner.maze.lock().unwrap();
        assert!(
            !maze.walls_removed.is_empty(),
            "KruskalAlgorithm should remove walls."
        );
    }

    #[test]
    fn test_recursive_backtracker_removes_walls() {
        let algorithm = Box::new(RecursiveBacktracker);
        let runner = setup_algorithm_runner(algorithm);

        runner.start();
        thread::sleep(Duration::from_secs(2)); // Allow the algorithm to run for 2 seconds
        runner.stop();

        let maze = runner.maze.lock().unwrap();
        assert!(
            maze.has_wall_removed(0, 0, 1, 0),
            "RecursiveBacktracker should remove wall between (0, 0) and (1, 0)."
        );
    }

    #[test]
    fn test_kruskal_algorithm_removes_walls() {
        let algorithm = Box::new(KruskalAlgorithm);
        let runner = setup_algorithm_runner(algorithm);

        runner.start();
        thread::sleep(Duration::from_secs(2)); // Allow the algorithm to run for 2 seconds
        runner.stop();

        let maze = runner.maze.lock().unwrap();
        assert!(
            maze.has_wall_removed(0, 0, 1, 0),
            "KruskalAlgorithm should remove wall between (0, 0) and (1, 0)."
        );
    }
}
