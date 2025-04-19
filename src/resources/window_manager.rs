use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct WindowManager {
    pub bloom_settings: bool,
    pub diagnostics: bool,
    pub settings: bool,
}
