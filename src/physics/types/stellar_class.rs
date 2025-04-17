use crate::physics::randomization::random_stellar_class;
use rand::Rng;

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
}
