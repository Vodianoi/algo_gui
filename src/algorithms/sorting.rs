use std::{
    any::Any,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use console_engine::Color;

use crate::data::data_structures::{Runnable, Scene, SortingContext};

#[derive(Clone)]
pub struct BubbleSort;

impl Runnable<Vec<i32>, SortingContext> for BubbleSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        for i in 0..data.len() {
            for j in 0..data.len() - i - 1 {
                if !running.load(Ordering::SeqCst) {
                    return; // Stop if the running flag is false
                }

                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }

                // Update the scene for visualization
                let context = SortingContext {
                    highlights: vec![j, j + 1],
                    color: Color::Red,
                };
                scene.lock().unwrap().update(data, &context);
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct SelectionSort;

impl Runnable<Vec<i32>, SortingContext> for SelectionSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        for i in 0..data.len() {
            let mut min_index = i;
            for j in i + 1..data.len() {
                if !running.load(Ordering::SeqCst) {
                    return; // Stop sorting if the `running` flag is set to false
                }

                if data[j] < data[min_index] {
                    min_index = j;
                }
            }

            data.swap(i, min_index);

            // Update the scene for visualization
            let context = SortingContext {
                highlights: vec![i, min_index],
                color: Color::Red,
            };
            scene.lock().unwrap().update(data, &context);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct InsertionSort;

impl Runnable<Vec<i32>, SortingContext> for InsertionSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        for i in 1..data.len() {
            let key = data[i];
            let mut j = i;
            while j > 0 && data[j - 1] > key {
                if !running.load(Ordering::SeqCst) {
                    return; // Stop sorting if the `running` flag is set to false
                }

                data[j] = data[j - 1];
                j -= 1;

                // Update the scene for visualization
                let context = SortingContext {
                    highlights: vec![j, j + 1],
                    color: Color::Red,
                };
                scene.lock().unwrap().update(data, &context);
            }
            data[j] = key;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct MergeSort;

impl Runnable<Vec<i32>, SortingContext> for MergeSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        let n = data.len();
        let mut temp = data.clone();
        self.merge_sort(data, &mut temp, 0, n - 1, &scene, &running);
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MergeSort {
    fn merge_sort(
        &self,
        data: &mut Vec<i32>,
        temp: &mut Vec<i32>,
        left: usize,
        right: usize,
        scene: &Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: &Arc<AtomicBool>,
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
        scene: &Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: &Arc<AtomicBool>,
    ) {
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        while i <= mid && j <= right {
            if !running.load(Ordering::SeqCst) {
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

        // Update the scene for visualization
        let context = SortingContext {
            highlights: (left..=right).collect(),
            color: Color::Green,
        };
        scene.lock().unwrap().update(data, &context);
    }
}

#[derive(Clone)]
pub struct QuickSort;

impl Runnable<Vec<i32>, SortingContext> for QuickSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        let high = data.len() as i32 - 1;
        self.quick_sort(data, 0, high, &scene, &running);
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl QuickSort {
    fn quick_sort(
        &self,
        data: &mut Vec<i32>,
        low: i32,
        high: i32,
        scene: &Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: &Arc<AtomicBool>,
    ) {
        if low < high {
            let pi = self.partition(data, low, high, scene, running);
            self.quick_sort(data, low, pi - 1, scene, running);
            self.quick_sort(data, pi + 1, high, scene, running);
        }
    }

    fn partition(
        &self,
        data: &mut Vec<i32>,
        low: i32,
        high: i32,
        scene: &Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: &Arc<AtomicBool>,
    ) -> i32 {
        let pivot = data[high as usize];
        let mut i = low - 1;

        for j in low..high {
            if !running.load(Ordering::SeqCst) {
                return 0;
            }

            if data[j as usize] < pivot {
                i += 1;
                data.swap(i as usize, j as usize);
            }

            // Update the scene for visualization
            let context = SortingContext {
                highlights: vec![i as usize, j as usize],
                color: Color::Red,
            };
            scene.lock().unwrap().update(data, &context);
        }

        data.swap((i + 1) as usize, high as usize);

        // Update the scene for visualization
        let context = SortingContext {
            highlights: vec![(i + 1) as usize, high as usize],
            color: Color::Green,
        };
        scene.lock().unwrap().update(data, &context);

        i + 1
    }
}

#[derive(Clone)]
pub struct HeapSort;

impl Runnable<Vec<i32>, SortingContext> for HeapSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        let n = data.len();
        for i in (0..n / 2).rev() {
            self.heapify(data, n, i, &scene, &running);
        }

        for i in (0..n).rev() {
            if !running.load(Ordering::SeqCst) {
                return;
            }

            data.swap(0, i);

            // Update the scene for visualization
            let context = SortingContext {
                highlights: vec![0, i],
                color: Color::Red,
            };
            scene.lock().unwrap().update(data, &context);

            self.heapify(data, i, 0, &scene, &running);
        }
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HeapSort {
    fn heapify(
        &self,
        data: &mut Vec<i32>,
        n: usize,
        i: usize,
        scene: &Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: &Arc<AtomicBool>,
    ) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && data[left] > data[largest] {
            largest = left;
        }

        if right < n && data[right] > data[largest] {
            largest = right;
        }

        if largest != i {
            data.swap(i, largest);

            // Update the scene for visualization
            let context = SortingContext {
                highlights: vec![i, largest],
                color: Color::Red,
            };
            scene.lock().unwrap().update(data, &context);

            self.heapify(data, n, largest, scene, running);
        }
    }
}

#[derive(Clone)]
pub struct ShellSort;

impl Runnable<Vec<i32>, SortingContext> for ShellSort {
    fn run(&self, data: &mut Vec<i32>, scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>, running: Arc<AtomicBool>) {
        let n = data.len();
        let mut gap = n / 2;

        while gap > 0 {
            for i in gap..n {
                let temp = data[i];
                let mut j = i;

                while j >= gap && data[j - gap] > temp {
                    if !running.load(Ordering::SeqCst) {
                        return;
                    }

                    data[j] = data[j - gap];
                    j -= gap;

                    // Update the scene for visualization
                    let context = SortingContext {
                        highlights: vec![j, j + gap],
                        color: Color::Red,
                    };
                    scene.lock().unwrap().update(data, &context);
                }

                data[j] = temp;
            }

            gap /= 2;
        }
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
