use crate::physics::randomization::stellar::{random_stellar_class, random_stellar_mass};
use rand::Rng;
use std::ops::Range;

pub const MIN_MASS_M: f64 = 0.08;
pub const MAX_MASS_M: f64 = 0.45;
pub const MIN_MASS_K: f64 = 0.45;
pub const MAX_MASS_K: f64 = 0.8;
pub const MIN_MASS_G: f64 = 0.8;
pub const MAX_MASS_G: f64 = 1.04;
pub const MIN_MASS_F: f64 = 1.04;
pub const MAX_MASS_F: f64 = 1.4;
pub const MIN_MASS_A: f64 = 1.4;
pub const MAX_MASS_A: f64 = 2.1;
pub const MIN_MASS_B: f64 = 2.1;
pub const MAX_MASS_B: f64 = 16.0;
pub const MIN_MASS_O: f64 = 16.0;
pub const MAX_MASS_O: f64 = 150.1;

pub const MASS_RANGE_M: Range<f64> = MIN_MASS_M..MAX_MASS_M;
pub const MASS_RANGE_K: Range<f64> = MIN_MASS_K..MAX_MASS_K;
pub const MASS_RANGE_G: Range<f64> = MIN_MASS_G..MAX_MASS_G;
pub const MASS_RANGE_F: Range<f64> = MIN_MASS_F..MAX_MASS_F;
pub const MASS_RANGE_A: Range<f64> = MIN_MASS_A..MAX_MASS_A;
pub const MASS_RANGE_B: Range<f64> = MIN_MASS_B..MAX_MASS_B;
pub const MASS_RANGE_O: Range<f64> = MIN_MASS_O..MAX_MASS_O;

#[derive(Debug, Clone, Copy)]
pub enum StellarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl StellarClass {
    pub fn random(rng: &mut impl Rng) -> Self {
        random_stellar_class(rng)
    }

    pub fn generate_random_mass(&self, rng: &mut impl Rng) -> f64 {
        random_stellar_mass(rng, self)
    }

    pub fn from_stellar_mass(mass: f64) -> Self {
        match mass {
            f64::NEG_INFINITY..MAX_MASS_M => Self::M,
            MIN_MASS_K..MAX_MASS_K => Self::K,
            MIN_MASS_G..MAX_MASS_G => Self::G,
            MIN_MASS_F..MAX_MASS_F => Self::F,
            MIN_MASS_A..MAX_MASS_A => Self::A,
            MIN_MASS_B..MAX_MASS_B => Self::B,
            MIN_MASS_O..f64::INFINITY => Self::O,
            _ => unreachable!(),
        }
    }
}
