use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct ControlSettings {
    pub star_focus_auto_zoom_factor: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            star_focus_auto_zoom_factor: Self::DEFAULT_STAR_FOCUS_AUTO_ZOOM_FACTOR,
        }
    }
}

impl ControlSettings {
    pub const DEFAULT_STAR_FOCUS_AUTO_ZOOM_FACTOR: f32 = 7.0;

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
