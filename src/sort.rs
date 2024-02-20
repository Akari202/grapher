use std::time::Duration;
use crate::axis::Axis2D;
use crate::colors::{GREEN, PINK, RED, WHITE};
use crate::{audio, config};
use crate::coordinate::PixelCoordinate2D;
use crate::renderer::{Drawable, Renderer};

const PADDING: u32 = 10;
const ELEMENT_PADDING: u32 = 3;
const ELEMENT_WIDTH: u32 = 15;
const ELEMENT_MAX: u32 = 512;
const AUDIO_SORT_LOOP: bool = true;

pub struct SortingVisualization {
    element_count: u32,
    elements: Vec<u32>,
    active_elements: Vec<usize>,
    index: usize,
    gap: usize,
    comparison_count: u32,
    swap_count: u32,
    pub(crate) sorted: bool,
    correct: bool,
    sort_type: SortType,
    pub(crate) auto_sort: bool
}

pub trait DrawableSortingVisualization {
    fn draw(&self, renderer: &mut Renderer, sort_vis: &SortingVisualization) -> Result<(), String>;
}

#[derive(Clone, Copy, PartialOrd, PartialEq)]
pub enum SortType {
    Insertion,
    Shell,
    Merge,
    Quick
}

impl SortingVisualization {
    pub fn new(sort_type: SortType) -> SortingVisualization {
        let element_count = (config::WINDOW_WIDTH - PADDING * 2 + ELEMENT_PADDING) / (ELEMENT_WIDTH + ELEMENT_PADDING);
        println!("element_count: {}", element_count);
        let elements: Vec<u32> = (0..element_count).map(|i| i * ELEMENT_MAX / element_count).collect();
        SortingVisualization {
            element_count,
            elements,
            active_elements: Vec::new(),
            index: 0,
            gap: 0,
            comparison_count: 0,
            swap_count: 0,
            sorted: true,
            correct: true,
            sort_type,
            auto_sort: false
        }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.elements.shuffle(&mut rng);
        self.sorted = false;
        self.correct = false;
        self.index = 0;
        self.comparison_count = 0;
        self.swap_count = 0;
        self.active_elements.clear();
        self.gap = 0;
    }

    pub fn step(&mut self) {
        if !self.sorted {
            match self.sort_type {
                SortType::Insertion => self.stepable_insertion_sort(),
                SortType::Shell => self.stepable_shell_sort(),
                SortType::Merge => self.stepable_merge_sort(),
                SortType::Quick => self.stepable_quick_sort(),
                _ => {}
            }
        }
    }

    fn set_sorted(&mut self) {
        self.sorted = true;
        if self.elements.is_sorted() {
            self.correct = true;
        } else {
            self.correct = false;
        }
        self.auto_sort = false;

        if AUDIO_SORT_LOOP {
            self.auto_sort = true;
            self.shuffle();
        }
    }

    fn stepable_insertion_sort(&mut self) {
        loop {
            if self.index == 0 {
                self.index += 1;
                self.comparison_count = 0;
                self.swap_count = 0;
            }
            if self.index >= self.elements.len() {
                self.active_elements.clear();
                self.index = 0;
                self.set_sorted();
                break;
            } else {
                if self.active_elements.is_empty() {
                    self.active_elements.push(self.index - 1);
                    self.active_elements.push(self.index);
                    break;
                } else if self.active_elements.len() == 2 {
                    self.comparison_count += 1;
                    if config::PLAY_AUDIO {
                        audio::play_tone(
                            self.elements[self.active_elements[0]] as f32 * 2.0,
                            Duration::from_millis(1000 / config::FRAMERATE as u64)
                        );
                    }
                    if self.elements[self.active_elements[0]] > self.elements[self.active_elements[1]] {
                        self.swap_count += 1;
                        self.elements.swap(self.active_elements[0], self.active_elements[1]);
                        if self.active_elements[0] == 0 {
                            self.active_elements.clear();
                            self.index += 1;
                        } else {
                            self.active_elements[0] -= 1;
                            self.active_elements[1] -= 1;
                            break;
                        }
                    } else {
                        self.active_elements.clear();
                        self.index += 1;
                    }
                }
            }
        }
    }

    fn insertion_sort(&mut self, elements: &mut Vec<u32>) {
        for i in 1..elements.len() {
            let temp = elements[i];
            let mut j = i;
            while j > 0 && elements[j - 1] > temp {
                elements[j] = elements[j - 1];
                j -= 1;
            }
            elements[j] = temp;
        }
    }

    fn stepable_shell_sort(&mut self) {
        loop {
            if self.gap == 0 {
                self.gap = self.elements.len() / 2;
                self.comparison_count = 0;
                self.swap_count = 0;
                self.index = 0;
            }
            if self.active_elements.is_empty() {
                if self.index + self.gap * 2 > self.elements.len() {
                    if self.gap == 1 {
                        self.set_sorted();
                        break;
                    }
                    self.gap /= 2;
                    self.index = 0;
                }
                for i in 0..(self.elements.len() - self.index) / self.gap {
                    self.active_elements.push(i * self.gap + self.index);
                }
                break;
            } else {
                for i in 0..self.active_elements.len() {
                    if self.active_elements[i] + self.gap < self.elements.len() {
                        self.comparison_count += 1;
                        if self.elements[self.active_elements[i]] > self.elements[self.active_elements[i] + self.gap] {
                            self.swap_count += 1;
                            self.elements.swap(self.active_elements[i], self.active_elements[i] + self.gap);
                        } else {
                            self.index += 1;
                        }
                    }
                }
                self.index += 1;
                self.active_elements.clear();
            }
        }
    }

    fn shell_sort(&mut self, elements: &mut Vec<u32>) {
        let mut gap = elements.len() / 2;
        while gap > 0 {
            for i in gap..elements.len() {
                let temp = elements[i];
                let mut j = i;
                while j >= gap && elements[j - gap] > temp {
                    elements[j] = elements[j - gap];
                    j -= gap;
                }
                elements[j] = temp;
            }
            gap /= 2;
        }
    }

    fn stepable_merge_sort(&mut self) {
        unimplemented!("Merge sort is not yet implemented");
    }

    fn merge_sort(&mut self, elements: &mut Vec<u32>) {
        let mid = elements.len() / 2;
        if mid > 0 {
            let mut left = elements[..mid].to_vec();
            let mut right = elements[mid..].to_vec();
            self.merge_sort(&mut left);
            self.merge_sort(&mut right);
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;
            while i < left.len() && j < right.len() {
                if left[i] < right[j] {
                    elements[k] = left[i];
                    i += 1;
                } else {
                    elements[k] = right[j];
                    j += 1;
                }
                k += 1;
            }
            while i < left.len() {
                elements[k] = left[i];
                i += 1;
                k += 1;
            }
            while j < right.len() {
                elements[k] = right[j];
                j += 1;
                k += 1;
            }
        }
    }

    fn stepable_quick_sort(&mut self) {
        unimplemented!("Quick sort is not yet implemented");
    }

    fn quick_sort(&mut self, elements: &mut Vec<u32>) {
        unimplemented!("Quick sort is not yet implemented");
    }

    fn element_number_to_pixel_x(&self, element_number: u32) -> i32 {
        (element_number * (ELEMENT_WIDTH + ELEMENT_PADDING) + PADDING) as i32
    }
}

impl Drawable for SortingVisualization {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        for (i, element) in self.elements.iter().enumerate() {
            let position = PixelCoordinate2D::new(
                self.element_number_to_pixel_x(i as u32),
                (config::WINDOW_HEIGHT - *element - PADDING) as i32
            );
            let color = if self.sorted && self.correct {
                GREEN
            } else if self.sorted && !self.correct {
                RED
            } else if self.active_elements.contains(&i) {
                RED
            } else if self.index == i && i != 0 && self.sort_type == SortType::Insertion {
                PINK
            } else {
                WHITE
            };
            renderer.draw_rect(position, ELEMENT_WIDTH, *element, color)?;
        }
        renderer.draw_text(
            &format!("{} Sort", match self.sort_type {
                SortType::Insertion => "Insertion",
                SortType::Shell => "Shell",
                SortType::Merge => "Merge",
                SortType::Quick => "Quick"
            }),
            PixelCoordinate2D::new(0, 0),
            WHITE,
            20
        )?;
        renderer.draw_text(
            &format!("Elements: {}", self.element_count),
            PixelCoordinate2D::new(0, 25),
            WHITE,
            20
        )?;
        renderer.draw_text(
            &format!("Comparisons: {}", self.comparison_count),
            PixelCoordinate2D::new(0, 50),
            WHITE,
            20
        )?;
        renderer.draw_text(
            &format!("Swaps: {}", self.swap_count),
            PixelCoordinate2D::new(0, 75),
            WHITE,
            20
        )?;
        if self.sorted {
            if self.correct {
                renderer.draw_text(
                    "Sorted!",
                    PixelCoordinate2D::new(0, 100),
                    GREEN,
                    20
                )?;
            } else {
                renderer.draw_text(
                    "Not Sorted :(",
                    PixelCoordinate2D::new(0, 100),
                    RED,
                    20
                )?;
            }
        }
        Ok(())
    }
}
