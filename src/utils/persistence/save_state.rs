use crate::utils::persistence::settings::SaveStateSettings;
use bevy::prelude::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SaveState {
    settings: SaveStateSettings,
}

impl SaveState {
    pub fn save(world: &World) -> Self {
        Self {
            settings: SaveStateSettings::save(world),
        }
    }

    pub fn load(&self, world: &mut World) {
        self.settings.load(world);
    }

    pub fn apply_validations(&mut self) {
        self.settings.apply_validations();
    }
}
