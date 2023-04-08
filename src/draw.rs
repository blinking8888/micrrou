use nannou::Draw;
use num_traits::Unsigned;

/// A Drawable object can be drawn to the current frame by implementing the
/// draw() method.
pub trait Drawable {
    /// Specific implementation to draw the object
    fn draw(&self, draw: &Draw);
}

/// A width dimention in T units
pub struct Width<T: Unsigned>(pub T);

/// A width dimention in T units
pub struct Height<T: Unsigned>(pub T);

/// Descibes the attributes of the canvas where the drawings are put
pub struct Canvas {
    w: Width<u32>,
    h: Height<u32>,
}

impl Canvas {
    /// Creates a rectangular canvas
    pub fn new(w: Width<u32>, h: Height<u32>) -> Self {
        Self { w, h }
    }

    /// Creates a square canvas (w==h)
    pub fn new_square(edge: Width<u32>) -> Self {
        let Width(edge) = edge;

        Self {
            w: Width(edge),
            h: Height(edge),
        }
    }

    /// Width of the canvas
    pub fn width(&self) -> u32 {
        let Width(w) = self.w;
        w
    }

    /// Height of the canvas
    pub fn height(&self) -> u32 {
        let Height(h) = self.h;
        h
    }
}
