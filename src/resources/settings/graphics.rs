use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct GraphicsSettings {
    pub vsync: bool,
    pub render_distance: f32,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            render_distance: Self::DEFAULT_RENDER_DISTANCE,
        }
    }
}

impl GraphicsSettings {
    pub const DEFAULT_RENDER_DISTANCE: f32 = 10000.0;

    pub fn toggle_vsync(&mut self) {
        self.vsync = !self.vsync;
    }
}
