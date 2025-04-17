use crate::physics::types::stellar_class::StellarClass;
use rand::Rng;
use std::ops::Range;

const WEIGHT_CLASS_O: u32 = 3;
const WEIGHT_CLASS_B: u32 = 12_000;
const WEIGHT_CLASS_A: u32 = 61_000;
const WEIGHT_CLASS_F: u32 = 300_000;
const WEIGHT_CLASS_G: u32 = 760_000;
const WEIGHT_CLASS_K: u32 = 1_200_000;
const WEIGHT_CLASS_M: u32 = 7_600_000;

const MASS_CLASS_O: Range<f64> = 16.0..150.1;
const MASS_CLASS_B: Range<f64> = 2.1..16.0;
const MASS_CLASS_A: Range<f64> = 1.4..2.1;
const MASS_CLASS_F: Range<f64> = 1.04..1.4;
const MASS_CLASS_G: Range<f64> = 0.8..1.04;
const MASS_CLASS_K: Range<f64> = 0.45..0.8;
const MASS_CLASS_M: Range<f64> = 0.08..0.45;

const TOTAL_STELLAR_CLASS_WEIGHT: u32 = WEIGHT_CLASS_O
    + WEIGHT_CLASS_B
    + WEIGHT_CLASS_A
    + WEIGHT_CLASS_F
    + WEIGHT_CLASS_G
    + WEIGHT_CLASS_K
    + WEIGHT_CLASS_M;

const PROB_CUM_O: u32 = WEIGHT_CLASS_O;
const PROB_CUM_B: u32 = PROB_CUM_O + WEIGHT_CLASS_B;
const PROB_CUM_A: u32 = PROB_CUM_B + WEIGHT_CLASS_A;
const PROB_CUM_F: u32 = PROB_CUM_A + WEIGHT_CLASS_F;
const PROB_CUM_G: u32 = PROB_CUM_F + WEIGHT_CLASS_G;
const PROB_CUM_K: u32 = PROB_CUM_G + WEIGHT_CLASS_K;

pub fn random_stellar_class(rng: &mut impl Rng) -> StellarClass {
    let random_number = rng.random_range(0..TOTAL_STELLAR_CLASS_WEIGHT);
    match random_number {
        0..PROB_CUM_O => StellarClass::O,
        PROB_CUM_O..PROB_CUM_B => StellarClass::B,
        PROB_CUM_B..PROB_CUM_A => StellarClass::A,
        PROB_CUM_A..PROB_CUM_F => StellarClass::F,
        PROB_CUM_F..PROB_CUM_G => StellarClass::G,
        PROB_CUM_G..PROB_CUM_K => StellarClass::K,
        _ => StellarClass::M,
    }
}

pub fn random_stellar_mass(rng: &mut impl Rng, stellar_class: StellarClass) -> f64 {
    let range = match stellar_class {
        StellarClass::O => MASS_CLASS_O,
        StellarClass::B => MASS_CLASS_B,
        StellarClass::A => MASS_CLASS_A,
        StellarClass::F => MASS_CLASS_F,
        StellarClass::G => MASS_CLASS_G,
        StellarClass::K => MASS_CLASS_K,
        StellarClass::M => MASS_CLASS_M,
    };
    rng.random_range(range)
}
