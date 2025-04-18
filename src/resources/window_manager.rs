use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct WindowManager {
    pub settings: bool,
    pub diagnostics: bool,
}
