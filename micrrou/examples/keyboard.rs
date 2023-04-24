/// This is a simple mouse handling controls that logs the mouse events and stats on the console
use micrrou::prelude::*;

#[derive(Default)]
struct KeyEventHandlerModel {
    num_presses: u32,
    num_releases: u32,
    drawings: Vec<Box<dyn Drawable>>,
}

impl Model for KeyEventHandlerModel {
    fn create() -> Self {
        KeyEventHandlerModel::default()
    }

    fn get_drawings(&self) -> &[Box<dyn Drawable>] {
        self.drawings.as_slice()
    }

    fn update(&mut self) {}

    fn key_event(&mut self, _app: &nannou::App, event: keyboard::Event) {
        let key = match event {
            KeyEvent::Pressed(key) => {
                self.num_presses += 1;
                Self::print_key_pressed(&key);
                key
            }
            KeyEvent::Released(key) => {
                self.num_releases += 1;
                key
            }
        };

        if key == Key::Return {
            println!("\nStats");
            println!("\tpresses: {}", self.num_presses);
            println!("\treleases: {}", self.num_releases);
        }
    }
}

impl KeyEventHandlerModel {
    fn print_key_pressed(key: &Key) {
        use std::io::Write;
        match key {
            Key::Back => print!("\x08 \x08"),
            Key::Space => print!(" "),
            _ => print!("{:?}", &key),
        }
        std::io::stdout().flush().unwrap();
    }
}

pub fn main() {
    nannou_app::launch::<KeyEventHandlerModel, 200, 200>();
}
