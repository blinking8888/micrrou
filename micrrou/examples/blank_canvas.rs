use std::slice::Iter;

use micrrou::prelude::*;

struct MyModel {
    drawings: Vec<Box<dyn Drawable>>,
    frame_count: usize,
}

impl Model for MyModel {
    fn create() -> Self {
        Self {
            drawings: Vec::new(),
            frame_count: 0,
        }
    }

    fn get_drawings<'a>(&'a self) -> Iter<'a, Box<dyn Drawable>> {
        self.drawings.iter()
    }

    fn update(&mut self) {
        self.frame_count += 1;
        println!("frame count: {}", self.frame_count);
    }
}

pub fn main() {
    nannou_app::launch::<MyModel, 900, 900>();
}
