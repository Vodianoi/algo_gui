use std::sync::{Arc, Mutex};

use console_engine::{Color, ConsoleEngine};

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

        // After sorting, make a last loop on all data to highlight it
        let data = self.scene.0.lock().unwrap().buffer.back().unwrap().clone();
        for i in 0..data.len() {
                // Highlight from 0 to i
                let highlight = (0..=i).collect();
                let highlight_color = Color::Green;

                self.scene.update(data.clone(), highlight, highlight_color);
            
        }
    }

    pub fn stop(&self) {
        self.running.set(false);
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
                let highlight = vec![j, j + 1];
                scene.update(data.clone(), highlight, Color::Red);
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
            let highlight = vec![i, min_index];
            scene.update(data.clone(), highlight, Color::Red);
        }
    }

    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct InsertionSort;

impl SortingAlgorithm for InsertionSort {
    fn sort(&self, scene: SharedSortScene, running: SharedBool) {
        let mut data = scene.get_data();
        for i in 1..data.len() {
            let key = data[i];
            let mut j = i;
            while j > 0 && data[j - 1] > key {
                if !running.get() {
                    return; // Stop sorting if the `running` flag is set to false
                }

                data[j] = data[j - 1];
                j -= 1;

                // Update the scene data for visualization
                let highlight = vec![j, j + 1];
                scene.update(data.clone(), highlight, Color::Red);
            }
            data[j] = key;
        }
    }

    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct MergeSort;

impl SortingAlgorithm for MergeSort {
    fn sort(&self, scene: SharedSortScene, running: SharedBool) {
        let mut data = scene.get_data();
        let n = data.len();
        let mut temp = data.clone();
        self.merge_sort(&mut data, &mut temp, 0, n - 1, &scene, &running);
    }

    fn clone_box(&self) -> Box<dyn SortingAlgorithm> {
        Box::new(self.clone())
    }


}

impl MergeSort {
    fn merge_sort(
        &self,
        data: &mut Vec<i32>,
        temp: &mut Vec<i32>,
        left: usize,
        right: usize,
        scene: &SharedSortScene,
        running: &SharedBool,
    ) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;
        self.merge_sort(data, temp, left, mid, scene, running);
        self.merge_sort(data, temp, mid + 1, right, scene, running);
        self.merge(data, temp, left, mid, right, scene, running);
    }

    fn merge(
        &self,
        data: &mut Vec<i32>,
        temp: &mut Vec<i32>,
        left: usize,
        mid: usize,
        right: usize,
        scene: &SharedSortScene,
        running: &SharedBool,
    ) {
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        while i <= mid && j <= right {
            if !running.get() {
                return;
            }

            if data[i] <= data[j] {
                temp[k] = data[i];
                i += 1;
            } else {
                temp[k] = data[j];
                j += 1;
            }

            k += 1;
        }

        while i <= mid {
            temp[k] = data[i];
            i += 1;
            k += 1;
        }

        while j <= right {
            temp[k] = data[j];
            j += 1;
            k += 1;
        }

        for i in left..=right {
            data[i] = temp[i];
        }
        
        let highlight = (left..=right).collect();
        scene.update(data.clone(), highlight, Color::Green);
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

    pub fn update(&self, data: Vec<i32>, highlight: Vec<usize>, highlight_color: Color) {
        self.0.lock().unwrap().update(data, &highlight, highlight_color);
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
