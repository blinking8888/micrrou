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

    fn get_drawings(&self) -> &[Box<dyn Drawable>] {
        &self.drawings
    }

    fn update(&mut self) {
        self.frame_count += 1;
        println!("frame count: {}", self.frame_count);
    }
}

pub fn main() {
    nannou_app::launch::<MyModel, 900, 900>();
}
