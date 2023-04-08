use nannou::noise::NoiseFn;
use nannou::prelude::*;

use crate::drawable::Drawable;

#[readonly::make]
pub struct Cell {
    pub xy: Vec2,
    pub wh: Vec2,
    pub nv: f64,
}

#[readonly::make]
pub struct NoiseGrid<N: NoiseFn<[f64; 3]>> {
    pub noise: N,
    pub cols: usize,
    pub rows: usize,
    pub wh: Vec2,
    pub k: f64,
    pub z: f64,
    max_xy: Vec2,
    cell_wh: Vec2,
}

impl<N> NoiseGrid<N>
where
    N: NoiseFn<[f64; 3]>,
{
    pub fn new(noise: N, cols: usize, rows: usize, canvas_wh: Vec2, k: f64) -> Self {
        let max_xy = vec2(canvas_wh.x * 0.5, canvas_wh.y * 0.5);
        let cell_wh = vec2(canvas_wh.x / cols as f32, canvas_wh.y / rows as f32);

        Self {
            noise,
            cols,
            rows,
            wh: canvas_wh,
            k,
            z: 0.0,
            max_xy,
            cell_wh,
        }
    }

    pub fn cell_from_position(&self, pos: Vec2) -> Cell {
        let col_id = ((pos.x + self.max_xy.x) / self.cell_wh.x).floor() as usize;
        let row_id = ((self.max_xy.y - pos.y) / self.cell_wh.y).floor() as usize;

        let wh = self.cell_wh;
        let nv = self.get_noise_value(row_id, col_id);
        let center_offset = self.cell_wh / 2.0;
        let x = -self.max_xy.x + col_id as f32 * wh.x + center_offset.x;
        let y = self.max_xy.y - (row_id as f32 * wh.y + center_offset.y);

        Cell {
            xy: vec2(x, y),
            wh,
            nv,
        }
    }

    fn get_noise_value(&self, row_id: usize, col_id: usize) -> f64 {
        let k = self.k;
        let xn = k * col_id as f64;
        let yn = k * row_id as f64;
        self.noise.get([xn, yn, self.z])
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}

impl<'a, N> NoiseGrid<N>
where
    N: NoiseFn<[f64; 3]>,
{
    pub fn iter(&'a self) -> NoiseGridIterator<'a, N> {
        NoiseGridIterator {
            index: 0,
            noise_grid: self,
        }
    }
}

impl<N> Drawable for NoiseGrid<N>
where
    N: NoiseFn<[f64; 3]>,
{
    fn draw(&self, draw: &Draw) {
        let noise_grid_info = NoiseGridInfo::from(self);
        for cell in self.iter() {
            let angle = noise_grid_info.map_value(cell.nv, 0.0, TAU);
            let len = cell.wh.x * 0.5;
            let start = vec2(-len, 0.0);
            let end = vec2(len, 0.0);
            let gray = noise_grid_info.map_value(cell.nv, 0.0, 1.0);
            let color = rgb(gray, gray, gray);
            draw.xy(cell.xy)
                .rotate(angle)
                .arrow()
                .start(start)
                .end(end)
                .color(color)
                .stroke_weight(1.0);
        }
    }
}

pub struct NoiseGridIterator<'a, N: NoiseFn<[f64; 3]>> {
    index: usize,
    noise_grid: &'a NoiseGrid<N>,
}

impl<'a, N> Iterator for NoiseGridIterator<'a, N>
where
    N: NoiseFn<[f64; 3]>,
{
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let cols = self.noise_grid.cols;
        let rows = self.noise_grid.rows;
        if self.index < (cols * rows) {
            let col_id = self.index % cols;
            let row_id = self.index / cols;

            let wh = self.noise_grid.cell_wh;
            let center_offset = self.noise_grid.cell_wh / 2.0;
            let x = -self.noise_grid.max_xy.x + col_id as f32 * wh.x + center_offset.x;
            let y = self.noise_grid.max_xy.y - (row_id as f32 * wh.y + center_offset.y);
            let nv = self.noise_grid.get_noise_value(row_id, col_id);

            self.index += 1;

            Some(Cell {
                xy: vec2(x, y),
                wh,
                nv,
            })
        } else {
            None
        }
    }
}

#[readonly::make]
pub struct NoiseGridInfo {
    pub min: f64,
    pub max: f64,
}

impl NoiseGridInfo {
    fn get_info<N: NoiseFn<[f64; 3]>>(noise_grid: &NoiseGrid<N>) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        (0..noise_grid.rows).for_each(|row| {
            (0..noise_grid.cols).for_each(|col| {
                let nv = noise_grid.get_noise_value(row, col);
                if nv < min {
                    min = nv;
                }

                if nv > max {
                    max = nv;
                }
            });
        });
        (min, max)
    }

    pub fn map_value<T: NumCast>(&self, value: f64, out_min: T, out_max: T) -> T {
        map_range(value, self.min, self.max, out_min, out_max)
    }
}

impl<N> From<&NoiseGrid<N>> for NoiseGridInfo
where
    N: NoiseFn<[f64; 3]>,
{
    fn from(noise_grid: &NoiseGrid<N>) -> Self {
        let (min, max) = Self::get_info(noise_grid);
        Self { min, max }
    }
}
