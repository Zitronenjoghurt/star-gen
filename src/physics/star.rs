use crate::physics::randomization::random_stellar_mass;
use crate::physics::types::stellar_class::StellarClass;
use rand::Rng;

#[derive(Debug)]
pub struct Star {
    stellar_class: StellarClass,
    mass: f64,
}

impl Star {
    pub fn new_random(rng: &mut impl Rng) -> Self {
        let stellar_class = StellarClass::random(rng);
        let mass = random_stellar_mass(rng, stellar_class);
        Self {
            stellar_class,
            mass,
        }
    }
}
