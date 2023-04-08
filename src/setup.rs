use nannou::prelude::*;

use crate::draw::{Canvas, Drawable};

/// This is the trait that the application would implement so the drawings
/// can be put into the canvas via the Nannou framework.
pub trait Model {
    /// Constructor for the Model
    fn create() -> Self;
    /// Returns a slice of the drawable objects
    fn get_drawings(&self) -> &[Box<dyn Drawable>];
    /// Called to update the model for each frame drawing.
    fn update(&mut self);
    /// Returns the [Canvas] of the drawing
    fn canvas(&self) -> &Canvas;
}

/// Easily setup a custom model for the drawing.  The model needs to
/// implement the [Model] trait.
pub fn setup<M>()
where
    M: Model + 'static,
{
    nannou::app(create_model::<M>).update(update).run();
}

fn create_model<M>(app: &App) -> M
where
    M: Model + 'static,
{
    let model = M::create();
    let canvas = model.canvas();

    app.new_window()
        .title(app.exe_name().unwrap())
        .size(canvas.width(), canvas.height())
        .view(view::<M>)
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
