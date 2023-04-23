use macrrou::launch_nannou_app;
use micrrou::{
    prelude::{Canvas, Drawable, Height, Width},
    setup::Model,
};

struct ModelData {
    drawings: Vec<Box<dyn Drawable>>,
    canvas: Canvas,
}

impl Default for ModelData {
    fn default() -> Self {
        Self {
            drawings: Vec::new(),
            canvas: Canvas::new(Width(900), Height(900)),
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

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}

pub fn main() {
    launch_nannou_app!(for ModelData, canvas(width=900, height=900));
}
