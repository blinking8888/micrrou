use nannou::prelude::*;

use crate::drawable::Drawable;

pub struct WidthHeight(pub f32, pub f32);
pub struct RGBAColor(pub u8, pub u8, pub u8, pub u8);

pub struct Object {
    pub points: Vec<Vec2>,
    pub wh: WidthHeight,
    pub color: RGBAColor,
}

impl Drawable for Object {
    fn draw(&self, draw: &Draw) {
        draw.polyline()
            //.w_h(self.wh.0, self.wh.1)
            .stroke_weight(self.wh.0)
            .points(self.points.iter().cloned())
            //.x_y(self.location.0, self.location.1)
            .rgba8(self.color.0, self.color.1, self.color.2, self.color.3);
    }
}

impl Object {
    pub fn create_random(canvas_w: f32, canvas_h: f32) -> Self {
        let points = vec![Self::random_point(canvas_w, canvas_h)];
        Self {
            points,
            wh: Self::random_wh(1.0, 3.0),
            color: Self::random_color(),
        }
    }
    fn random_color() -> RGBAColor {
        let r = random_range(245, 255);
        let g = random_range(245, 255);
        let b = random_range(245, 255);
        let a = random_range(127, u8::MAX);

        RGBAColor(r, g, b, a)
    }

    fn random_point(canvas_w: f32, canvas_h: f32) -> Vec2 {
        let half_x = canvas_w / 2.0;
        let x = random_range(-half_x, half_x);
        let half_y = canvas_h / 2.0;
        let y = random_range(-half_y, half_y);
        Vec2::new(x, y)
    }

    fn random_wh(min: f32, max: f32) -> WidthHeight {
        WidthHeight(random_range(min, max), random_range(min, max))
    }

    pub fn is_out_of_bounds(&self, canvas_w: f32, canvas_h: f32) -> bool {
        let x_lower = -canvas_w / 2.0;
        let x_upper = canvas_w / 2.0;
        let y_lower = -canvas_h / 2.0;
        let y_upper = canvas_h / 2.0;

        let x = self.points[0].x;
        let y = self.points[0].y;
        if x < x_lower || x > x_upper || y < y_lower || y > y_upper {
            true
        } else {
            false
        }
    }
}
