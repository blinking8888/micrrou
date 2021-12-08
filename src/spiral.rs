use std::f32::consts::TAU;

#[derive(Copy, Clone)]
pub struct Spiral {
    pos: usize,
    count: usize,
    radius: f32,
    offset: f32,
    current_radius: f32,
    steps: usize,
}

impl Spiral {
    pub fn new(count: usize, radius: f32, steps: usize, offset: f32) -> Self {
        let current_radius = radius;
        Self {
            pos: 0,
            count,
            radius,
            offset,
            current_radius,
            steps,
        }
    }

    /*
    fn step(&mut self) -> (f32, f32) {
        let pos = self.pos as f32;
        let a = pos * self.golden_ratio;
        let angle = a * TAU;

        let x = angle.cos() * pos + self.offset;
        let y = angle.sin() * pos + self.offset;

        self.pos += 1;

        (x, y)
    }
    */

    fn step(&mut self) -> Option<(f32, f32)> {
        let steps = self.steps as f32;
        let angle_step = self.pos as f32 % steps;
        let angle = (angle_step / steps) * TAU;

        let x = angle.cos() * self.current_radius + self.offset;
        let y = angle.sin() * self.current_radius + self.offset;

        self.current_radius += self.radius;
        // self.current_radius *= self.golden_ratio;
        self.pos += 1;

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
