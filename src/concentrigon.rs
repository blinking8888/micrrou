use nannou::prelude::*;

use crate::attrs::*;
use crate::drawable::Drawable;
use crate::palette::*;
use crate::shape::Shape;

#[derive(Copy, Clone, Debug)]
pub enum InnerDecrease {
    Fixed(f32),
    Random(f32, f32),
}

impl InnerDecrease {
    fn get_amount(&self) -> f32 {
        match self {
            InnerDecrease::Fixed(amt) => *amt,
            InnerDecrease::Random(min, max) => random_range(*min, *max),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ConcentriGon {
    pub attrs: Attributes,
    inner_decrease: InnerDecrease,
    shape: Shape,
    pallette: Palette,
    fill: bool,
}

impl ConcentriGon {
    pub fn new(
        origin: Point2,
        wh: (f32, f32),
        shape: Shape,
        pallette: Palette,
        fill: bool,
    ) -> Self {
        let mut attrs = Attributes::default();
        let stroke_weight = if fill { 0.0 } else { 2.0 };
        attrs
            .set_location(origin)
            .set_height(wh.1)
            .set_width(wh.0)
            .set_rgba(255, 255, 255, 255)
            .set_stroke_weight(stroke_weight);

        ConcentriGon {
            attrs,
            inner_decrease: InnerDecrease::Random(wh.0 * 0.05, wh.1 * 0.20),
            shape,
            pallette,
            fill,
        }
    }

    fn get_inner(&self) -> Option<Self> {
        let amtx = self.inner_decrease.get_amount();

        let w_dec = amtx * 2.0;
        let (w, h) = (self.width() - w_dec, self.height() - w_dec);

        if w <= 0.0 || h <= 0.0 {
            None
        } else {
            //            let chance = random_range(0, 100);
            let addx = amtx;
            let addy = amtx;
            //          if chance < 5 {
            //let diffx = w_dec - amtx;
            //addx = random_range(0.0, diffx);
            //let diffy = w_dec - amtx;
            //addy = random_range(0.0, diffy);
            //1        }
            let pt_dec = Point2::new(addx, -addy);
            let new_location = self.location() + pt_dec;
            let mut attrs = self.attrs.clone();
            attrs.set_location(new_location).set_height(h).set_width(w);

            Some(Self {
                attrs,
                inner_decrease: InnerDecrease::Random(w * 0.05, w * 0.66),
                shape: self.shape,
                pallette: self.pallette,
                fill: self.fill,
            })
        }
    }

    pub fn draw_all(me: Self, draw: &Draw) {
        me.draw(draw);
        if let Some(inner) = me.get_inner() {
            Self::draw_all(inner, draw);
        }
    }
}

impl SettableAttributes for ConcentriGon {
    fn get_attr_mut(&mut self) -> &mut Attributes {
        &mut self.attrs
    }

    fn get_attr(&self) -> &Attributes {
        &self.attrs
    }
}

impl Drawable for ConcentriGon {
    fn draw(&self, draw: &Draw) {
        let (center_x, center_y) = (
            self.location().x + self.width() / 2.0,
            self.location().y - self.height() / 2.0,
        );

        let angle_step = self.shape.angle_step();
        let mut angle = 0.0;
        let radius = if self.width() < self.height() {
            self.width() / 2.0
        } else {
            self.height() / 2.0
        };

        let color: Rgb8 = self.pallette.pick_random();
        if self.shape == Shape::Circle {
            draw.ellipse()
                .x_y(center_x, center_y)
                .width(self.width())
                .height(self.height())
                .stroke_weight(self.stroke_weight())
                .color(color);
        } else {
            let mut pts = Vec::new();
            while angle <= TAU {
                let x = radius * angle.cos() + center_x;
                let y = radius * angle.sin() + center_y;
                pts.push(Point2::new(x, y));
                angle += angle_step;
            }

            let draw = draw.rotate(self.rotation());

            if self.fill {
                draw.polygon()
                    .stroke_weight(self.stroke_weight())
                    .points(pts)
                    .color(color);
            } else {
                draw.polyline()
                    .stroke_weight(self.stroke_weight())
                    .points(pts)
                    .color(color);
            };
        }
    }
}
