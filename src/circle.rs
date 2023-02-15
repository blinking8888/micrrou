use nannou::prelude::*;

use crate::drawable::Drawable;

pub struct Circle {
    center: Vec2,
    radius: f32,
    stroke_weight: f32,
    color: Rgb8,
    growable: bool,
}

impl Drawable for Circle {
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .stroke(self.color)
            .stroke_weight(self.stroke_weight)
            .xy(self.center)
            .w_h(self.radius * 2.0, self.radius * 2.0)
            .color(self.color);
    }
}

impl Circle {
    pub fn new(center: Vec2, radius: f32, color: Rgb8) -> Self {
        Self {
            center,
            radius,
            stroke_weight: 1.0,
            color,
            growable: true,
        }
    }

    pub fn overlaps_another(&self, other: &Self) -> bool {
        let distance = self.center.distance(other.center);
        let radii = self.radius + other.radius + self.stroke_weight + other.stroke_weight;
        distance < radii
    }

    pub fn is_on_edge(&self, xmax: f32, ymax: f32) -> bool {
        let r = self.radius + self.stroke_weight;
        let center = self.center;
        r + center.x >= xmax
            || r + center.y >= ymax
            || center.x - r <= -xmax
            || center.y - r <= -ymax
    }

    pub fn grow(&mut self, amt: f32) {
        if self.growable {
            self.radius += amt;
        }
    }

    pub fn stop_growing(&mut self) {
        self.growable = false;
    }
    pub fn is_growable(&self) -> bool {
        self.growable
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use nannou::prelude::*;

    #[test]
    fn should_not_touch() {
        let color: Rgb8 = Palette::Orange.pick_random();
        let c1 = Circle::new(Vec2::new(-1.0, 0.0), 5.0, color);
        let c2 = Circle::new(Vec2::new(6.0, 0.0), 1.0, color);

        assert_eq!(c1.overlaps_another(&c2), false);
    }

    #[test]
    fn should_not_touch_tangents() {
        let color: Rgb8 = Palette::Orange.pick_random();
        let c1 = Circle::new(Vec2::new(-1.0, 0.0), 5.0, color);
        let c2 = Circle::new(Vec2::new(6.0, 0.0), 2.0, color);

        assert_eq!(c1.overlaps_another(&c2), false);
    }

    #[test]
    fn should_overlap() {
        let color: Rgb8 = Palette::Orange.pick_random();
        let c1 = Circle::new(Vec2::new(0.0, 0.0), 5.0, color);
        let c2 = Circle::new(Vec2::new(0.0, 5.0), 1.0, color);

        assert_eq!(c1.overlaps_another(&c2), true);
    }
}
