use crate::physics::objects::star::Star;
use crate::physics::types::stellar_class::StellarClass;
use bevy::prelude::{Event, Vec3};
use rand::Rng;

#[derive(Debug, Event)]
pub struct StarSpawnEvent {
    star: Star,
    id: Option<u64>,
}

impl StarSpawnEvent {
    pub fn new(star: Star, id: Option<u64>) -> Self {
        Self { star, id }
    }

    pub fn get_star(&self) -> &Star {
        &self.star
    }

    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn random(rng: &mut impl Rng, position: Vec3) -> Self {
        Self {
            star: Star::new_random(rng, position),
            id: None,
        }
    }

    pub fn with_mass(position: Vec3, mass: f64) -> Self {
        Self {
            star: Star::new(mass, position),
            id: None,
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
            id: None,
        }
    }
}
