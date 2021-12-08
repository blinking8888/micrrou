use nannou::prelude::*;

use crate::attrs::{Attributes, SettableAttributes};
use crate::drawable::Drawable;

pub struct MovingPoint {
    attrs: Attributes,
    pub angle: f32,
    angle_step: f32,
    pub radius: f32,
    start_xy: Point2,
    started: bool,
    k: f32,
}

impl SettableAttributes for MovingPoint {
    fn get_attr_mut(&mut self) -> &mut Attributes {
        &mut self.attrs
    }

    fn get_attr(&self) -> &Attributes {
        &self.attrs
    }
}

impl Drawable for MovingPoint {
    fn draw(&self, draw: &Draw) {
        let color = self.rgba8();
        draw.ellipse()
            .stroke_weight(self.stroke_weight())
            .xy(self.location())
            .rgba8(color.0, color.1, color.2, color.3)
            .w_h(self.width(), self.height());
    }
}

impl MovingPoint {
    pub fn new(angle_step: f32, radius: f32, k: f32) -> Self {
        let start_xy = Point2::new(0.0, 0.0);
        Self {
            attrs: Attributes::default(),
            angle: 0.0,
            angle_step,
            radius,
            start_xy,
            started: false,
            k,
        }
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn x(&self) -> f32 {
        let k = self.k;
        self.angle.cos() * self.radius * (self.angle * k).cos() + self.location().x
    }

    pub fn y(&self) -> f32 {
        let k = self.k;
        self.angle.sin() * self.radius * (self.angle * k).cos()
    }

    pub fn step(&mut self) -> &mut Self {
        self.angle += self.angle_step; // 2.0.sqrt();
        self
    }

    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self
    }
}

impl Iterator for MovingPoint {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x();
        let y = self.y();

        if self.start_xy == Point2::new(0.0, 0.0) {
            self.start_xy = Point2::new(x, y);
        }

        self.step();
        if self.started && x == self.start_xy.x && y == self.start_xy.y {
            println!("No more!");
            self.started = false;
            None
        } else {
            self.started = true;
            Some(Point2::new(x, y))
        }
    }
}
