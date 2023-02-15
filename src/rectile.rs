use std::convert::From;

use nannou::prelude::*;

use crate::concentrigon::*;
use crate::drawable::Drawable;
use crate::palette::*;
use crate::shape::Shape;

impl From<Rectile> for ConcentriGon {
    fn from(rectile: Rectile) -> Self {
        ConcentriGon::new(
            rectile.origin,
            rectile.wh,
            Shape::Rectangle,
            Palette::YOMONO,
            true,
        )
    }
}

impl From<&Rectile> for ConcentriGon {
    fn from(rectile: &Rectile) -> Self {
        ConcentriGon::new(
            rectile.origin,
            rectile.wh,
            Shape::Rectangle,
            Palette::YOMONO,
            true,
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectile {
    origin: Point2,
    wh: (f32, f32),
}

impl Drawable for Rectile {
    fn draw(&self, draw: &Draw) {
        let (x, y) = (self.origin.x, self.origin.y);
        let (width, height) = (self.wh.0, self.wh.1);
        let pt1 = self.origin;
        let pt2 = Point2::new(x + width, y);
        let pt3 = Point2::new(x + width, y - height);
        let pt4 = Point2::new(x, y - height);

        let pts = vec![pt1, pt2, pt3, pt4, pt1];

        draw.polyline().stroke_weight(2.0).points(pts).color(WHITE);
    }
}

impl Rectile {
    pub fn new(origin: Point2, wh: (f32, f32)) -> Self {
        Self { origin, wh }
    }

    pub fn draw_all(me: &Self, draw: &Draw) {
        let r = ConcentriGon::from(me);
        ConcentriGon::draw_all(r, draw);
        if let Some(divs) = me.divs() {
            for fractal in divs.iter() {
                Self::draw_all(fractal, draw);
            }
        }
    }

    fn divs(&self) -> Option<Vec<Self>> {
        let (nw, nh) = (self.wh.0 / 2.0, self.wh.1 / 2.0);

        let chance = random_range(0, 100);
        if nw < 20.0 || nh < 20.0 || chance < 25 {
            None
        } else {
            let (x, y) = (self.origin.x, self.origin.y);

            let mut v = Vec::new();
            for i in 0..2 {
                let x0 = (i as f32) * nw + x;
                for j in 0..2 {
                    let y0 = y - (j as f32) * nh;
                    let div = Self::new(Point2::new(x0, y0), (nw, nh));
                    v.push(div);
                }
            }
            Some(v)
        }
    }
}
