use std::sync::{Arc, Mutex};

use console_engine::ConsoleEngine;

use crate::scenes::sort_scene::SortScene;

pub trait SortingAlgorithm: Send + Sync {
    fn sort(&self, scene: SharedSortScene, running: SharedBool);
    fn clone_box(&self) -> Box<dyn SortingAlgorithm>;
}

impl<T> SortingAlgorithm for T
where
    T: 'static + Send + Sync + Clone + Fn(SharedSortScene, SharedBool),
{
    fn sort(&self, scene: SharedSortScene, running: SharedBool) {
        self(scene, running);
    }


    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}

pub struct SortingRunner {
    algorithm: Box<dyn SortingAlgorithm>,
    scene: SharedSortScene,
    pub running: SharedBool,
}

impl SortingRunner {
    pub fn new(algorithm: Box<dyn SortingAlgorithm>, scene: SortScene) -> Self {
        Self {
            algorithm,
            scene: SharedSortScene::new(scene),
            running: SharedBool::new(true),
        }
    }

    pub fn start(&self) {
        let scene = self.scene.clone();
        let running = self.running.clone();
        let algorithm = self.algorithm.clone_box();

        // Start the sorting thread
        algorithm.sort(scene, running);
    }

    pub fn stop(&self) {
        self.running.set(false);
    }

    pub fn get_data(&self) -> Vec<i32> {
        self.scene.get_data()
    }

    pub fn render(&self, engine: &mut ConsoleEngine) {
        self.scene.render(engine);
    }
}

impl Clone for SortingRunner {
    fn clone(&self) -> Self {
        Self {
            algorithm: self.algorithm.clone_box(),
            scene: self.scene.clone(),
            running: self.running.clone(),
        }
    }
}

#[derive(Clone)]
pub struct BubbleSort;

impl SortingAlgorithm for BubbleSort {
    fn sort(&self, scene: SharedSortScene, running: SharedBool) {
        let mut data = scene.get_data();
        for i in 0..data.len() {
            for j in 0..data.len() - i - 1 {
                if !running.get() {
                    return; // Stop sorting if the `running` flag is set to false
                }

                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }

                // Update the scene data for visualization
                scene.update(data.clone());
            }
        }
    }

    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct SelectionSort;

impl SortingAlgorithm for SelectionSort {
    fn sort(&self, scene: SharedSortScene, running: SharedBool) {
        let mut data = scene.get_data();
        for i in 0..data.len() {
            let mut min_index = i;
            for j in i + 1..data.len() {
                if !running.get() {
                    return; // Stop sorting if the `running` flag is set to false
                }

                if data[j] < data[min_index] {
                    min_index = j;
                }
            }

            data.swap(i, min_index);

            // Update the scene data for visualization
            scene.update(data.clone());
        }
    }

    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}

pub struct SharedSortScene(Arc<Mutex<SortScene>>);

impl SharedSortScene {
    pub fn new(scene: SortScene) -> Self {
        Self(Arc::new(Mutex::new(scene)))
    }

    pub fn get_data(&self) -> Vec<i32> {
        self.0.lock().unwrap().data.lock().unwrap().clone()
    }

    pub fn update(&self, data: Vec<i32>) {
        self.0.lock().unwrap().update(data);
    }

    pub fn render(&self, engine: &mut ConsoleEngine) {
        self.0.lock().unwrap().draw(engine);
    }
}

impl Clone for SharedSortScene {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub struct SharedBool(Arc<Mutex<bool>>);

impl SharedBool {
    pub fn new(value: bool) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }

    pub fn get(&self) -> bool {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, value: bool) {
        let mut lock = self.0.lock().unwrap();
        *lock = value;
    }
}

impl Clone for SharedBool {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
