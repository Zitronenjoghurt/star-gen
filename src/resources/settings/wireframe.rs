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
