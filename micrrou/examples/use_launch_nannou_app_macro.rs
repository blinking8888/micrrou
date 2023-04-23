use micrrou::launch_nannou_app;
use micrrou::{nannou_app::Model, prelude::Drawable};

struct ModelData {
    drawings: Vec<Box<dyn Drawable>>,
}

impl Default for ModelData {
    fn default() -> Self {
        Self {
            drawings: Vec::new(),
        }
    }
}

impl Model for ModelData {
    fn create() -> Self {
        Self::default()
    }

    fn get_drawings(&self) -> &[Box<dyn Drawable>] {
        &self.drawings
    }

    fn update(&mut self) {}
}

pub fn main() {
    launch_nannou_app!(for ModelData, canvas(width=900, height=900));
}
