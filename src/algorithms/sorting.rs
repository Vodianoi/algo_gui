use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use console_engine::ConsoleEngine;

use crate::scenes::sort_scene::SortScene;

pub trait SortingAlgorithm: Send + Sync {
    fn sort(&self, scene: Arc<Mutex<SortScene>>, running: Arc<Mutex<bool>>);
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn SortingAlgorithm>;
}

impl<T> SortingAlgorithm for T
where
    T: 'static + Send + Sync + Clone + Fn(Arc<Mutex<SortScene>>, Arc<Mutex<bool>>),
{
    fn sort(&self, scene: Arc<Mutex<SortScene>>, running: Arc<Mutex<bool>>) {
        self(scene, running);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}

pub struct SortingRunner {
    algorithm: Box<dyn SortingAlgorithm>,
    scene: Arc<Mutex<SortScene>>,
    pub running: Arc<Mutex<bool>>,
}

impl SortingRunner {
    pub fn new(algorithm: Box<dyn SortingAlgorithm>, scene: SortScene) -> Self {
        Self {
            algorithm,
            scene: Arc::new(Mutex::new(scene)),
            running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn start(&self) {
        let scene = Arc::clone(&self.scene);
        let running = Arc::clone(&self.running);
        let algorithm = self.algorithm.clone_box();

        // Start the sorting thread
        thread::spawn(move || {
            algorithm.sort(scene, running);
        });
    }

    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }

    pub fn get_data(&self) -> Vec<i32> {
        self.scene.lock().unwrap().data.lock().unwrap().clone()
    }

    pub fn render(&self, engine: &mut ConsoleEngine) {
        self.scene.lock().unwrap().draw(engine);
    }
}

impl Clone for SortingRunner {
    fn clone(&self) -> Self {
        Self {
            algorithm: self.algorithm.clone_box(),
            scene: Arc::clone(&self.scene),
            running: Arc::clone(&self.running),
        }
    }
}

#[derive(Clone)]
pub struct BubbleSort;

impl SortingAlgorithm for BubbleSort {
    fn sort(&self, scene: Arc<Mutex<SortScene>>, running: Arc<Mutex<bool>>) {
        let data = scene.lock().unwrap().data.clone();
        let mut data = data.lock().unwrap();
        for i in 0..data.len() {
            for j in 0..data.len() - i - 1 {
                if !*running.lock().unwrap() {
                    return; // Stop sorting if the `running` flag is set to false
                }

                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }

                // Update the scene data for visualization
                scene.lock().unwrap().data = Arc::new(Mutex::new(data.clone()));
                // Add a delay for the animation
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}
