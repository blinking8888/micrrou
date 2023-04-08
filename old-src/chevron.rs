use nannou::prelude::*;

use crate::angle::Angle;
use crate::drawable::Drawable;
use crate::point::Point;

pub struct Chevron {
    angle: Angle,
    length: f32,
    xy: Point<f32>,
}

impl Drawable for Chevron {
    fn draw(&self, draw: &Draw) {
        draw.polyline().points(self.points());
    }
}

impl Chevron {
    pub fn new(angle: Angle, length: f32, xy: Point<f32>) -> Self {
        Self { angle, length, xy }
    }

    pub fn points(&self) -> Vec<Vec2> {
        let angle = Angle::Degrees(90.0) - (self.angle / 2.0);
        let mut ret = Vec::new();

        let x = self.xy.x + self.length * angle.cos();
        let y = self.xy.y - self.length * angle.sin();
        ret.push(Vec2::new(x, y));

        ret.push(Vec2::new(self.xy.x, self.xy.y));

        let x = self.xy.x - self.length * angle.cos();
        ret.push(Vec2::new(x, y));
        ret
    }

    pub fn angle(&self) -> Angle {
        self.angle
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn width(&self) -> f32 {
        let angle = Angle::Degrees(90.0) - (self.angle / 2.0);
        let cos = angle.cos();
        let x1 = self.xy.x + self.length * cos;
        let x2 = self.xy.x - self.length * cos;
        (x1 - x2).abs()
    }

    pub fn height(&self) -> f32 {
        let angle = Angle::Degrees(90.0) - (self.angle / 2.0);
        let y = self.xy.y - (self.length * angle.sin());
        (self.xy.y - y).abs()
    }
}
