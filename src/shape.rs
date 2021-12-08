use std::f32::consts::TAU;

use nannou::prelude::*;

use crate::attrs::{Attributes, SettableAttributes};
use crate::drawable::Drawable;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Line,
    Triangle,
    Square,
    Rectangle,
    Pentagon,
    Hexagon,
    Heptagon,
    Ocatagon,
    Nonagon,
    Decagon,
    Circle,
}

impl Shape {
    pub fn sides(&self) -> usize {
        match self {
            Self::Line => 2,
            Self::Triangle => 3,
            Self::Square => 4,
            Self::Rectangle => 4,
            Self::Pentagon => 5,
            Self::Hexagon => 6,
            Self::Heptagon => 7,
            Self::Ocatagon => 8,
            Self::Nonagon => 9,
            Self::Decagon => 10,
            Self::Circle => 0,
        }
    }
    pub fn angle_step(&self) -> f32 {
        if *self != Self::Circle {
            TAU / (self.sides() as f32)
        } else {
            TAU / 100.0
        }
    }
}

#[readonly::make]
pub struct DrawableShape {
    pub attrs: Attributes,
    pub shape: Shape,
}

impl DrawableShape {
    pub fn new(shape: Shape, attrs: Attributes) -> Self {
        Self { attrs, shape }
    }

    pub fn set_attrs(&mut self, attrs: Attributes) -> &mut Self {
        self.attrs = attrs;
        self
    }
}

impl SettableAttributes for DrawableShape {
    fn get_attr_mut(&mut self) -> &mut Attributes {
        &mut self.attrs
    }

    fn get_attr(&self) -> &Attributes {
        &self.attrs
    }
}

impl Drawable for DrawableShape {
    fn draw(&self, draw: &Draw) {
        let (center_x, center_y) = (self.location().x, self.location().y);

        let angle_step = self.shape.angle_step();
        let radius = if self.width() < self.height() {
            self.width() / 2.0
        } else {
            self.height() / 2.0
        };

        let color = self.rgba8();
        if self.shape == Shape::Circle {
            draw.ellipse()
                .x_y(center_x, center_y)
                .width(self.width())
                .height(self.height())
                .rgb8(color.0, color.1, color.2);
        } else if self.shape == Shape::Rectangle {
            draw.rect()
                .x_y(center_x, center_y)
                .width(self.width())
                .height(self.height())
                .rgb8(color.0, color.1, color.2)
                .rotate(self.rotation());
        } else {
            let mut pts = Vec::new();
            let mut angle = 0.0 + self.rotation();
            while angle <= (TAU + self.rotation()) {
                let x = radius * angle.cos() + center_x;
                let y = radius * angle.sin() + center_y;
                pts.push(Point2::new(x, y));
                angle += angle_step;
            }

            draw.polyline()
                .stroke_weight(self.stroke_weight())
                .points(pts)
                .rgb8(color.0, color.1, color.2);
        }
    }
}
