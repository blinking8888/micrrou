use std::slice::Iter;

/// This is a simple mouse handling controls that logs the mouse events and stats on the console
use micrrou::prelude::*;

#[derive(Default)]
struct MouseHandlerModel {
    wheel_scroll_delta: f32,
    num_presses: u32,
    num_releases: u32,
    drawings: Vec<Box<dyn Drawable>>,
}

impl Model for MouseHandlerModel {
    fn create() -> Self {
        MouseHandlerModel::default()
    }

    fn get_drawings<'a>(&'a self) -> Iter<'a, Box<dyn Drawable>> {
        self.drawings.iter()
    }

    fn update(&mut self) {
        println!("Stats");
        println!("\tpresses: {}", self.num_presses);
        println!("\treleases: {}", self.num_releases);
        println!("\tscrolls: {}", self.wheel_scroll_delta);
    }

    fn mouse_event(&mut self, _app: &nannou::App, event: mouse::Event) {
        match event {
            MouseEvent::Pressed(key) => {
                self.num_presses += 1;
                println!("{key:#?} key was pressed");
            }
            MouseEvent::Released(key) => {
                self.num_releases += 1;
                println!("{key:#?} key was released");
            }
            MouseEvent::WheelScroll(delta) => {
                self.wheel_scroll_delta += delta;
            }
        }
    }
}

pub fn main() {
    nannou_app::launch::<MouseHandlerModel, 200, 200>();
}
