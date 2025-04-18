use crate::physics::objects::star::Star;
use bevy::prelude::Component;

#[derive(Debug, Component, Clone)]
pub struct StarData(Star);

impl StarData {
    pub fn new(star: Star) -> Self {
        Self(star)
    }

    pub fn get_star(&self) -> &Star {
        &self.0
    }
}
