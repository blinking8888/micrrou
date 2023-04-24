use nannou::prelude::*;

use crate::controller::{keyboard, mouse};
use crate::draw::Drawable;

/// This is the trait that the application would implement so the drawings
/// can be put into the canvas via the Nannou framework.
pub trait Model {
    /// Constructor for the Model
    fn create() -> Self;

    /// Returns a slice of the drawable objects
    fn get_drawings(&self) -> &[Box<dyn Drawable>];

    /// Called to update the model for each frame drawing.
    fn update(&mut self);

    /// Implement to handle mouse events.  The default is to do nothing
    fn mouse_event(&mut self, _app: &App, _event: mouse::Event) {}

    /// Implement this to handle keyboard events.  The default is to do nothing.
    fn key_event(&mut self, _app: &App, _event: keyboard::Event) {}
}

/// Easily setup a custom model for the drawing.  The model needs to
/// implement the [Model] trait.
pub fn launch<M, const W: u32, const H: u32>()
where
    M: Model + 'static,
{
    nannou::app(create_model::<M, W, H>).update(update).run();
}

fn create_model<M, const W: u32, const H: u32>(app: &App) -> M
where
    M: Model + 'static,
{
    let model = M::create();

    app.new_window()
        .title(app.exe_name().unwrap())
        .size(W, H)
        .view(view::<M>)
        .mouse_pressed::<M>(mouse::pressed_handler)
        .mouse_released::<M>(mouse::released_handler)
        .mouse_wheel::<M>(mouse::wheel_handler)
        .key_pressed::<M>(keyboard::pressed_handler)
        .key_released::<M>(keyboard::released_handler)
        .build()
        .unwrap();

    model
}

fn view<M>(app: &App, model: &M, frame: Frame)
where
    M: Model,
{
    let draw = app.draw();
    for object in model.get_drawings() {
        object.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update<M>(_app: &App, model: &mut M, _update: Update)
where
    M: Model,
{
    model.update();
}
