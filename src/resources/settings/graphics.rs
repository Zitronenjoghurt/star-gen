use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct GraphicsSettings {
    pub vsync: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self { vsync: true }
    }
}

impl GraphicsSettings {
    pub fn toggle_vsync(&mut self) {
        self.vsync = !self.vsync;
    }
}
