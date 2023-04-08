use std::convert::From;

use nannou::color::*;
use nannou::image::Primitive;
use nannou::prelude::*;

pub type Triplet<T = u8> = (T, T, T);

pub const RUSTY: [Triplet<u8>; 5] = [
    (255, 120, 0),
    (255, 173, 99),
    (255, 150, 57),
    (197, 93, 0),
    (155, 73, 0),
];

pub const RYO: [Triplet; 6] = [
    (219, 216, 31),
    (209, 153, 69),
    (173, 77, 29),
    (250, 37, 0),
    (250, 138, 0),
    (245, 250, 142),
];

pub const YOMONO: [Triplet; 5] = [
    (222, 220, 90),
    (166, 163, 3),
    (148, 146, 61),
    (232, 231, 144),
    (102, 101, 42),
];

pub const PASTEL: [Triplet; 5] = [
    (61, 128, 217),
    (3, 140, 62),
    (242, 159, 5),
    (242, 68, 5),
    (242, 167, 160),
];

pub const FIRE: [Triplet; 5] = [
    (38, 10, 4),
    (140, 39, 3),
    (217, 86, 11),
    (242, 139, 48),
    (242, 167, 27),
];

pub const NORTHERN_LIGHTS: [Triplet; 5] = [
    (41, 52, 115),
    (28, 54, 89),
    (3, 140, 101),
    (3, 166, 106),
    (191, 107, 107),
];
pub const DARK_ROAD_CURVE: [Triplet; 4] =
    [(10, 7, 8), (68, 68, 68), (116, 116, 116), (177, 177, 177)];
pub const STARK_CONTRAST: [Triplet; 4] = [
    (17, 13, 23),
    (193, 199, 198),
    (108, 106, 97),
    (255, 238, 213),
];
pub const SKIN_TONES: [Triplet; 5] = [
    (242, 208, 167),
    (191, 165, 132),
    (115, 58, 38),
    (166, 94, 68),
    (64, 29, 22),
];

pub const PHM: [Triplet; 3] = [PHM_R, PHM_B, (0, 0, 0)];

pub const BLACK_OR_WHITE: [Triplet; 2] = [(0, 0, 0), (255, 255, 255)];

pub const CMYK: [Triplet; 4] = [(31, 224, 227), (227, 31, 224), (224, 227, 31), (0, 0, 0)];
pub const CM: [Triplet; 2] = [(31, 224, 227), (227, 31, 224)];

pub const PHM_R: Triplet = (210, 73, 145); //(31, 224, 227);
pub const PHM_B: Triplet = (40, 121, 150); //(103, 219, 255); // (0, 190, 255); //(227, 31, 224);
pub const PHM_P: Triplet = (216, 107, 107); //(255, 167, 167);

pub trait PickRandom<T> {
    fn pick_random(&self) -> T;
}

pub trait SetAlpha<T: Primitive> {
    fn set_alpha(&mut self, alpha: T);
}

#[derive(Copy, Clone, Debug)]
pub enum Palette {
    Rusty,
    RYO,
    YOMONO,
    White,
    Black,
    NorthernLights,
    DarkRoadCurve,
    StarkContrast,
    SkinTones,
    Pastel,
    Fire,
    Orange,
    Cmyk,
    Cm,
    BlackOrWhite,
    PrettyHateMachine,
}

pub const WHITE_P: [Triplet; 1] = [(255, 255, 255)];
pub const BLACK_P: [Triplet; 1] = [(0, 0, 0)];
pub const ORANGE_P: [Triplet; 1] = [(255, 116, 0)];

fn to_rgba_f32(r: u8, g: u8, b: u8) -> Rgba<f32> {
    const MAX: f32 = 255.0;
    Rgba::from((r as f32 / MAX, g as f32 / MAX, b as f32 / MAX, 1.0))
}

impl Palette {
    fn randomizer<T, U>(palette: &[Triplet<T>]) -> U
    where
        T: Copy + Clone,
        U: From<Triplet<T>>,
    {
        let index = random_range(0, palette.len());
        let color = palette[index];
        U::from(color)
    }

    pub const fn get(&self) -> &[Triplet] {
        match self {
            Self::Cm => &CM,
            Self::Cmyk => &CMYK,
            Self::Rusty => &RUSTY,
            Self::RYO => &RYO,
            Self::YOMONO => &YOMONO,
            Self::White => &WHITE_P,
            Self::Black => &BLACK_P,
            Self::Orange => &ORANGE_P,
            Self::NorthernLights => &NORTHERN_LIGHTS,
            Self::DarkRoadCurve => &DARK_ROAD_CURVE,
            Self::StarkContrast => &STARK_CONTRAST,
            Self::SkinTones => &SKIN_TONES,
            Self::Pastel => &PASTEL,
            Self::Fire => &FIRE,
            Self::BlackOrWhite => &BLACK_OR_WHITE,
            Self::PrettyHateMachine => &PHM,
        }
    }
}

impl PickRandom<Rgb<u8>> for Palette {
    fn pick_random(&self) -> Rgb<u8> {
        Self::randomizer(self.get())
    }
}

impl PickRandom<Rgba<f32>> for Palette {
    fn pick_random(&self) -> Rgba<f32> {
        let color: Rgb8 = Self::randomizer(self.get());
        to_rgba_f32(color.red, color.green, color.blue)
    }
}

impl PickRandom<Triplet> for Palette {
    fn pick_random(&self) -> Triplet {
        Self::randomizer(self.get())
    }
}

pub struct PaletteData<'a>(&'a [Triplet]);

impl<'a> PaletteData<'a> {
    pub fn get(&'a self, index: usize) -> Option<Triplet> {
        if index < self.0.len() {
            Some(self.0[index])
        } else {
            None
        }
    }

    pub fn size(&'a self) -> usize {
        self.0.len()
    }
}

/// A cache of a Palette type usually for a post-compute for
/// Rgb8 type so they don't get re-computed again.
pub struct PaletteCache<T = Rgba<f32>> {
    cache: Vec<T>,
}

impl PaletteCache<Rgba<f32>> {
    /// # Creates a PaletteCache from a slice of Triplet array
    ///
    /// ```
    /// use micrrou::palette::{PaletteCache,RYO};
    ///
    /// let palette = PaletteCache::from_triplets(&RYO);
    /// assert!(palette.count() == RYO.len());
    /// ```
    pub fn from_triplets(triplets: &[Triplet]) -> Self {
        let mut cache = Vec::new();
        for triplet in triplets.iter() {
            cache.push(to_rgba_f32(triplet.0, triplet.1, triplet.2))
        }

        Self { cache }
    }
}

impl PaletteCache {
    pub fn count(&self) -> usize {
        self.cache.len()
    }
}

impl<T> PaletteCache<T> {
    /// # Creates an Iter for the PaletteCache
    ///
    /// ```
    /// use nannou::color::Rgba;
    /// use micrrou::palette::{PaletteCache, Triplet};
    ///
    /// const PALETTE: [ Triplet; 2 ] = [
    ///     (1,2,3),
    ///     (255,254,253),
    /// ];
    /// let palette = PaletteCache::from_triplets(&PALETTE);
    /// let mut p = palette.iter();
    ///
    /// assert!(p.next() ==
    ///         Some(&Rgba::from((1.0/255.0, 2.0/255.0, 3.0/255.0, 1.0))));
    /// assert!(p.next() ==
    ///         Some(&Rgba::from((1.0, 254.0/255.0, 253.0/255.0, 1.0))));
    /// assert!(p.next() == None);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.cache.iter()
    }
}

impl<T> SetAlpha<T> for PaletteCache<Rgba<T>>
where
    T: Primitive + Component,
{
    /// # Sets the alpha channel for all the elements
    ///
    /// ```
    /// use micrrou::palette::{PaletteCache, NORTHERN_LIGHTS};
    /// use micrrou::palette::{PickRandom, SetAlpha};
    ///
    /// let mut palette = PaletteCache::from_triplets(&NORTHERN_LIGHTS);
    ///
    /// palette.set_alpha(0.51);
    /// assert!(palette.pick_random().alpha == 0.51);
    ///
    /// ```
    fn set_alpha(&mut self, alpha: T) {
        for color in self.cache.iter_mut() {
            color.alpha = alpha;
        }
    }
}

impl<T> PickRandom<T> for PaletteCache<T>
where
    T: Clone,
{
    fn pick_random(&self) -> T {
        let index = random_range(0, self.cache.len());
        self.cache[index].clone()
    }
}
