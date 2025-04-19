use crate::physics::objects::star::Star;
use crate::physics::types::stellar_class::StellarClass;
use bevy::prelude::{Event, Vec3};
use rand::Rng;

#[derive(Debug, Default, Event)]
pub struct StarSpawnEvent {
    position: Vec3,
    mass: Option<f64>,
    stellar_class: Option<StellarClass>,
}

impl StarSpawnEvent {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn with_mass(position: Vec3, mass: f64) -> Self {
        Self {
            position,
            mass: Some(mass),
            ..Default::default()
        }
    }

    pub fn with_stellar_class(position: Vec3, stellar_class: StellarClass) -> Self {
        Self {
            position,
            stellar_class: Some(stellar_class),
            ..Default::default()
        }
    }

    pub fn generate_star(&self, rng: &mut impl Rng) -> Star {
        let mass = if let Some(mass) = self.mass {
            mass
        } else if let Some(stellar_class) = self.stellar_class {
            stellar_class.generate_random_mass(rng)
        } else {
            let stellar_class = StellarClass::random(rng);
            stellar_class.generate_random_mass(rng)
        };
        Star::new(mass, self.position)
    }
}
