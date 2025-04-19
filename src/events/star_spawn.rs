use crate::physics::objects::star::Star;
use crate::physics::types::stellar_class::StellarClass;
use bevy::prelude::{Event, Vec3};
use rand::Rng;

#[derive(Debug, Event)]
pub struct StarSpawnEvent {
    star: Star,
}

impl StarSpawnEvent {
    pub fn get_star(&self) -> &Star {
        &self.star
    }

    pub fn random(rng: &mut impl Rng, position: Vec3) -> Self {
        Self {
            star: Star::new_random(rng, position),
        }
    }

    pub fn with_mass(position: Vec3, mass: f64) -> Self {
        Self {
            star: Star::new(mass, position),
        }
    }

    pub fn random_with_stellar_class(
        rng: &mut impl Rng,
        position: Vec3,
        stellar_class: StellarClass,
    ) -> Self {
        let mass = stellar_class.generate_random_mass(rng);
        Self {
            star: Star::new(mass, position),
        }
    }
}
