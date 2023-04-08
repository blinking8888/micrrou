use nannou::noise::NoiseFn;
use nannou::prelude::*;

use crate::drawable::Drawable;
use crate::palette::{Palette, PickRandom};

pub struct Silk {
    seed: Point2,
    points: Vec<Point2>,
    radius: f32,
    len: usize,
}

impl Silk {
    pub fn new(x_bounds: (f32, f32), y_bounds: (f32, f32)) -> Self {
        let x = random_range(x_bounds.0, x_bounds.1);
        let y = random_range(y_bounds.0, y_bounds.1);

        Self {
            seed: Point2::new(x, y),
            points: Vec::new(),
            radius: random_range(1.0, 10.0),
            len: 1,
        }
    }

    pub fn populate(&mut self, t: f64, noise: &dyn NoiseFn<[f64; 3]>) {
        let k = 0.005;
        let mut xn = self.seed.x as f64;
        let mut yn = self.seed.y as f64;
        self.points.clear();
        self.points.push(Point2::new(xn as f32, yn as f32));
        for _i in 0..self.len {
            let angle = (noise.get([xn * k, yn * k, t * k]) as f32) * TAU;
            let x = angle.cos() * self.radius + (xn as f32);
            let y = angle.sin() * self.radius + (yn as f32);

            if x > -450.0 && x < 450.0 && y > -450.0 && y < 450.0 {
                self.points.push(Point2::new(x, y));
                xn = x as f64;
                yn = y as f64;
            } else {
                break;
            }
        }
        self.len += 1;
        self.seed.x += 0.5;
        self.seed.y += 0.5;
    }
}

impl Drawable for Silk {
    fn draw(&self, draw: &Draw) {
        let palette = Palette::StarkContrast;
        let mut color: Rgba<f32> = palette.pick_random();
        color.alpha = random_range(0.0, 1.0);
        draw.polyline()
            .stroke_weight(1.0)
            .points(self.points.iter().cloned())
            .color(color);
    }
}
