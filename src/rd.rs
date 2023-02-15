use nannou::math::map_range;

#[derive(Clone, Copy)]
pub struct ReactionDiffusionRates {
    pub da: f32,
    pub db: f32,
    pub feed_rate: f32,
    pub kill_rate: f32,
}

impl Default for ReactionDiffusionRates {
    fn default() -> Self {
        Self {
            da: 1.0,
            db: 0.5,
            feed_rate: 0.055,
            kill_rate: 0.062,
        }
    }
}

#[readonly::make]
#[derive(Clone, Copy)]
pub struct Cell {
    pub a: f32,
    pub b: f32,
}

impl Cell {
    fn clip_value(value: f32) -> f32 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }

    pub fn new(a: f32, b: f32) -> Self {
        Self {
            a: Self::clip_value(a),
            b: Self::clip_value(b),
        }
    }

    pub fn to_u8(&self) -> u8 {
        let frac = self.b / (self.a + self.b);
        map_range(frac.abs(), 0.0, 1.0, 0, u8::MAX)
    }

    pub fn map<F, T>(&self, map_fn: F) -> T
    where
        F: Fn(f32, f32) -> T,
    {
        map_fn(self.a, self.b)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { a: 1.0, b: 0.0 }
    }
}

impl std::ops::Add for Cell {
    type Output = Cell;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl std::ops::AddAssign for Cell {
    fn add_assign(&mut self, other: Self) {
        self.a = self.a + other.a;
        self.b = self.b + other.b;
    }
}

impl std::ops::Mul<Cell> for Cell {
    type Output = Cell;
    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            a: self.a * other.a,
            b: self.b * other.b,
        }
    }
}

impl std::ops::Mul<f32> for Cell {
    type Output = Cell;
    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            a: self.a * other,
            b: self.b * other,
        }
    }
}

trait Laplace<T, U> {
    fn laplace(&self, weights: &T) -> U;
}

pub struct Cell3x3<T = Cell> {
    cells: [(T, T, T); 3],
}

impl<T> Cell3x3<T>
where
    T: Clone,
{
    fn center(&self) -> T {
        self.cells[1].1.clone()
    }
}

impl Cell3x3<Cell> {
    fn from_mixture_at_xy(mixture: &Mixture, x: usize, y: usize) -> Option<Self> {
        if x == 0 || y == 0 || x == (mixture.width - 1) || y == (mixture.height - 1) {
            None
        } else {
            let above = (y - 1) * mixture.width + x;
            let index = y * mixture.width + x;
            let below = (y + 1) * mixture.width + x;

            let cells: [(Cell, Cell, Cell); 3] = [
                (
                    mixture.cells[above - 1],
                    mixture.cells[above],
                    mixture.cells[above + 1],
                ),
                (
                    mixture.cells[index - 1],
                    mixture.cells[index],
                    mixture.cells[index + 1],
                ),
                (
                    mixture.cells[below - 1],
                    mixture.cells[below],
                    mixture.cells[below + 1],
                ),
            ];
            Some(Self { cells })
        }
    }
}

impl Laplace<Cell3x3<Cell>, Cell> for Cell3x3<Cell> {
    fn laplace(&self, weights: &Cell3x3<Cell>) -> Cell {
        let mut sum = Cell::new(0.0, 0.0);
        for i in 0..3 {
            sum += self.cells[i].0 * weights.cells[i].0;
            sum += self.cells[i].1 * weights.cells[i].1;
            sum += self.cells[i].2 * weights.cells[i].2;
        }

        sum
    }
}

impl Laplace<Cell3x3<f32>, Cell> for Cell3x3<Cell> {
    fn laplace(&self, weights: &Cell3x3<f32>) -> Cell {
        let mut sum = Cell::new(0.0, 0.0);
        for i in 0..3 {
            sum += self.cells[i].0 * weights.cells[i].0;
            sum += self.cells[i].1 * weights.cells[i].1;
            sum += self.cells[i].2 * weights.cells[i].2;
        }

        sum
    }
}

impl<T> From<[(T, T, T); 3]> for Cell3x3<T> {
    fn from(cells: [(T, T, T); 3]) -> Self {
        Self { cells }
    }
}

#[derive(Clone)]
pub struct Mixture {
    cells: Vec<Cell>,
    rdks: Vec<ReactionDiffusionRates>,
    width: usize,
    height: usize,
}

impl Mixture {
    fn new(width: usize, height: usize) -> Self {
        let n = width * height;
        let cells = Vec::with_capacity(n);
        let rdks = Vec::with_capacity(n);

        Self {
            cells,
            rdks,
            width,
            height,
        }
    }

    fn populate<F>(&mut self, func: F)
    where
        F: Fn(usize, usize) -> Cell,
    {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                self.cells.push(func(x, y));
            });
        });
    }

    fn populate_constants<F>(&mut self, func: F)
    where
        F: Fn(usize, usize) -> ReactionDiffusionRates,
    {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                self.rdks.push(func(x, y));
            });
        });
    }

    fn for_each<F>(&mut self, mut func: F)
    where
        F: FnMut(usize, usize) -> Cell,
    {
        if self.cells.len() < self.width * self.height {
            eprintln!("Looks like the Mixture has not been initialized yet!");
        } else {
            let mut i = 0;
            (0..self.height).for_each(|y| {
                (0..self.width).for_each(|x| {
                    self.cells[i] = func(x, y);
                    i += 1;
                });
            });
        }
    }

    fn at_xy(&self, x: usize, y: usize) -> Option<Cell> {
        let index = (y * self.width) + x;
        if index < self.cells.len() {
            Some(self.cells[index])
        } else {
            None
        }
    }

    // formula from https://karlsims.com/rd.html
    fn react_and_diffuse(
        cells3x3: &Cell3x3<Cell>,
        conv3x3: &Cell3x3<f32>,
        k: &ReactionDiffusionRates,
    ) -> Cell {
        let cell = cells3x3.center();

        let laplace: Cell = cells3x3.laplace(conv3x3);
        let a_bpow2 = cell.a * cell.b * cell.b;
        let a = cell.a + (k.da * laplace.a) - a_bpow2 + (k.feed_rate * (1.0 - cell.a));
        let b = cell.b + (k.db * laplace.b) + a_bpow2 - ((k.kill_rate + k.feed_rate) * cell.b);
        Cell::new(a, b)
    }

    fn update(&mut self, values: &Self, conv3x3: &Cell3x3<f32>) {
        let mut iter = values.cells.iter();
        let mut k_iter = values.rdks.iter();

        self.for_each(|x, y| {
            if let Some(cell) = iter.next() {
                if let Some(cells3x3) = Cell3x3::<Cell>::from_mixture_at_xy(values, x, y) {
                    let rdk = k_iter.next().unwrap();
                    Self::react_and_diffuse(&cells3x3, conv3x3, rdk)
                } else {
                    *cell
                }
            } else {
                eprintln!("Something's not right!");
                Cell::new(0.0, 0.0)
            }
        });
    }
}

pub type ConvolutionMatrix = Cell3x3<f32>;

pub struct PetriDish {
    buf1: Mixture,
    buf2: Mixture,
    index: usize,
    conv3x3: ConvolutionMatrix,
}

impl PetriDish {
    pub fn new(width: usize, height: usize) -> Self {
        let buf1 = Mixture::new(width, height);
        let buf2 = buf1.clone();
        Self {
            buf1,
            buf2,
            index: 0,
            conv3x3: Cell3x3::from([(0.05, 0.2, 0.05), (0.2, -1.0, 0.2), (0.05, 0.2, 0.05)]),
        }
    }

    pub fn seed_cells<F>(&mut self, seed_fn: F) -> &mut Self
    where
        F: Fn(usize, usize) -> Cell,
    {
        self.buf1.populate(seed_fn);
        self.buf2.cells = self.buf1.cells.clone();
        self
    }

    pub fn seed_rdks<F>(&mut self, seed_fn: F) -> &mut Self
    where
        F: Fn(usize, usize) -> ReactionDiffusionRates,
    {
        self.buf1.populate_constants(seed_fn);
        self.buf2.rdks = self.buf1.rdks.clone();
        self
    }

    pub fn update(&mut self) {
        if self.index == 0 {
            self.buf1.update(&self.buf2, &self.conv3x3);
        } else {
            self.buf2.update(&self.buf1, &self.conv3x3);
        }

        self.index = (self.index + 1) % 2;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if self.index == 0 {
            self.buf1.at_xy(x, y)
        } else {
            self.buf2.at_xy(x, y)
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Cell> {
        self.get_current_buffer().cells.iter()
    }

    fn get_current_buffer(&self) -> &Mixture {
        if self.index == 0 {
            &self.buf1
        } else {
            &self.buf2
        }
    }
}
