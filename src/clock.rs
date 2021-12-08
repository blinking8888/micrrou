use nannou::prelude::*;

use crate::attrs::{Attributes, SettableAttributes};
use crate::drawable::Drawable;

struct ClockHand {
    attrs: Attributes,
    angle: f32,
    rad_per_step: f32,
}

impl SettableAttributes for ClockHand {
    fn get_attr_mut(&mut self) -> &mut Attributes {
        &mut self.attrs
    }

    fn get_attr(&self) -> &Attributes {
        &self.attrs
    }
}

impl Drawable for ClockHand {
    fn draw(&self, draw: &Draw) {
        let color = self.rgba8();
        let end_x = self.location().x + (self.width() / 2.0) * self.angle.cos();
        let end_y = self.location().y + (self.height() / 2.0) * self.angle.sin();
        draw.line()
            .stroke_weight(self.stroke_weight())
            .points(self.location(), Point2::new(end_x, end_y))
            .rgba8(color.0, color.1, color.2, color.3);
    }
}

impl ClockHand {
    fn step(&mut self) {
        self.angle -= self.rad_per_step;
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Stopped,
    Starting,
    Running,
    Fading,
}

pub struct Clock {
    attrs: Attributes,
    big_hand: ClockHand,
    small_hand: ClockHand,
    state: State,
}

impl Drawable for Clock {
    fn draw(&self, draw: &Draw) {
        let color = self.rgba8();
        draw.ellipse()
            .xy(self.location())
            .w_h(self.width(), self.height())
            .rgba8(color.0, color.1, color.2, color.3);
        self.big_hand.draw(draw);
        self.small_hand.draw(draw);
    }
}

impl SettableAttributes for Clock {
    fn get_attr_mut(&mut self) -> &mut Attributes {
        &mut self.attrs
    }

    fn get_attr(&self) -> &Attributes {
        &self.attrs
    }
}

impl Clock {
    pub fn new(center: Point2, radius: f32, minutes_per_step: f32) -> Self {
        let mut small_hand_attrs = Attributes::default();
        let small_hand_len = radius * 2.0 / 3.0;
        small_hand_attrs
            .set_location(center)
            .set_height(small_hand_len)
            .set_width(small_hand_len)
            .set_stroke_weight(10.0)
            .set_rgba(0, 0, 0, 0);

        // 1 minute/step = TAU/60.0 radian/step
        let rad_per_step = minutes_per_step * TAU / (60.0 * 60.0);
        let small_hand = ClockHand {
            attrs: small_hand_attrs,
            angle: TAU / 4.0,
            rad_per_step,
        };

        let mut big_hand_attrs = small_hand_attrs;
        let big_hand_len = radius;
        let rad_per_step = minutes_per_step * TAU / 60.0;
        big_hand_attrs
            .set_rgba(0, 0, 0, 0)
            .set_height(big_hand_len)
            .set_width(big_hand_len);

        let big_hand = ClockHand {
            attrs: big_hand_attrs,
            angle: TAU / 4.0,
            rad_per_step,
        };

        let mut attrs = big_hand_attrs;
        attrs.set_rgba(255, 255, 255, 0);
        Self {
            attrs,
            big_hand,
            small_hand,
            state: State::Stopped,
        }
    }

    fn starting(&mut self) {
        self.big_hand.set_alpha(self.big_hand.alpha() + 1);
        self.small_hand.set_alpha(self.small_hand.alpha() + 1);
        self.set_alpha(self.alpha() + 1);

        if self.alpha() == u8::MAX {
            self.state = State::Running;
        }
    }

    fn fading(&mut self) {
        self.big_hand.set_alpha(self.big_hand.alpha() - 1);
        self.small_hand.set_alpha(self.small_hand.alpha() - 1);
        self.set_alpha(self.alpha() - 1);

        if self.alpha() == u8::MIN {
            self.state = State::Stopped;
            self.small_hand.rad_per_step = 0.0;
            self.big_hand.rad_per_step = 0.0;
        }
    }

    pub fn step(&mut self) -> &mut Self {
        match self.state {
            State::Stopped => {}
            State::Starting => {
                self.starting();
            }
            State::Fading => {
                self.fading();
            }
            State::Running => {}
        }
        self.small_hand.step();
        self.big_hand.step();
        self
    }

    pub fn fade(&mut self) -> &mut Self {
        self.state = State::Fading;
        self
    }

    pub fn run(&mut self, minutes_per_step: f32) -> &mut Self {
        // 1 minute/step = TAU/60.0 radian/step
        let rad_per_step = minutes_per_step * TAU / (60.0 * 60.0);
        self.small_hand.rad_per_step = rad_per_step;

        let rad_per_step = rad_per_step * 60.0;
        self.big_hand.rad_per_step = rad_per_step;

        self.state = State::Starting;

        self
    }

    pub fn is_stopped(&self) -> bool {
        self.state == State::Stopped
    }
}
