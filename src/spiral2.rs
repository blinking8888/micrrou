use crate::angle::Angle;

#[derive(Clone)]
pub struct Offset(pub f32, pub f32);

#[derive(Clone)]
pub struct Spiral {
    pos: usize,
    count: usize,
    offset: Offset,
    radius: f32,
    current_radius: f32,
    angle: Angle,
    angle_step: Angle,
}

impl Spiral {
    pub fn new(count: usize, radius: f32, angle_step: Angle, offset: Offset) -> Self {
        let current_radius = radius;
        Self {
            pos: 0,
            count,
            offset,
            radius,
            current_radius,
            angle: Angle::Degrees(0.0),
            angle_step,
        }
    }

    fn step(&mut self) -> Option<(f32, f32)> {
        let angle = self.angle;

        let x = angle.cos() * self.current_radius + self.offset.0;
        let y = angle.sin() * self.current_radius + self.offset.1;

        self.pos += 1;
        self.angle += self.angle_step;
        self.current_radius += self.radius;

        if x.is_finite() && y.is_finite() {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn reset(&mut self, pos: usize) {
        self.pos = pos;
        self.current_radius = self.radius;
    }

    pub fn move_offset(&mut self, offset: Offset) {
        self.offset = offset;
    }

    pub fn set_angle(&mut self, angle: Angle) {
        self.angle = angle;
    }
}

impl Iterator for Spiral {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.count {
            None
        } else {
            self.step()
        }
    }
}
