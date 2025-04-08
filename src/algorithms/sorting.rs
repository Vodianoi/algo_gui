use std::{
    any::Any,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use console_engine::Color;

use crate::data::data_structures::{Runnable, Scene, SortHighlight, SortingContext};

#[derive(Clone)]
pub struct BubbleSort;

impl Runnable<Vec<i32>, SortingContext> for BubbleSort {
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
        for i in 0..data.len() {
            for j in 0..data.len() - i - 1 {
                if !running.load(Ordering::SeqCst) {
                    return; // Stop if the running flag is false
                }

                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }

                // Update the scene for visualization
                let hightlight = SortHighlight {
                    indices: vec![j, j + 1],
                    color: Color::Red,
                };
                let context = SortingContext {
                    highlights: vec![hightlight],
                };
                scene.lock().unwrap().update(data, &context);
            }
        }

        // Final update for the sorted array
        final_check(data, &scene);
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
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
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
            let hightlight = SortHighlight {
                indices: vec![i, min_index],
                color: Color::Red,
            };
            let context = SortingContext {
                highlights: vec![hightlight],
            };
            scene.lock().unwrap().update(data, &context);
        }

        // Final update for the sorted array
        final_check(data, &scene);
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
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
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
                let hightlight = SortHighlight {
                    indices: vec![j, j + 1],
                    color: Color::Red,
                };
                let context = SortingContext {
                    highlights: vec![hightlight],
                };
                scene.lock().unwrap().update(data, &context);
            }
            data[j] = key;
        }

        // Final update for the sorted array
        final_check(data, &scene);
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
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
        let n = data.len();
        let mut temp = data.clone();
        self.merge_sort(data, &mut temp, 0, n - 1, &scene, &running);

        // Final update for the sorted array
        final_check(data, &scene);
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

        // Highlight the subarrays being merged
        let highlights: Vec<SortHighlight> = (left..=mid)
            .map(|index| SortHighlight {
                indices: vec![index],
                color: Color::Blue,
            })
            .chain((mid + 1..=right).map(|index| SortHighlight {
                indices: vec![index],
                color: Color::Yellow,
            }))
            .collect();
        let context = SortingContext { highlights };
        scene.lock().unwrap().update(data, &context);

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
                let highlight = SortHighlight {
                    indices: if k > 0 { vec![k - 1] } else { vec![] },
                    color: Color::Red,
                };
                let context = SortingContext {
                    highlights: vec![highlight],
                };
                scene.lock().unwrap().update(data, &context);
            }
            k += 1;
        }

        while i <= mid {
            temp[k] = data[i];
            i += 1;
            k += 1;

            let highlight = SortHighlight {
                indices: vec![k - 1],
                color: Color::Red,
            };
            let context = SortingContext {
                highlights: vec![highlight],
            };
            scene.lock().unwrap().update(data, &context);
        }

        while j <= right {
            temp[k] = data[j];
            j += 1;
            k += 1;
        }

        for i in left..=right {
            data[i] = temp[i];
        }

        let hightlight = SortHighlight {
            indices: vec![k - 1],
            color: Color::Red,
        };
        let highlights: Vec<SortHighlight> = (left..=right)
            .map(|index| SortHighlight {
                indices: vec![index],
                color: Color::Green,
            })
            .collect();
        let concatenated_highlights = highlights
            .into_iter()
            .chain(vec![hightlight])
            .collect::<Vec<_>>();
        let context = SortingContext {
            highlights: concatenated_highlights,
        };
        scene.lock().unwrap().update(data, &context);
        // }

        // // Final update for the merged subarray
        // let hightlight = SortHighlight {
        //     indices: (left..=right).collect(),
        //     color: Color::Green,
        // };
        // let context = SortingContext {
        //     highlights: vec![hightlight],
        // };
        // scene.lock().unwrap().update(data, &context);
    }
}

#[derive(Clone)]
pub struct QuickSort;

impl Runnable<Vec<i32>, SortingContext> for QuickSort {
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
        let high = data.len() as i32 - 1;
        self.quick_sort(data, 0, high, &scene, &running);
        // Final update for the sorted array
        final_check(data, &scene);
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
            let highlight = SortHighlight {
                indices: vec![i as usize, j as usize],
                color: Color::Red,
            };
            let context = SortingContext {
                highlights: vec![highlight],
            };
            scene.lock().unwrap().update(data, &context);
        }

        data.swap((i + 1) as usize, high as usize);

        // Update the scene for visualization
        let highlight = SortHighlight {
            indices: vec![(i + 1) as usize, high as usize],
            color: Color::Red,
        };
        let context = SortingContext {
            highlights: vec![highlight],
        };
        scene.lock().unwrap().update(data, &context);

        i + 1
    }
}

#[derive(Clone)]
pub struct HeapSort;

impl Runnable<Vec<i32>, SortingContext> for HeapSort {
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
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
            let highlight = SortHighlight {
                indices: vec![0, i],
                color: Color::Red,
            };
            let context = SortingContext {
                highlights: vec![highlight],
            };
            scene.lock().unwrap().update(data, &context);

            self.heapify(data, i, 0, &scene, &running);
        }

        // Final update for the sorted array
        final_check(data, &scene);
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
            let highlight = SortHighlight {
                indices: vec![i, largest],
                color: Color::Red,
            };
            let context = SortingContext {
                highlights: vec![highlight],
            };
            scene.lock().unwrap().update(data, &context);

            self.heapify(data, n, largest, scene, running);
        }
    }
}

#[derive(Clone)]
pub struct ShellSort;

impl Runnable<Vec<i32>, SortingContext> for ShellSort {
    fn run(
        &self,
        data: &mut Vec<i32>,
        scene: Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>,
        running: Arc<AtomicBool>,
    ) {
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
                    let highlight = SortHighlight {
                        indices: vec![j, j + gap],
                        color: Color::Red,
                    };
                    let context = SortingContext {
                        highlights: vec![highlight],
                    };
                    scene.lock().unwrap().update(data, &context);
                }

                data[j] = temp;
            }

            gap /= 2;
        }

        // Final update for the sorted array
        final_check(data, &scene);
    }

    fn clone_box(&self) -> Box<dyn Runnable<Vec<i32>, SortingContext>> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Util function to highlight final sorted array
pub fn final_check(data: &Vec<i32>, scene: &Arc<Mutex<dyn Scene<Vec<i32>, SortingContext>>>) {
    let mut highlights = Vec::new();
    let mut is_sorted = true;

    for i in 0..data.len() {
        if i > 0 && data[i] < data[i - 1] {
            highlights.push(SortHighlight {
                indices: vec![i],
                color: Color::Red,
            });
            is_sorted = false;
        } else {
            highlights.push(SortHighlight {
                indices: vec![i],
                color: Color::Green,
            });
        }

        // Update the scene at each step
        let context = SortingContext {
            highlights: highlights.clone(),
        };
        scene.lock().unwrap().update(data, &context);
    }

    if !is_sorted {
        return; // Data is not sorted
    }
}
