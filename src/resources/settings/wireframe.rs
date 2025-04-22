use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Resource, PartialEq, Serialize, Deserialize)]
pub struct WireframeSettings {
    pub active: bool,
}

impl WireframeSettings {
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}
