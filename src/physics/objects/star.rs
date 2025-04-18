use crate::physics::calculation::stellar::*;
use crate::physics::randomization::stellar::random_stellar_mass;
use crate::physics::types::stellar_class::StellarClass;
use bevy::prelude::Color;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Star {
    stellar_class: StellarClass,
    mass: f64,
    radius: f64,
    luminosity: f64,
    surface_temperature: f64,
    color: Color,
}

impl Star {
    pub fn new(mass: f64) -> Self {
        let surface_temperature = stellar_surface_temperature_from_mass(mass);

        Self {
            stellar_class: StellarClass::from_stellar_mass(mass),
            mass,
            radius: stellar_radius_from_mass(mass),
            luminosity: stellar_luminosity_from_mass(mass),
            surface_temperature,
            color: stellar_color_from_surface_temperature(surface_temperature),
        }
    }

    pub fn new_random(rng: &mut impl Rng) -> Self {
        let stellar_class = StellarClass::random(rng);
        let mass = random_stellar_mass(rng, stellar_class);
        let radius = stellar_radius_from_mass(mass);
        let luminosity = stellar_luminosity_from_mass(mass);
        let surface_temperature = stellar_surface_temperature_from_mass(mass);
        let color = stellar_color_from_surface_temperature(surface_temperature);

        Self {
            stellar_class,
            mass,
            radius,
            luminosity,
            surface_temperature,
            color,
        }
    }

    pub fn get_stellar_class(&self) -> StellarClass {
        self.stellar_class
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_luminosity(&self) -> f64 {
        self.luminosity
    }

    pub fn get_surface_temperature(&self) -> f64 {
        self.surface_temperature
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}
