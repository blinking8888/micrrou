use nannou::noise::NoiseFn;
use nannou::prelude::*;

pub struct NoiseLoop<N>
where
    N: NoiseFn<[f64; 3]>,
{
    noise: N,
    radius: f32,
    offset: Vec2,
    z: f64,
}

impl<N> NoiseLoop<N>
where
    N: NoiseFn<[f64; 3]>,
{
    pub fn new(noise: N, radius: f32) -> Self {
        let offset = vec2(random_range(-1000.0, 1000.0), random_range(-1000.0, 1000.0));
        Self {
            noise,
            radius,
            offset,
            z: 0.0,
        }
    }

    pub fn get(&self, angle: f32) -> f64 {
        let k = 0.005;
        let x = (self.offset.x + self.radius * angle.cos()) as f64;
        let y = (self.offset.y + self.radius * angle.sin()) as f64;

        self.noise.get([x * k, y * k, self.z])
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}

impl<'a, N> NoiseLoop<N>
where
    N: NoiseFn<[f64; 3]>,
{
    pub fn iter(&'a self, count: usize) -> NoiseLoopIter<'a, N> {
        NoiseLoopIter {
            noise_loop: self,
            index: 0,
            count,
        }
    }
}

pub struct NoiseLoopIter<'a, N: NoiseFn<[f64; 3]>> {
    noise_loop: &'a NoiseLoop<N>,
    index: usize,
    count: usize,
}

impl<'a, N> Iterator for NoiseLoopIter<'a, N>
where
    N: NoiseFn<[f64; 3]>,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let angle = (self.index as f32 / self.count as f32) * TAU;
            self.index += 1;
            Some(self.noise_loop.get(angle))
        } else {
            None
        }
    }
}

#[readonly::make]
pub struct NoiseLoopMap {
    pub min: f64,
    pub max: f64,
}

impl NoiseLoopMap {
    pub fn map_value<T: NumCast>(&self, value: f64, out_min: T, out_max: T) -> T {
        map_range(value, self.min, self.max, out_min, out_max)
    }

    fn get_info<N: NoiseFn<[f64; 3]>>(noise_loop_iter: NoiseLoopIter<N>) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        for nv in noise_loop_iter {
            if nv < min {
                min = nv;
            }

            if nv > max {
                max = nv;
            }
        }
        (min, max)
    }
}

impl<'a, N> From<NoiseLoopIter<'a, N>> for NoiseLoopMap
where
    N: NoiseFn<[f64; 3]>,
{
    fn from(noise_loop_iter: NoiseLoopIter<'a, N>) -> Self {
        let (min, max) = Self::get_info(noise_loop_iter);
        Self { min, max }
    }
}
