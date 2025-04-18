use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct WireframeSettings {
    pub active: bool,
}

impl WireframeSettings {
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}

#[derive(Debug, Resource)]
pub struct BloomSettings {
    pub active: bool,
}

impl Default for BloomSettings {
    fn default() -> Self {
        Self { active: true }
    }
}

impl BloomSettings {
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}
