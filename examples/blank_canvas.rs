use micrrou::draw::*;
use micrrou::setup::*;

struct MyModel {
    canvas: Canvas,
    drawings: Vec<Box<dyn Drawable>>,
    frame_count: usize,
}

impl Model for MyModel {
    fn create() -> Self {
        Self {
            canvas: Canvas::new(Width(900), Height(900)),
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

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}

pub fn main() {
    setup::<MyModel>();
}
