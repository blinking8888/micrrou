/// Module that provides interface to mouse events
pub mod mouse {
    pub use nannou::state::mouse::Button;
    use nannou::{
        prelude::{MouseScrollDelta, TouchPhase},
        App,
    };

    use crate::nannou_app::Model;

    #[derive(Debug)]
    /// Mouse event types
    pub enum Event {
        /// Mouse press
        Pressed(Button),
        /// Mouse release
        Released(Button),
        /// Mouse scrool (+ indicates up, - indicates down)
        WheelScroll(f32),
    }

    /// The generic function to caputure mouse released events.
    pub(crate) fn released_handler<M: Model>(app: &App, model: &mut M, button: Button) {
        model.mouse_event(app, Event::Released(button));
    }

    /// The generic function to caputure mouse pressed events.
    pub(crate) fn pressed_handler<M: Model>(app: &App, model: &mut M, button: Button) {
        model.mouse_event(app, Event::Pressed(button));
    }

    /// The generic function to caputure mouse wheel events.
    pub(crate) fn wheel_handler<M: Model>(
        app: &App,
        model: &mut M,
        dt: MouseScrollDelta,
        _phase: TouchPhase,
    ) {
        match dt {
            MouseScrollDelta::LineDelta(_, y) if y > 0.0 => {
                model.mouse_event(app, Event::WheelScroll(y));
            }
            MouseScrollDelta::LineDelta(_, y) if y < 0.0 => {
                model.mouse_event(app, Event::WheelScroll(y));
            }
            _ => {}
        }
    }
}

/// Module that provides interface to keyboard events
pub mod keyboard {
    pub use nannou::prelude::Key;
    use nannou::prelude::*;

    use crate::nannou_app::Model;

    #[derive(Debug)]
    /// Keyboard event types
    pub enum Event {
        /// A key was pressed
        Pressed(Key),
        /// A key was released
        Released(Key),
    }

    pub(crate) fn pressed_handler<M: Model>(app: &App, model: &mut M, key: Key) {
        model.key_event(app, Event::Pressed(key));
    }

    pub(crate) fn released_handler<M: Model>(app: &App, model: &mut M, key: Key) {
        model.key_event(app, Event::Released(key));
    }
}
